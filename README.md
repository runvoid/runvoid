# The Runvoid Programming Language (v0.2.0)

<div align="center">

[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.2.0-brightgreen.svg)](Cargo.toml)
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
- 🎨 **Conversational Natural Lists:** `remember backpack = "sword", "potion"`, `for every item in backpack`, `add item to list`, `remove item from list`, `if list has item`, `{count list}`.
- 🌈 **Terminal Colors & Audio:** `say green "Done!"`, `say red "Error!"`, `say yellow "Warning!"`, terminal `beep`, and text-to-speech `speak`.
- 🕹️ **2D Screen Canvas & GUI:** Draw hardware-accelerated 2D games (`screen "Title", 640, 480 { draw circle ...; draw box ...; draw line ... }`) and native desktop windows (`window "App" { button "Click" { ... } }`).
- 🧭 **Interactive CLI & Dialogs:** `choose` menus, hidden password prompts (`ask hidden`), modal confirmations (`ask user`), and graphical alerts (`alert`).
- 📁 **Native Filesystem & Web APIs:** `create folder`, `delete file`, `copy file ... to ...`, `file ... exists`, `download url into path`, `read web url`.
- ⚡ **Benchmarking & Text Manipulation:** `measure time { ... }`, `make var uppercase / lowercase / trim`, `replace a with b in str`, `starts with`, `ends with`.
- 🛠️ **Developer Tooling & Ecosystem:**
  - Built-in formatter: `runvoid fmt <file>`
  - Starter templates: `runvoid new <game|gui|script>`
  - Terminal cheat sheet: `runvoid cheat`
  - Official VS Code extension (`editors/vscode/`)
- ⚙️ **Dual-Mode Language Paradigm:**
  - **Beginner Mode (Default):** Built-in Garbage Collector (GC), automatic type inference, zero boilerplate, batteries included.
  - **Pro Systems Mode (`add Advanced`):** Zero-GC (`remove garbageC`), strict static typing (`remove Basic`), modular standard library (`use ior`, `math`, `sys`, `mem`, `fs`, `net`, `thread`), external C dynamic libraries (`use lib "..."`), multi-file imports (`use "mod.rv"`), bare-metal freestanding binaries (`remove Linux`, `add Freestanding`), direct inline x86_64 assembly (`asm { ... }`), CPU cycle benchmarking (`measure cycles { ... }`), raw pointers & memory allocation (`addr`, `@`, `alloc`, `free`), C-compatible POD structs (`struct`), native C FFI (`extern "C"`), OS multithreading (`thread`), and hardware atomic operations (`atomic add`).

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

### 4. Interactive Terminal Cheat Sheet (`cheat` mode):
```bash
runvoid cheat
```

### 5. Create a Project Template (`new` mode):
```bash
runvoid new game my_game.rv    # 2D Screen canvas starter
runvoid new gui my_gui.rv      # Desktop GUI starter
runvoid new script my_app.rv   # Conversational starter script
```

### 6. Auto-Format Code (`fmt` mode):
```bash
runvoid fmt -w hello.rv
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
