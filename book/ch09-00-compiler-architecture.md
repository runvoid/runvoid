# 9. Compiler Architecture & Ecosystem

The Runvoid compiler is engineered from the ground up in **Rust** to provide memory-safe, deterministic, and lightning-fast compilation. It targets **x86_64 machine architecture** by generating human-readable **NASM assembly code**, which is assembled and linked into standalone native executables for **Linux** and **Windows**.

Understanding the compiler's internal pipeline gives you deep insight into how your conversational code transforms into raw silicon instructions.

---

## 1. High-Level Compiler Pipeline

```
  +-------------------------------------------------------------+
  | Source Code (.rv)                                           |
  +-------------------------------------------------------------+
                                 |
                                 v
  +-------------------------------------------------------------+
  | Lexical Analysis (src/lexer.rs)                             |
  | Tokenizes text, handles comments & string interpolations    |
  +-------------------------------------------------------------+
                                 |
                                 v
  +-------------------------------------------------------------+
  | Recursive Descent Parser (src/parser.rs)                    |
  | Builds AST, resolves recursive multi-file imports (`use`)   |
  +-------------------------------------------------------------+
                                 |
                                 v
  +-------------------------------------------------------------+
  | Typechecker & Semantic Validator (src/typechecker.rs)       |
  | Checks directive rules, validates strict types & structs    |
  +-------------------------------------------------------------+
                                 |
                                 v
  +-------------------------------------------------------------+
  | AST Optimizer (src/optimizer.rs)                            |
  | Constant folding, algebraic reduction, dead-code removal    |
  +-------------------------------------------------------------+
                                 |
                                 v
  +-------------------------------------------------------------+
  | x86_64 NASM Code Generator (src/codegen.rs)                 |
  | Emits System V AMD64 assembly, stack frames, align checks   |
  +-------------------------------------------------------------+
                                 |
                 +---------------+---------------+
                 |                               |
                 v (Linux)                       v (Windows)
  +-----------------------------+ +-----------------------------+
  | Assembler: nasm -f elf64    | | Assembler: nasm -f win64    |
  | Linker: gcc -no-pie         | | Linker: x86_64-w64-mingw32  |
  +-----------------------------+ +-----------------------------+
                 |                               |
                 v                               v
       Linux ELF64 Binary             Windows PE32+ .exe
```

---

## 2. Walkthrough of Compiler Modules

### 1. Lexical Analysis (`src/lexer.rs`)
The lexer scans the input UTF-8 source stream character-by-character. It converts conversational keywords (`remember`, `action`, `otherwise`, `repeat`), symbols, numbers, and identifiers into a strongly typed `Token` stream. 
- String literals with embedded expressions (e.g., `"Value: {x + 1}"`) are parsed into distinct template chunks and expression tokens.
- Comments (`//`, `#`, `;`) are stripped without polluting the AST.

### 2. Recursive Descent Parser (`src/parser.rs`)
The parser converts the token stream into an Abstract Syntax Tree (`Program` and `Statement` nodes).
- Statements are parsed top-down using recursive descent.
- When an imported file is encountered (`use "path/file.rv"`), the parser recursively reads and parses the target file, deduplicating symbols and merging AST subtrees.

### 3. Typechecker & Semantic Validation (`src/typechecker.rs`)
The typechecker verifies that the AST adheres to all declared architectural directives:
- When `remove Basic` is present, it validates that every variable and function declares a valid static type (`Int`, `String`, `Bool`, `Ptr`, or user-defined `struct`).
- Checks that functions are called with the correct argument arity and compatible types.
- Verifies that `extern "C"` declarations conform to C ABI calling conventions.

### 4. AST Optimizer (`src/optimizer.rs`)
Before emitting assembly code, Runvoid passes the AST through optimization passes:
- **Constant Folding:** Expressions involving compile-time literals (e.g., `remember x = 10 * 60 + 5`) are evaluated during compilation into `605`, eliminating runtime CPU cycles.
- **Dead Code Elimination (DCE):** Unreachable branches inside `if false` or code after early `give` statements are pruned.

### 5. Assembly Code Generator (`src/codegen.rs`)
The code generator walks the optimized AST and outputs clean, commented x86_64 NASM assembly code:
- **Stack Alignment:** Automatically maintains strict 16-byte stack frame alignment before any external function call, preventing SIMD alignment faults on Windows and Linux.
- **Cross-Platform Read-Only Sections:** Emits `section .rodata` on Linux and `section .rdata` on Windows.
- **Calling Conventions:** Generates System V AMD64 ABI compliant code (`rdi`, `rsi`, `rdx`, `rcx`, `r8`, `r9`).

### 6. C Runtime Layer (`runtime/gui.c` & `runtime/runtime.asm`)
The native runtime provides high-performance implementations for strings, dynamic collections, terminal I/O, dialogs, audio, and graphics.
- Marked with `RV_API` (`__attribute__((sysv_abi))`) to ensure seamless interop between the NASM assembly output and MinGW GCC.
- Compiles with zero external dependencies on Linux and Windows.

### 7. Compiler Orchestrator (`src/compiler.rs`)
Manages the build pipeline: invokes `nasm`, compiles `runtime/gui.c`, links the final object files using GCC or MinGW GCC, and cleans up temporary build artifacts.

---

## 3. Tooling and Editor Ecosystem

Runvoid ships with first-class developer tooling:

- **Source Formatter (`src/formatter.rs`):** Powers `runvoid fmt [-w]`, parsing and formatting Runvoid source code into standard 4-space indented canonical style.
- **Interactive REPL (`src/repl.rs`):** An interactive terminal shell that evaluates statements line-by-line while maintaining scope state.
- **VS Code Extension (`editors/vscode/`):** Contains official TextMate grammar definitions (`syntaxes/runvoid.tmLanguage.json`), language configuration (`language-configuration.json`), and code snippets for Visual Studio Code.

