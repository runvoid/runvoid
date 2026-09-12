# 8.8 Bare-Metal Freestanding Binaries

In systems software engineering, there is a fundamental distinction between two execution environments:

1. **Hosted Environment:** The application runs on top of a fully booted operating system (Linux or Windows). The OS loader allocates virtual memory, loads dynamic shared libraries (`libc.so`, `ntdll.dll`), initializes thread-local storage, and calls `main`.
2. **Freestanding Environment:** The software executes on raw physical hardware or within an early boot environment where **no operating system exists**. There is no `libc`, no heap allocator, no standard I/O streams, and no dynamic linker.

Runvoid provides first-class support for freestanding compilation via `add Freestanding`.

---

## 1. Directives for Freestanding Mode

To compile a pure, dependency-free freestanding binary, configure the top-level directives:

```runvoid
remove garbageC
remove Basic
remove Linux
add Advanced
add Freestanding

// Entry point: The compiler emits `global _start`
asm {
    ; Direct Linux sys_write (syscall 1):
    ; stdout = fd 1, message string, length 14
    mov rax, 1
    mov rdi, 1
    lea rsi, [rel msg]
    mov rdx, 14
    syscall

    ; Direct Linux sys_exit (syscall 60):
    mov rax, 60
    xor rdi, rdi
    syscall

msg:
    db "Raw Metal OK!", 10
}
```

---

## 2. Compilation and Linkage

When `add Freestanding` is declared, the Runvoid compiler radically reconfigures its code generator and linker flags:

1. **Entry Point Substitution:** Rather than generating a standard `main` symbol that expects C runtime initialization (`__libc_start_main`), the compiler emits the raw ELF entry symbol: `global _start`.
2. **Elimination of CRT Objects:** The linker completely bypasses C runtime initialization objects (`crt1.o`, `crti.o`, `crtbegin.o`, `crtend.o`, `crtn.o`).
3. **Pure Static Linkage:** The linker invocation switches from `gcc` to direct GNU `ld`:
   ```bash
   ld -m elf_x86_64 -nostdlib -static -s entry.o -o kernel.bin
   ```

The resulting executable contains **zero dynamic dependencies** (`ldd binary` reports `not a dynamic executable`) and can be as small as **400 bytes**!

---

## 3. Direct Linux Syscall Reference (x86_64)

In a freestanding Linux binary, you invoke operating system services directly through the CPU's `syscall` instruction:

| Syscall Name | `%rax` Number | `%rdi` (Arg 1) | `%rsi` (Arg 2) | `%rdx` (Arg 3) |
| :--- | :--- | :--- | :--- | :--- |
| **`sys_read`** | `0` | File Descriptor (`fd`) | Buffer Pointer (`char*`) | Byte Count (`size_t`) |
| **`sys_write`** | `1` | File Descriptor (`fd`) | Buffer Pointer (`char*`) | Byte Count (`size_t`) |
| **`sys_open`** | `2` | Filename Pointer | Open Flags (`O_RDONLY`, etc.) | Mode / Permissions |
| **`sys_close`** | `3` | File Descriptor (`fd`) | - | - |
| **`sys_mmap`** | `9` | Preferred Address | Allocation Length | Memory Protection |
| **`sys_exit`** | `60` | Exit Status Code | - | - |

---

## 4. Writing an x86_64 Bare-Metal OS Kernel

Beyond raw Linux binaries, Runvoid can be used to write standalone OS kernels running directly under QEMU or physical hardware.

### 1. The Multiboot 1 / 2 Header Standard
For GRUB or modern bootloaders to recognize your binary as a bootable kernel, an ELF binary must contain a 12-byte aligned header within the first 8 KB of the file:
- **Magic:** `0x1BADB002`
- **Flags:** `0x00000003` (Align modules on 4KB boundaries, provide memory map)
- **Checksum:** `-(0x1BADB002 + 0x00000003)` ($= 0xE4524FFB$)

### 2. The VGA 80x25 Text-Mode Buffer (`0xB8000`)
In protected 32-bit or long 64-bit mode, the PC hardware maps text-mode video memory directly to physical address `0xB8000`. Video memory consists of 2,000 two-byte words ($80 \times 25$ characters):

```
+-------------------+-------------------+
| Attribute (1 Byte)| ASCII Char (1 Byte)|
+-------------------+-------------------+
Attribute Byte Layout:
Bit 7: Blink / Bright Background
Bits 6-4: Background Color (0 = Black, 1 = Blue, 4 = Red)
Bits 3-0: Foreground Color (2 = Green, 14 = Yellow, 15 = White)
```

```runvoid
remove garbageC
remove Basic
remove Linux
add Advanced
add Freestanding

action kernel_main(): Void {
    // Pointer to VGA video memory:
    remember vga_buffer: Ptr = 0xB8000

    // Color: White text (0x0F) on Blue background (0x10) -> 0x1F
    remember attr = 0x1F

    // Write "STARVOID OS" banner
    remember title = "STARVOID OS v1.3 - BARE METAL KERNEL"
    remember idx = 0
    while idx < title.length {
        remember char_code = char_at(title, idx)
        remember cell_offset = idx * 2
        @(vga_buffer + cell_offset) = (attr bit shift left 8) bit or char_code
        idx = idx + 1
    }

    // Halt the CPU indefinitely:
    asm {
    .halt_loop:
        cli
        hlt
        jmp .halt_loop
    }
}
```

---

## 5. Serial Port COM1 (UART) Output for Headless Kernels

When booting in virtualized environments or headless cloud servers without a VGA display, kernels output telemetry through the **Serial COM1 Port (`0x3F8`)**:

```runvoid
remove garbageC
remove Basic
remove Linux
add Advanced
add Freestanding

action serial_init(): Void {
    asm {
        mov dx, 0x3F9       ; Disable interrupts
        xor al, al
        out dx, al

        mov dx, 0x3FB       ; Enable DLAB (set baud rate divisor)
        mov al, 0x80
        out dx, al

        mov dx, 0x3F8       ; Divisor low byte (38400 baud)
        mov al, 0x03
        out dx, al

        mov dx, 0x3F9       ; Divisor high byte
        xor al, al
        out dx, al

        mov dx, 0x3FB       ; 8 bits, no parity, one stop bit (8N1)
        mov al, 0x03
        out dx, al
    }
}

action serial_write_byte(b: Int): Void {
    asm {
        ; Wait until transmitter holding register is empty (port 0x3FD bit 5)
    .wait_tx:
        mov dx, 0x3FD
        in al, dx
        test al, 0x20
        jz .wait_tx

        ; Send byte to COM1 (port 0x3F8)
        mov dx, 0x3F8
        mov rax, [rbp - 8]
        out dx, al
    }
}
```

---

## 6. Creating a Bootable ISO & Testing in QEMU

To turn your compiled kernel into a bootable ISO:

```bash
# 1. Compile freestanding kernel with Runvoid:
runvoid build --target linux src/kernel.rv -o isodir/boot/kernel.elf

# 2. Configure GRUB menu (isodir/boot/grub/grub.cfg):
cat << 'EOF' > isodir/boot/grub/grub.cfg
menuentry "StarVoid OS" {
    multiboot /boot/kernel.elf
    boot
}
EOF

# 3. Create bootable hybrid ISO image:
grub-mkrescue -o starvoid.iso isodir

# 4. Boot directly in QEMU emulator:
qemu-system-x86_64 -cdrom starvoid.iso -serial stdio
```

QEMU initializes the virtual CPU, sets up 64-bit Long Mode, and jumps directly into your Runvoid `_start` entry point!

This demonstrates the ultimate reach of Runvoid: from high-level conversational scripts down to writing your own operating system from scratch!
