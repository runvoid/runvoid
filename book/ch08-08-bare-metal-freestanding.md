# 8.8 Bare-Metal Freestanding Binaries

In systems software engineering, there is a fundamental distinction between two execution environments:

1. **Hosted Environment:** The application runs on top of a fully booted operating system (Linux or Windows). The OS loader allocates virtual memory, loads dynamic shared libraries (`libc.so`, `ntdll.dll`), initializes thread-local storage, and calls `main`.
2. **Freestanding Environment:** The software executes on the raw physical hardware or within an early boot environment where **no operating system exists**. There is no `libc`, no heap allocator, no standard I/O streams, and no dynamic linker.

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

In protected 32-bit or long 64-bit mode, the PC hardware maps text-mode video memory directly to physical address `0xB8000`. You can write text directly to the screen by storing character-attribute byte pairs into that address:

```runvoid
remove garbageC
remove Basic
remove Linux
add Advanced
add Freestanding

action kernel_main(): Void {
    // Pointer to VGA video memory:
    remember vga_buffer: Ptr = 0xB8000

    // Write 'R' with green-on-black attribute (0x02):
    @(vga_buffer + 0) = 0x0252  // 0x52 = 'R', 0x02 = Green
    @(vga_buffer + 2) = 0x0255  // 'U'
    @(vga_buffer + 4) = 0x024E  // 'N'
    @(vga_buffer + 6) = 0x0256  // 'V'
    @(vga_buffer + 8) = 0x024F  // 'O'

    // Halt the CPU indefinitely:
    asm {
    .halt_loop:
        cli
        hlt
        jmp .halt_loop
    }
}
```

This demonstrates the ultimate reach of Runvoid: from high-level conversational scripts down to writing your own operating system from scratch!

