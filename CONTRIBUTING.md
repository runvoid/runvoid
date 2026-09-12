# Contributing to Runvoid

First off, thank you for considering contributing to Runvoid! Contributions from developers of all skill levels are welcome.

---

## Code of Conduct
By participating in this project, you agree to abide by the terms of our [Code of Conduct](CODE_OF_CONDUCT.md).

---

## Crucial Invariant: Language & Encoding
**Runvoid has a strict codebase invariant: All repository files (source code, tests, documentation, comments, commit messages, and workflows) must contain ONLY English text.**
Before committing any changes, you must run:
```bash
rg '\p{Cyrillic}'
```
This command must return zero matches. The automated GitHub Actions CI will fail any pull request containing Cyrillic characters.

---

## Local Development Setup

### Prerequisites
- **Rust Toolchain:** `rustc` and `cargo` (>= 1.80)
- **Assembler:** `nasm` (>= 2.15)
- **C Linker & Libraries:** `gcc`, `libX11-dev` (Linux)
- **Windows Cross-Compilation (Optional):** `x86_64-w64-mingw32-gcc`, `wine64`

### Quickstart
1. Fork and clone the repository:
   ```bash
   git clone https://github.com/your-username/runvoid.git
   cd runvoid
   ```
2. Build the project:
   ```bash
   cargo build
   ```
3. Run the automated test suite:
   ```bash
   cargo test
   ```
4. Verify code formatting and linting:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   ```

---

## Pull Request Workflow

1. Create a descriptive feature branch:
   ```bash
   git checkout -b feat/my-new-feature
   ```
2. Write clean, readable code with comprehensive comments.
3. Add automated tests covering the changes:
   - Lexer tests in `src/lexer.rs`
   - Parser tests in `src/parser.rs`
   - Code generation / end-to-end tests in `src/compiler.rs`
4. Rebuild the book if documentation was changed:
   ```bash
   mdbook build
   ```
5. Ensure all checks pass before pushing:
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   rg '\p{Cyrillic}'
   ```
6. Open a pull request against the `main` branch.
