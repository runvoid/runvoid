# 8. Pro Systems Mode

When your project demands deterministic performance, microsecond execution guarantees, raw hardware access, and direct interaction with the operating system kernel, Runvoid transforms into an uncompromising systems programming language.

In high-level scripting mode, Runvoid prioritizes conversational simplicity, dynamic collections, and automated garbage collection. But under the surface, Runvoid was engineered from day one on top of native x86_64 machine code. 

**Pro Systems Mode** gives you the ability to selectively disable high-level runtime layers and take manual command of the underlying hardware:
- Enforce strict, compile-time static typing with zero runtime overhead.
- Strip the garbage collection subsystem entirely for zero-pause determinism.
- Link modular, granular modules from the standard library without dragging in unused symbols.
- Allocate, inspect, and manipulate raw memory addresses via hardware pointers.
- Define C-compatible POD (Plain Old Data) structs and bind directly to C shared libraries (`.so` / `.dll`).
- Coordinate concurrent CPU cores with OS threads and atomic hardware primitives.
- Inject raw inline NASM assembly and profile CPU execution using the `rdtsc` cycle counter.
- Compile freestanding, bare-metal ELF binaries with no C runtime or standard library dependencies.

---

## Chapter Outline

This chapter guides you step-by-step through every facet of systems development in Runvoid:

* **8.1 Systems Philosophy & Directives:** How top-level compiler directives (`remove garbageC`, `remove Basic`, `add Advanced`, `add Freestanding`) reshape the compiler pipeline.
* **8.2 Strict Static Typing:** Eliminating type ambiguity with explicit type signatures (`: Int`, `: String`, `: Bool`, `: Ptr`).
* **8.3 The Modular Standard Library:** Zero-overhead imports (`use ior`, `math`, `sys`, `mem`, `fs`, `net`, `thread`) and dead-code elimination.
* **8.4 Raw Pointers & Manual Memory:** Address-of operators, memory dereferencing, manual heap allocation, and memory safety hygiene.
* **8.5 C-Compatible Structs & C FFI:** Struct memory alignment, field offsets, and linking with external C/C++ libraries.
* **8.6 Multithreading & Hardware Atomics:** Spawning native OS threads, mutex synchronization, and hardware lock-prefixed atomic instructions.
* **8.7 Inline Assembly & CPU Cycle Profiling:** Injecting hand-crafted NASM instructions and benchmarking performance with nanosecond cycle precision.
* **8.8 Bare-Metal Freestanding Binaries:** Building operating system kernels, bootloaders, and firmware with custom `_start` entry points and zero runtime baggage.

Welcome to the metal.

---

## Systems Philosophy: The Zero-Cost Abstraction Principle

In software systems engineering, Bjarne Stroustrup famously coined the Zero-Cost Abstraction rule:
1. What you don't use, you don't pay for.
2. What you do use, you couldn't hand-code any better yourself.

Runvoid Pro Systems Mode implements this principle to the letter:

```
+-------------------------------------------------------------------------------+
| Runvoid Beginner Mode                                                         |
| [ Garbage Collector (GC) ] [ Dynamic Type Inference ] [ Batteries Included ]  |
+-------------------------------------------------------------------------------+
                                       |
                                       | `remove garbageC` (Drop GC overhead)
                                       | `remove Basic`    (Demand strict types)
                                       | `add Advanced`    (Unlock systems tools)
                                       v
+-------------------------------------------------------------------------------+
| Runvoid Pro Systems Mode                                                      |
| [ Zero-Pause Execution ] [ Static Types ] [ Raw Pointers ] [ Inline Assembly ]|
+-------------------------------------------------------------------------------+
                                       |
                                       | `remove Linux`
                                       | `add Freestanding`
                                       v
+-------------------------------------------------------------------------------+
| Runvoid Bare-Metal Kernel Mode                                                |
| [ No libc ] [ Entry point: _start ] [ Direct VGA 0xB8000 ] [ Raw Syscalls ]   |
+-------------------------------------------------------------------------------+
```

---

## Systems Configuration Matrix

| Directive | Impact on Pipeline | Binary Size Impact | Target Use Case |
| :--- | :--- | :--- | :--- |
| **Default** | Mark-and-sweep GC active, dynamic types allowed | ~35 KB – 80 KB | CLI scripts, desktop GUIs, web utilities |
| **`remove garbageC`** | Eliminates allocation headers & GC sweep passes | -15 KB | Real-time audio, games, latency-critical apps |
| **`remove Basic`** | Mandates explicit static type signatures on variables | 0 KB | Large team codebases, rigorous refactoring |
| **`add Advanced`** | Enables `struct`, raw pointers, inline `asm`, atomics | Modular | Systems software, network servers, C FFI |
| **`add Freestanding`** | Strips `libc`, emits naked `_start`, links with `ld -s` | Micro (< 4 KB) | OS kernels, bootloaders, microcontrollers |


