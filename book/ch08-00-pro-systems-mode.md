# 8. Pro Systems Mode

When your project outgrows high-level conveniences and demands direct hardware manipulation, strict static typing, and microsecond-level determinism, Runvoid transforms into an uncompromising systems programming language.

In this chapter, we explore the capabilities unlocked by **Pro Systems Mode**:
- Compiler directives and activation
- Strict static typing (`: Int`, `: String`, `: Bool`, `: Ptr`)
- The modular standard library (`use ior`, `math`, `sys`, `mem`, `fs`, `net`, `thread`)
- Raw pointers and manual heap management (`addr`, `@`, `alloc`, `free`)
- C-compatible POD structs and native C FFI (`extern "C"`)
- Native POSIX multithreading and hardware atomics (`atomic add`)
- Hardware inline assembly and CPU clock cycle profiling (`measure cycles`)
- Bare-metal freestanding binaries without libc (`add Freestanding`)
