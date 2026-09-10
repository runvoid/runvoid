# The Runvoid Programming Language (v0.1.0)

<div align="center">

[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.1.0-brightgreen.svg)](Cargo.toml)
[![Platform](https://img.shields.io/badge/platform-Linux%20x86__64-orange.svg)](#requirements)
[![Compiler](https://img.shields.io/badge/compiler-Rust%20%2B%20NASM-red.svg)](src/)

**Simple and intuitive like Python & Bash. Fast, compact, and bare-metal like C & Assembly.**

[Documentation](docs/DOCUMENTATION.md) • [Examples](examples/) • [Architecture](docs/DOCUMENTATION.md#11-compiler-architecture) • [License](LICENSE)

</div>

---

## 🚀 About Runvoid

**Runvoid** is a modern programming language designed so that anyone can start coding immediately with a syntax that reads like natural English. Runvoid is **not** a slow interpreted script: its native compiler, written in **Rust**, compiles source code directly into **pure x86_64 NASM assembly** and links it into standalone ELF executables as small as **15 KB**—with zero external virtual machines or heavy runtimes.

### ✨ Key Features:
- 💡 **Human-Centric Syntax:** `say` for printing, `remember` for variables, `ask` for input, `if / otherwise` for control flow, `repeat` and `while` for loops.
- 🧵 **Native String Interpolation:** `say "Hello, {name}! You have {score} points."` supported out of the box.
- 🪟 **Built-in Native GUI:** Declarative X11/XWayland windows (`window "App" { button "Click" { ... } }`) and Immediate Mode GUI (`imrv draw ...`).
- 📂 **Expressive I/O & System APIs:** `read "file.txt"`, `write "data" into "file.txt"`, `wait 1`, `random 1 to 100`, and direct Bash execution via `run "command"`.
- ⚙️ **Dual-Mode Language Paradigm:**
  - **Beginner Mode (Default):** Built-in Garbage Collector (GC), automatic type inference, zero boilerplate, batteries included.
  - **Pro Dev Mode (`add Advanced`):** Disable GC (`remove garbageC`), enforce strict static typing (`remove Basic`), zero-overhead modular I/O (`use ior`), and aggressive compiler optimizations (Constant Folding, Dead Code Elimination, Peephole assembly rewrites, `-O3`, LTO, section stripping).

---

## 📦 Installation & Requirements

### System Requirements:
- **Operating System:** Linux x86_64
- **Rust Toolchain:** `rustc` & `cargo` (>= 1.80)
- **Assembler:** `nasm` (>= 2.15)
- **Linker:** `gcc` (for final ELF linking)
- **X11 Libraries:** `libX11` (optional; only needed when building GUI applications)

### Building from Source:
```bash
git clone https://github.com/runvoid/runvoid.git
cd runvoid
cargo build --release
```
The compiled binary will be placed at `target/release/runvoid`. You can optionally install it to `/usr/local/bin`:
```bash
sudo cp target/release/runvoid /usr/local/bin/
```

---

## ⚡ Quick Start

### 1. Instant Script Execution (`run` mode):
Create `hello.rv`:
```runvoid
say "Hello, Runvoid world!"

remember user = "Alex"
remember lucky_number = random 1 to 100

say "Hello, {user}! Your lucky number is: {lucky_number}"
```

Run directly without manual build steps:
```bash
runvoid run hello.rv
```

### 2. Standalone Native Compilation (`build` mode):
```bash
runvoid build hello.rv -o my_app
./my_app
```

### 3. Inspect Generated Assembly (`emit-asm` mode):
```bash
runvoid emit-asm hello.rv
```

---

## 🎨 Code Showcase

### 1. Native Windowed GUI:
```runvoid
window "Runvoid Control Panel", 450, 320 {
    label "Welcome to Runvoid GUI!"
    checkbox "Enable Dark Mode", 1
    checkbox "Automatic Updates", 0

    button "Save Settings" {
        say "Settings saved successfully!"
    }
}
```

### 2. File I/O, Sleep Timers & Random Numbers:
```runvoid
write "Runvoid v0.1 works great!" into "note.txt"
remember content = read "note.txt"
say "Read: {content}"

say "Waiting for 1 second..."
wait 1
say "Done waiting, proceeding!"
```

### 3. Pro Dev Mode (Zero-Overhead & High Performance):
```runvoid
remove garbageC
remove Basic
add Advanced
use ior

action square(n: Int): Int {
    give n * n
}

# Constant folding computes this statically at compile-time (150):
remember folded: Int = 10 * 10 + 50
say "Folded constant: {folded}"

remember val: Int = square(8)
say "Square of 8 = {val}"
```

---

## 📖 Documentation

For full details on syntax, language specification, memory model, compiler flags, and the AST pipeline, check the complete official handbook:
👉 **[docs/DOCUMENTATION.md](docs/DOCUMENTATION.md)**

---

## 🧪 Running Tests

Run the test suite to verify the compiler and typechecker:
```bash
cargo test
```

---

## 📄 License

The Runvoid programming language and compiler are licensed under the **GNU General Public License v3.0 (GPL-3.0-or-later)**. See the [LICENSE](LICENSE) file for complete details.
