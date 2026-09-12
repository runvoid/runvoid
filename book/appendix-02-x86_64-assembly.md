# Appendix B: x86_64 Assembly & Calling Conventions Reference

This appendix serves as a comprehensive hardware-level reference for developers inspecting Runvoid's emitted assembly (`runvoid emit-asm`) or authoring low-level inline assembly routines (`asm { ... }`).

---

## 1. The 16 General-Purpose 64-Bit Registers

The x86_64 CPU architecture provides sixteen 64-bit general-purpose registers:

| 64-Bit Name | 32-Bit Sub-Register | 16-Bit | 8-Bit Low | System V AMD64 Role (Linux/BSD/macOS) | Microsoft x64 Role (Windows) |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **`%rax`** | `%eax` | `%ax` | `%al` | Return Value / Accumulator | Return Value / Accumulator |
| **`%rbx`** | `%ebx` | `%bx` | `%bl` | Callee-Saved (Preserved by called fn) | Callee-Saved (Preserved by called fn) |
| **`%rcx`** | `%ecx` | `%cx` | `%cl` | 4th Function Argument / Shift Count | 1st Function Argument |
| **`%rdx`** | `%edx` | `%dx` | `%dl` | 3rd Function Argument / High Multiply | 2nd Function Argument |
| **`%rsi`** | `%esi` | `%si` | `%sil`| 2nd Function Argument / Source Index | Callee-Saved (Preserved by called fn) |
| **`%rdi`** | `%rdi` | `%di` | `%dil`| 1st Function Argument / Dest Index | Callee-Saved (Preserved by called fn) |
| **`%rbp`** | `%ebp` | `%bp` | `%bpl`| Frame / Base Pointer (Callee-Saved) | Frame / Base Pointer (Callee-Saved) |
| **`%rsp`** | `%esp` | `%sp` | `%spl`| Hardware Stack Pointer | Hardware Stack Pointer |
| **`%r8`**  | `%r8d` | `%r8w`| `%r8b`| 5th Function Argument | 3rd Function Argument |
| **`%r9`**  | `%r9d` | `%r9w`| `%r9b`| 6th Function Argument | 4th Function Argument |
| **`%r10`** | `%r10d`| `%r10w`|`%r10b`| Scratch / Caller-Saved (Temporary) | Scratch / Caller-Saved (Temporary) |
| **`%r11`** | `%r11d`| `%r11w`|`%r11b`| Scratch / Caller-Saved (Temporary) | Scratch / Caller-Saved (Temporary) |
| **`%r12`** | `%r12d`| `%r12w`|`%r12b`| Callee-Saved (Must be preserved) | Callee-Saved (Must be preserved) |
| **`%r13`** | `%r13d`| `%r13w`|`%r13b`| Callee-Saved (Must be preserved) | Callee-Saved (Must be preserved) |
| **`%r14`** | `%r14d`| `%r14w`|`%r14b`| Callee-Saved (Must be preserved) | Callee-Saved (Must be preserved) |
| **`%r15`** | `%r15d`| `%r15w`|`%r15b`| Callee-Saved (Must be preserved) | Callee-Saved (Must be preserved) |

---

## 2. Floating-Point & SIMD Vector Registers (`%xmm0` - `%xmm15`)

For IEEE-754 32-bit (Single) and 64-bit (Double) floating-point calculations, x86_64 uses SSE2 vector registers:

```
+-------------------------------------------------------------------------+
| %xmm0  .. %xmm7  : Floating-Point Arguments & Return Values (%xmm0)      |
| %xmm8  .. %xmm15 : Scratch Vector Registers (Caller-Saved)              |
+-------------------------------------------------------------------------+
```

Under System V AMD64:
- Float arguments 1 through 8 are passed in `%xmm0` through `%xmm7`.
- Float return values are returned in `%xmm0`.

---

## 3. ABI Comparison: Linux (System V) vs. Windows (MS x64)

Operating systems enforce strict Application Binary Interfaces (ABIs). Failing to honor ABI invariants results in immediate memory corruption or `STATUS_ACCESS_VIOLATION`:

```
Linux / BSD / macOS (System V AMD64):
+-------------------------------------------------------------------------+
| Arg 1: %rdi                                                             |
| Arg 2: %rsi                                                             |
| Arg 3: %rdx                                                             |
| Arg 4: %rcx                                                             |
| Arg 5: %r8                                                              |
| Arg 6: %r9                                                              |
| Extra: Pushed to Stack (Right-to-Left)                                  |
| Shadow Space: None required!                                            |
| Red Zone: 128 bytes below %rsp is protected from signal interruption    |
+-------------------------------------------------------------------------+

Microsoft Windows (x64 ABI):
+-------------------------------------------------------------------------+
| Arg 1: %rcx                                                             |
| Arg 2: %rdx                                                             |
| Arg 3: %r8                                                              |
| Arg 4: %r9                                                              |
| Extra: Pushed to Stack (Right-to-Left)                                  |
| Shadow Space: CALLER MUST ALLOCATE 32 BYTES ON STACK (sub rsp, 32)!     |
| Red Zone: None! Any data below %rsp can be corrupted by interrupts!     |
+-------------------------------------------------------------------------+
```

### The 16-Byte Stack Alignment Invariant

Both System V AMD64 and Microsoft x64 mandate that before executing a `call` instruction:

$$(\%rsp) \pmod{16} == 0$$

When the `call` instruction executes, the CPU pushes the 8-byte return instruction pointer (`%rip`) onto the stack. Therefore, upon entry to the called function (`fn_entry:`):

$$(\%rsp) \pmod{16} == 8$$

To re-establish 16-byte alignment inside a standard function prologue:
```nasm
my_function:
    push rbp            ; rsp decrements by 8 -> now rsp % 16 == 0
    mov rbp, rsp        ; establish new frame pointer
    sub rsp, 32         ; allocate 32 bytes of local storage (multiple of 16)
    
    ; ... function body ...
    
    mov rsp, rbp        ; restore stack pointer
    pop rbp             ; restore old base pointer
    ret                 ; pops return address, returning to caller
```

---

## 4. The 128-Byte System V "Red Zone"

The System V AMD64 specification defines a 128-byte region beyond the current stack pointer as the **Red Zone**:

```
High Memory
   |                             |
   | [ Active Stack Frame ]      |
   |                             |
   +-----------------------------+ <--- %rsp
   |                             |
   |   128-Byte Red Zone Area    |  <-- Scratch space: leaf functions can store
   |   (%rsp - 1 to %rsp - 128)  |      local variables here WITHOUT sub rsp!
   |                             |
   +-----------------------------+
   | Free Unallocated RAM        |
   v
Low Memory
```

Leaf functions (functions that do not call any other functions) can utilize this 128-byte zone for temporary variables without modifying `%rsp`, saving two CPU clock cycles per invocation!

---

## 5. Essential Assembly Instructions Reference

### 1. Data Movement
- `mov dst, src`: Copies quadword/doubleword from source to destination.
- `movzx dst, src`: Zero-extends a smaller register (e.g. 8-bit `%al`) into a 64-bit register.
- `movsx dst, src`: Sign-extends a signed integer from smaller to larger register.
- `lea dst, [base + index*scale + disp]`: Computes address without memory access (useful for fast math like `lea rax, [rdi + rdi*4]` to multiply by 5).

### 2. Integer Arithmetic
- `add dst, src`: `dst = dst + src` (sets `ZF`, `SF`, `CF`, `OF`).
- `sub dst, src`: `dst = dst - src`.
- `imul dst, src`: Signed integer multiplication.
- `cqo`: Sign-extends `%rax` across `%rdx:%rax` before 64-bit division.
- `idiv divisor`: Divides `%rdx:%rax` by `divisor`. Quotient in `%rax`, modulo in `%rdx`.
- `inc reg` / `dec reg`: Increments or decrements register by 1.

### 3. Bitwise & Logic
- `and dst, src`: Bitwise conjunction (`dst = dst & src`).
- `or dst, src`: Bitwise disjunction (`dst = dst | src`).
- `xor dst, src`: Bitwise exclusive-or (`dst = dst ^ src`). Clearing a register is idiomatically written `xor eax, eax` (encodes into 2 bytes vs 5 bytes for `mov eax, 0`).
- `not dst`: Bitwise inversion (one's complement).
- `shl dst, cl` / `shr dst, cl`: Logical shift left / shift right by count in `%cl`.
- `sar dst, cl`: Arithmetic shift right (preserves sign bit for negative numbers).

### 4. Branching & Comparison
- `cmp a, b`: Performs `a - b` and sets CPU flags without storing result.
- `test a, b`: Performs `a & b` and sets `ZF`/`SF` (e.g. `test rax, rax` checks if `%rax` is zero).
- `je / jz label`: Jump if equal / zero (`ZF = 1`).
- `jne / jnz label`: Jump if not equal / non-zero (`ZF = 0`).
- `jg label` / `jge label`: Signed jump if greater / greater-or-equal.
- `jl label` / `jle label`: Signed jump if less / less-or-equal.
- `ja label` / `jb label`: Unsigned jump if above / below.
- `jmp label`: Unconditional jump.
- `call label`: Push next `%rip` onto stack and jump to label.
- `ret`: Pop return address into `%rip` and resume execution.
- `syscall`: Transitions into kernel mode to execute POSIX OS system call (`%rax` = system call number).
