# 8.8 Bare-Metal Freestanding Binaries

For operating system kernels, bootloaders, micro-containers, or embedded targets where **neither libc nor any runtime library is permissible**, Runvoid provides Freestanding Mode.

## Directives for Freestanding Mode

Combine `remove Linux` and `add Freestanding`:

```runvoid
remove garbageC
remove Basic
remove Linux
add Advanced
add Freestanding

# Entry point 'global _start' is emitted automatically by the compiler.
# Linux sys_exit syscall: rax = 60, rdi = status code
asm {
    mov rax, 60
    xor rdi, rdi
    syscall
}
```

## How It Compiles

- The compiler emits `global _start` instead of `main`.
- All references to C runtime initialization (`rv_init`), memory allocators, and standard libraries are omitted.
- The binary is linked directly with GNU `ld -s` (not GCC), producing an ultra-lean standalone ELF executable containing pure machine instructions and zero external dependencies.
