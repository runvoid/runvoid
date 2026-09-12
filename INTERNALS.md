# Runvoid Compiler Internals & Architecture

This document serves as the comprehensive technical reference for the internal architecture, memory models, calling conventions, and runtime mechanisms of the Runvoid programming language compiler.

---

## 1. High-Level Architecture Pipeline

The Runvoid compiler (`runvoid`) is implemented in Rust. It transforms human-readable Runvoid source code (`.rv`) directly into native machine executables via x86_64 NASM assembly and target platform linkers.

```
+-------------+      +-------------+      +-------------------+
| Source Code | ---> | Lexer       | ---> | Recursive Descent |
| (.rv files) |      | (Tokens)    |      | Parser (AST)      |
+-------------+      +-------------+      +-------------------+
                                                    |
                                                    v
+-------------------+      +-------------+      +-------------------+
| Target Assembler  | <--- | x86_64 NASM | <--- | Semantic Checker  |
| (NASM + Linker)   |      | Codegen     |      | & AST Optimizer   |
+-------------------+      +-------------+      +-------------------+
          |
          +---> Linux: ELF-64 binary via `gcc`
          +---> Windows: PE32+ `.exe` via `x86_64-w64-mingw32-gcc`
```

### Compilation Stages

1. **Lexical Analysis (`src/lexer.rs`)**:
   - Scans UTF-8 source into a stream of strongly-typed `Token` objects.
   - Handles template strings with embedded interpolations: `"Value is {x + 1}"`.
   - Strips single-line (`//`, `#`) and semi-colon (`;`) comments.

2. **Recursive Descent Parser (`src/parser.rs`)**:
   - Parses tokens into an Abstract Syntax Tree (AST) rooted at `Program`.
   - Processes modular imports (`use "path/file.rv"`), recursively reading and deduplicating dependencies into a single AST.

3. **Semantic Validator & Typechecker (`src/typechecker.rs`)**:
   - Enforces architectural directives:
     - `remove Basic`: Demands explicit static type annotations (`Int`, `String`, `Bool`, `Ptr`, struct names).
     - `remove GC`: Prohibits runtime-allocated heap strings unless manual management is invoked.
     - `remove All`: Strips all runtime layers for freestanding execution.
   - Validates struct definitions, field offsets, and function signatures.
   - Validates foreign function interface (`extern "C"`) declarations.

4. **AST Optimizer (`src/optimizer.rs`)**:
   - **Constant Folding**: Evaluates compile-time arithmetic (e.g., `4 * 1024` -> `4096`).
   - **Dead Code Elimination (DCE)**: Prunes statements following unconditional `give` (return) statements or inside unreachable `if false` blocks.

5. **Assembly Generator (`src/codegen.rs`)**:
   - Generates x86_64 NASM assembly code conforming to target OS ABI requirements.
   - Manages stack frames, local variable offsets, and 16-byte stack alignment.

6. **Native Linking (`src/compiler.rs`)**:
   - Invokes `nasm -f elf64` (Linux) or `nasm -f win64` (Windows).
   - Compiles native runtime (`runtime/gui.c`).
   - Invokes `gcc` or `x86_64-w64-mingw32-gcc` to produce the final executable.

---

## 2. ABI & Calling Conventions

Runvoid supports cross-compilation between Linux and Windows x86_64. Each target adheres strictly to its platform ABI.

### System V AMD64 ABI (Linux)

Under Linux, function arguments are passed in 64-bit general-purpose registers:
1. First argument: `RDI`
2. Second argument: `RSI`
3. Third argument: `RDX`
4. Fourth argument: `RCX`
5. Fifth argument: `R8`
6. Sixth argument: `R9`
7. Subsequent arguments: Pushed onto the stack in reverse order.
8. Return value: `RAX` (integer / pointer) or `RDX:RAX` (128-bit).

### Microsoft x64 Calling Convention (Windows)

Under Windows (`--target x86_64-windows`):
1. First argument: `RCX`
2. Second argument: `RDX`
3. Third argument: `R8`
4. Fourth argument: `R9`
5. **Shadow Space**: The caller must allocate at least 32 bytes (4 quadwords) of shadow store on the stack before issuing the `call` instruction.
6. Return value: `RAX`.

### 16-Byte Stack Alignment Invariant

Both System V AMD64 and Microsoft x64 ABIs strictly mandate that the stack pointer `RSP` must be 16-byte aligned before any `call` instruction.
The Runvoid code generator tracks all stack pushes and local frame allocations, inserting dynamic padding (`sub rsp, 8` when odd) to guarantee 16-byte alignment and prevent hardware SSE/AVX alignment faults.

---

## 3. Memory Model & Garbage Collection

Runvoid features a dual-mode memory model:

### Standard Mode: Mark-and-Sweep Garbage Collector

In standard mode, dynamic entities (strings, lists, maps, closures) are tracked by a lightweight mark-and-sweep collector defined in `runtime/gui.c`:
- **Allocation**: Memory blocks are prepended with an internal header containing mark bits, size, and type descriptor.
- **Root Set**: Active stack frames and global variables registered via `rv_gc_register_root()`.
- **Mark Phase**: Recursively traverses roots, marking reachable allocation headers.
- **Sweep Phase**: Iterates over the global heap linked list, freeing unreferenced blocks via `free()` and returning memory to the system allocator.

### Pro Mode: Zero-Overhead Manual Memory

When `remove GC` is active, the garbage collector runtime is omitted:
- Direct raw pointer primitives: `allocate(size)` -> `malloc()`, `free(ptr)` -> `free()`.
- Pointer arithmetic and dereferencing: `*ptr` and `*(ptr + offset)`.
- Zero runtime overhead and predictable real-time execution.

---

## 4. Freestanding Bare-Metal Execution

When `remove All` is specified:
- All runtime dependencies (`libc`, GUI runtime, GC) are disabled.
- The compiler emits a naked `_start` entry point instead of standard `main`.
- Low-level interaction is performed via raw Linux syscalls (`syscall` instruction) or direct MMIO (e.g. writing directly to VGA video memory `0xB8000`).

Example Freestanding Minimal Binary:
```runvoid
remove All

action _start() {
    // Syscall 1 (sys_write) to stdout (fd 1)
    asm "mov rax, 1"
    asm "mov rdi, 1"
    asm "lea rsi, [rel msg]"
    asm "mov rdx, 14"
    asm "syscall"

    // Syscall 60 (sys_exit) with code 0
    asm "mov rax, 60"
    asm "xor rdi, rdi"
    asm "syscall"
}
```

---

## 5. Concurrency & Synchronization Primitives

Runvoid Pro mode provides multi-threading primitives directly backed by native OS threading:
- `thread action_name()`: Dispatches an action onto an OS-level worker thread.
- `atomic`: Translates into hardware-locked x86_64 instructions (`lock xadd`, `lock cmpxchg`, `mfence`) ensuring race-free state across threads without requiring mutex locks.

---

## 6. Project Manifest & Module System

The Runvoid build engine parses `runvoid.toml` manifests:
- Standard project structure:
  - `runvoid.toml`: Package name, target, and compiler optimization flags.
  - `src/main.rv`: Main application entry point.
  - `tests/`: Automated unit and integration test suites.
  - `bin/`: Target output binary folder.
- Run commands:
  - `runvoid run`: Automatically builds and executes project root.
  - `runvoid build`: Emits standalone executable into `bin/`.
  - `runvoid clean`: Wipes build artifacts.
