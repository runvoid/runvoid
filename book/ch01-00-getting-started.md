# 1. Getting Started

Let's begin your journey with Runvoid!

In this opening chapter, you will take your first concrete steps into the Runvoid ecosystem. You will install the compiler toolchain, write and dissect your first native program, and learn how to wield the unified command-line toolchain.

---

## What Makes Getting Started with Runvoid Unique?

Most compiled languages require hours of setup: downloading multiple gigabytes of toolchains, configuring environment variables, writing complex `Makefile` or `CMakeLists.txt` build scripts, and memorizing arcane syntax before you can print a single string.

Runvoid rejects this friction. 

```
 Traditional Compiled Systems Setup:
 [ Install LLVM (2GB) ] -> [ Configure Linkers ] -> [ Write CMake/Make ] -> [ Compile ]
                                                                                |
 Runvoid 5-Minute Setup:                                                        v
 [ Download runvoid (15MB) ] -----------------------------------------> [ Instant Run! ]
```

With Runvoid:
1. **Zero Runtime Virtual Machines:** No Python interpreter, no JVM, and no Node.js engine are required.
2. **Instant Feedback:** Run scripts directly with `runvoid run file.rv` like Python, while producing pure x86_64 machine code like C.
3. **Batteries Included:** Built-in test runner (`test`), auto-formatter (`fmt`), interactive shell (`repl`), and live code reloader (`watch`).
4. **First-Class Project Manifests:** Manage multi-file applications effortlessly with `runvoid init` and `runvoid.toml`.

---

## Chapter Overview

Specifically, this chapter covers:

* **1.1 Installation & System Requirements:** How to install the Runvoid compiler, assembler (`nasm`), and linkers on **Linux** and **Windows**, configure cross-compilation toolchains, verify cryptographic SHA-256 release checksums, and run inside Docker or VS Code DevContainers.
* **1.2 Hello, World!:** Writing your first program, understanding top-level execution, dissecting the generated x86_64 assembly output, and inspecting binary symbols with `objdump` and `strings`.
* **1.3 The Compiler & CLI Tooling:** Navigating the `runvoid` CLI—managing projects with `runvoid.toml`, compiling release binaries, formatting code, running test suites, launching interactive REPL sessions, and hot-reloading code with `runvoid watch`.

---

## The Runvoid Development Cycle

Whether building micro-utilities or complex distributed systems, the standard Runvoid development lifecycle flows through four rapid phases:

```
+-------------------------------------------------------------------------+
| 1. Scaffold / Initialize                                                |
|    $ runvoid init my_app                                                |
+-------------------------------------------------------------------------+
                                   |
                                   v
+-------------------------------------------------------------------------+
| 2. Interactive Development & Live Reloading                             |
|    $ runvoid watch                                                      |
+-------------------------------------------------------------------------+
                                   |
                                   v
+-------------------------------------------------------------------------+
| 3. Automated Verification & Testing                                     |
|    $ runvoid test                                                       |
+-------------------------------------------------------------------------+
                                   |
                                   v
+-------------------------------------------------------------------------+
| 4. Native Release Compilation                                           |
|    $ runvoid build --target linux                                       |
|    $ runvoid build --target windows                                     |
+-------------------------------------------------------------------------+
```

---

## Editor Support: Visual Studio Code

Runvoid includes an official Visual Studio Code extension located in `editors/vscode/` within the repository:
- **Syntax Highlighting:** Full TextMate grammar highlighting keywords (`remember`, `say`, `action`, `verify that`), string templates, and Pro mode directives.
- **Bracket Matching & Auto-Closing:** Automatic indentation and brace pair colorization.
- **Code Snippets:** Rapid templates for functions, loops, tests, and canvas windows (`rv-action`, `rv-test`, `rv-screen`).

To install the extension, package it with `vsce package` or download the pre-built `runvoid-vscode-1.3.0.vsix` artifact from official GitHub releases.

Whether you are running an Arch Linux workstation, an Ubuntu server, or a Windows 11 development laptop, Runvoid is built to feel right at home. Let’s get started!
