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

