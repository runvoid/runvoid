# 9. Compiler Architecture & Ecosystem

The Runvoid compiler is written in **Rust** and targets **NASM x86_64 Linux** following the System V AMD64 ABI.

## Compiler Internal Pipeline

1. **`src/lexer.rs`**: Tokenizes source text, scanning identifiers, numbers, keywords, and interpolated strings (`"Hello {user}"`).
2. **`src/parser.rs`**: Builds the Abstract Syntax Tree (AST) using a recursive descent parser. Handles recursive multi-file imports (`use "other.rv"`).
3. **`src/typechecker.rs`**: Verifies directive constraints, checks module requirements, verifies type annotations in Pro Mode, and validates struct schemas.
4. **`src/optimizer.rs`**: Implements AST optimization passes including compile-time Constant Folding and Dead Code Elimination (DCE).
5. **`src/codegen.rs`**: Emits high-performance x86_64 NASM assembly code.
6. **`runtime/runtime.asm` & `runtime/gui.c`**: Native Linux syscall implementations and minimal X11 windowing backend.
7. **`src/compiler.rs`**: Orchestrates assembler (`nasm`) and linker (`gcc` / `ld`) invocations in isolated temporary workspaces.

## Tooling & Ecosystem

- **`src/formatter.rs`**: The built-in formatter powering `runvoid fmt [-w]`.
- **VS Code Extension (`editors/vscode/`)**: Official TextMate syntax grammar, bracket-matching rules, and snippet configuration for Visual Studio Code.
