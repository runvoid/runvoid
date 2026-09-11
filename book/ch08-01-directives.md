# 8.1 Systems Philosophy & Directives

Activating Pro Systems Mode requires explicitly stating your architectural intentions to the compiler. Unlike traditional compilers that rely on arcane command-line flags (`-fno-builtin`, `-nostdlib`, `-fPIC`), Runvoid places architectural intent directly in the source code using **top-level compiler directives**.

---

## The Core Systems Directives

To transition a Runvoid file from conversational scripting into hardened systems mode, place directives at the very top of your file:

```runvoid
remove garbageC
remove Basic
add Advanced
```

Let’s examine what each directive instructs the compiler to do:

### 1. `remove garbageC`
Instructs the compiler and linker to eliminate the entire Garbage Collection subsystem:
- Memory allocations bypass the GC tracking pool and use direct OS allocator calls (`mmap` / `HeapAlloc`).
- The runtime eliminates stack scanning and mark-and-sweep passes.
- Shrinks the binary size by 12–18 KB and guarantees **zero garbage-collection pauses**.

### 2. `remove Basic`
Disables permissive, dynamic type inference and implicit coercions:
- Every variable declaration with `remember` must supply an explicit type annotation (e.g. `remember x: Int = 42`).
- Every action must declare parameter types and a return type (e.g. `action compute(n: Int) -> Int`).
- The compiler emits strict compile-time errors on type mismatches instead of attempting dynamic runtime conversions.

### 3. `add Advanced`
Unlocks low-level hardware and systems programming primitives:
- Grants access to raw memory pointers (`addr`, `@`), pointer arithmetic, and manual allocation (`alloc`, `free`).
- Enables inline NASM assembly blocks (`inline_asm`).
- Enables C FFI (`extern "C"`) and direct OS system calls.
- Activates advanced compiler optimization passes: Dead Code Elimination (DCE), Constant Folding, Peephole Optimization, and Link-Time Optimization (LTO).

### 4. `add Freestanding`
Enables bare-metal freestanding compilation:
- Strips the C standard library (`libc` / `msvcrt`) completely.
- Bypasses the standard runtime entry point (`main`) and transfers control directly to a custom `_start` symbol.
- Enables compilation of bootloaders, kernel modules, embedded firmware, and hypervisors.

---

## Directive Combinations & Architectural Archetypes

Runvoid’s modular directive design allows you to craft the exact runtime environment your application demands:

```
+---------------------------+-----------------------------------------------+
| Directive Pattern         | Best Suited For                               |
+---------------------------+-----------------------------------------------+
| (No directives)           | Scripts, rapid CLI prototypes, 2D GUI games   |
| remove garbageC           | Game loops, real-time audio DSP, low latency  |
| remove garbageC           | Systems utilities, network daemons, high-perf |
| remove Basic              | C libraries, embedded CLI tools               |
| add Advanced              |                                               |
| add Freestanding          | OS kernels, bootloaders, microcontrollers     |
+---------------------------+-----------------------------------------------+
```

---

## How Directives Modify the Compiler Pipeline

During compilation, directives are processed during the very first AST traversal stage:

1. **AST Root Verification:** Directives must appear before any executable statements or action declarations.
2. **Typechecker Hardening:** If `remove Basic` is detected, the typechecker enforces strict static type validation on every node.
3. **Codegen Configuration:**
   - If `remove garbageC` is active, GC hook injections around pointer allocations are suppressed.
   - If `add Advanced` is active, NASM code generation allows direct register assignments and memory dereferences.
   - If `add Freestanding` is active, the linker flags are altered to `-nostdlib -static` (on Linux) or `/NODEFAULTLIB` (on Windows).

