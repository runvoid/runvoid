# The Runvoid Programming Language Reference & Guide (v0.2.0)

Welcome to the official **Runvoid** documentation! 
This guide comprehensively covers the language syntax, runtime semantics, memory model, and compiler architecture.

---

## Table of Contents
1. [Introduction and Philosophy](#1-introduction-and-philosophy)
2. [Installation & Requirements](#2-installation--requirements)
3. [Command-Line Interface (CLI)](#3-command-line-interface-cli)
4. [Basic Syntax (Beginner Friendly)](#4-basic-syntax-beginner-friendly)
   - [Printing Output (`say`, `say_same`)](#41-printing-output-say-say_same)
   - [Variables (`remember`)](#42-variables-remember)
   - [String Interpolation](#43-string-interpolation)
   - [User Input (`ask`)](#44-user-input-ask)
   - [Arithmetic and Logic](#45-arithmetic-and-logic)
5. [Control Flow](#5-control-flow)
   - [Branching (`if`, `otherwise if`, `otherwise`)](#51-branching-if-otherwise-if-otherwise)
   - [Fixed Repetition Loop (`repeat`)](#52-fixed-repetition-loop-repeat)
   - [Conditional Loop (`while`)](#53-conditional-loop-while)
   - [Loop Interruption (`stop`, `skip`)](#54-loop-interruption-stop-skip)
6. [Functions (`action`, `give`)](#6-functions-action-give)
7. [File I/O, Timers, and Random Numbers](#7-file-io-timers-and-random-numbers)
   - [Reading & Writing Files (`read`, `write ... into ...`)](#71-reading--writing-files-read-write--into-)
   - [Timers & Sleep (`wait`)](#72-timers--sleep-wait)
   - [Random Numbers (`random ... to ...`)](#73-random-numbers-random--to-)
   - [Executing Bash Commands (`run`)](#74-executing-bash-commands-run)
8. [Built-in GUI & Immediate Mode (`imrv`)](#8-built-in-gui--immediate-mode-imrv)
   - [Declarative Windows (`window`, `label`, `button`, `checkbox`)](#81-declarative-windows-window-label-button-checkbox)
   - [Immediate Mode GUI (`imrv draw ...`)](#82-immediate-mode-gui-imrv-draw-)
9. [Memory Management & Garbage Collector (GC)](#9-memory-management--garbage-collector-gc)
   - [Standard Mode (Automatic GC)](#91-standard-mode-automatic-gc)
   - [Disabling the GC (`remove garbageC`)](#92-disabling-the-gc-remove-garbagec)
10. [Pro Dev & Systems Mode (`add Advanced`)](#10-pro-dev-mode-add-advanced)
    - [Requirements & Directives](#101-requirements--directives)
    - [Modular Standard Library](#102-modular-standard-library)
    - [Bare-Metal Freestanding Mode](#103-bare-metal-freestanding-mode)
    - [Hardware Inline Assembly](#104-hardware-inline-assembly)
    - [CPU Cycle Profiling](#105-cpu-cycle-profiling)
    - [Raw Pointers & Manual Memory](#106-raw-pointers--manual-memory)
    - [C-Compatible POD Structs](#107-c-compatible-pod-structs)
    - [Direct C Foreign Function Interface](#108-direct-c-foreign-function-interface)
    - [Multithreading & Atomic Operations](#109-multithreading--atomic-operations)
    - [Compiler Optimizations](#1010-compiler-optimizations)
11. [Compiler Architecture](#11-compiler-architecture)
12. [Beginner Conversational Extensions](#12-beginner-conversational-extensions)
    - [Natural Word Lists & Iteration](#121-natural-word-lists--iteration)
    - [Colorful Console Output & Terminal Sounds](#122-colorful-console-output--terminal-sounds)
    - [2D Hardware Screen Canvas](#123-2d-hardware-screen-canvas)
    - [Interactive Console Menus & Popups](#124-interactive-console-menus--popups)
    - [File System, Web & String Utilities](#125-file-system-web--string-utilities)
    - [Performance Benchmarking](#126-performance-benchmarking)
13. [Developer Tooling & Ecosystem](#13-developer-tooling--ecosystem)
    - [Formatter (`runvoid fmt`)](#131-formatter-runvoid-fmt)
    - [Starter Templates (`runvoid new`)](#132-starter-templates-runvoid-new)
    - [Interactive Cheat Sheet (`runvoid cheat`)](#133-interactive-cheat-sheet-runvoid-cheat)
    - [Official VS Code Extension](#134-official-vs-code-extension)

---

## 1. Introduction and Philosophy

The **Runvoid** language is built around three core design principles:
1. **Effortless Readability:** The syntax reads like plain conversational English. Even someone sitting in front of a computer for the first time can immediately comprehend what a program does.
2. **Scripting Agility + Machine Code Speed:** Runvoid programs can be executed instantly with a single command like Bash or Python scripts. However, beneath the surface, the Rust-based compiler translates code directly into pure **x86_64 NASM assembly**, linking it into native standalone ELF executables without any interpreter or virtual machine overhead.
3. **Seamless Transition to Systems Programming:** When a project demands peak efficiency and zero overhead, simple compiler directives disable the garbage collector, enforce strict static typing, and unleash aggressive compiler optimizations.

---

## 2. Installation & Requirements

### System Requirements:
- **Operating System:** Linux (x86_64).
- **Build Tools:**
  - `rustc` & `cargo` (Rust 1.80+)
  - `nasm` (Version 2.15+)
  - `gcc` (For linking final ELF executables)
  - `libX11` (Optional, only required when compiling GUI applications)

### Building from Source:
```bash
git clone https://github.com/runvoid/runvoid.git
cd runvoid
cargo build --release
```
The compiled executable will be created at `target/release/runvoid`. You can install it globally to `/usr/local/bin`:
```bash
sudo cp target/release/runvoid /usr/local/bin/
```

---

## 3. Command-Line Interface (CLI)

The Runvoid compiler CLI provides three primary subcommands:

### 1. `runvoid run <file.rv>`
Compiles the file into a temporary executable and immediately executes it (ideal for fast feedback and scripting workflows):
```bash
runvoid run script.rv
```

### 2. `runvoid build <file.rv> [-o <name>] [--verbose]`
Compiles the source code into an optimized, standalone native ELF binary:
```bash
runvoid build script.rv -o my_program
./my_program
```

### 3. `runvoid emit-asm <file.rv>`
Emits the generated x86_64 NASM assembly code directly for inspection and debugging:
```bash
runvoid emit-asm script.rv
```

---

## 4. Basic Syntax (Beginner Friendly)

### 4.1 Printing Output (`say`, `say_same`)
- `say` prints an expression followed by a newline.
- `say_same` prints without a trailing newline.

```runvoid
say "Hello, world!"
say_same "Loading: "
say 100
```

### 4.2 Variables (`remember`)
Variables are declared using the clear and natural keyword `remember`:
```runvoid
remember user = "Alex"
remember age = 25
remember is_ready = true

# Reassigning an existing variable:
age = 26
```

### 4.3 String Interpolation
Any valid expression can be embedded directly into string literals using curly braces `{...}`:
```runvoid
remember count = 5
say "You have {count} new messages and {count * 2} points!"
```

### 4.4 User Input (`ask`)
The `ask` statement prints a prompt and waits for user input from standard input:
```runvoid
remember name = ask "What is your name? "
say "Pleased to meet you, {name}!"
```

### 4.5 Arithmetic and Logic
- **Arithmetic:** `+`, `-`, `*`, `/`, `%`
- **Comparison:** `is` (equals), `is not` (not equals), `<`, `>`, `<=`, `>=` (symbolic `==` and `!=` are also accepted)
- **Boolean Logic:** `and`, `or`, `not`

```runvoid
remember x = 10
remember y = 20
if x < y and not (x is 0) {
    say "Condition satisfied!"
}
```

---

## 5. Control Flow

### 5.1 Branching (`if`, `otherwise if`, `otherwise`)
Runvoid replaces cryptic `else` constructs with readable `otherwise` blocks:
```runvoid
remember score = 85

if score > 90 {
    say "Excellent!"
} otherwise if score >= 80 {
    say "Good job!"
} otherwise {
    say "Needs practice!"
}
```

### 5.2 Fixed Repetition Loop (`repeat`)
```runvoid
# Repeat a block 3 times:
repeat 3 {
    say "Hello!"
}

# Repeat with loop index variable:
repeat 5 as step {
    say "Step {step} of 5"
}
```

### 5.3 Conditional Loop (`while`)
```runvoid
remember counter = 1
while counter <= 5 {
    say "Iteration: {counter}"
    counter = counter + 1
}
```

### 5.4 Loop Interruption (`stop`, `skip`)
- `stop` — breaks out of the loop immediately (*break*).
- `skip` — advances to the next iteration (*continue*).

```runvoid
repeat 10 as i {
    if i is 3 {
        skip
    }
    if i is 7 {
        stop
    }
    say i
}
```

---

## 6. Functions (`action`, `give`)

Functions are defined with the `action` keyword, and values are returned with `give`:
```runvoid
action multiply(a, b) {
    give a * b
}

remember result = multiply(6, 7)
say "6 * 7 = {result}"
```

---

## 7. File I/O, Timers, and Random Numbers

### 7.1 Reading & Writing Files (`read`, `write ... into ...`)
```runvoid
# Write text to a file:
write "Important notes" into "notes.txt"

# Read back from a file:
remember text = read "notes.txt"
say "Read: {text}"
```

### 7.2 Timers & Sleep (`wait`)
```runvoid
say "Waiting for 2 seconds..."
wait 2
say "Time is up!"
```

### 7.3 Random Numbers (`random ... to ...`)
```runvoid
remember dice = random 1 to 6
say "Rolled number: {dice}"
```

### 7.4 Executing Bash Commands (`run`)
Runvoid integrates natively with the Linux shell environment:
```runvoid
run "echo 'Hello from Linux Bash' && uname -r"
```

---

## 8. Built-in GUI & Immediate Mode (`imrv`)

### 8.1 Declarative Windows (`window`, `label`, `button`, `checkbox`)
Create native X11/XWayland GUI windows declaratively:
```runvoid
window "Runvoid Control Panel", 450, 300 {
    label "Welcome to Runvoid Native GUI!"
    checkbox "Enable Dark Mode", 1
    checkbox "Auto-Save", 0

    button "Save" {
        say "Settings saved!"
    }
}
```

### 8.2 Immediate Mode GUI (`imrv draw ...`)
Designed for dynamic tools, game panels, and debug overlays:
```runvoid
imrv draw text "Developer HUD"
remember sound = imrv draw checkbox "Enable Sound", 1
imrv draw button "Start"
```

---

## 9. Memory Management & Garbage Collector (GC)

### 9.1 Standard Mode (Automatic GC)
By default, all dynamic string allocations are automatically managed by a lightweight Mark-and-Sweep garbage collector integrated into the native runtime. Developers do not need to manually allocate or free memory.

### 9.2 Disabling the GC (`remove garbageC`)
Placing the directive at the very top of your file:
```runvoid
remove garbageC
```
completely strips the garbage collection subsystem. Memory allocations are routed through a direct bump/heap allocator without GC tracking pauses or metadata overhead.

---

## 10. Pro Dev & Systems Mode (`add Advanced`)

When maximum performance, absolute control, and minimal binary footprint are required, Runvoid transforms into an uncompromising systems programming language.

### 10.1 Requirements & Directives
Pro Dev mode requires explicitly opting out of high-level runtime conveniences:
```runvoid
remove garbageC
remove Basic
add Advanced
```
In this mode:
- The garbage collector is completely omitted (Zero-GC).
- Strict static typing is strictly enforced (`: Int`, `: String`, `: Bool`, `: Ptr`, or custom struct names).
- Untyped variables or undeclared functions are rejected at compile time.

### 10.2 Modular Standard Library
Instead of bundling a monolithic standard library, Pro Mode requires explicit module imports:
```runvoid
use ior       # Low-level I/O: say, ask, user dialogs
use math      # High-performance math: sqrt, sin, cos, pow, abs (links -lm)
use sys       # System primitives: exit, getpid
use mem       # Manual memory allocations: alloc, free
use fs        # Direct file operations: read, write, copy, delete
use net       # TCP sockets: tcp_listen, tcp_accept, tcp_connect, tcp_send, tcp_recv, tcp_close
use thread    # Native OS threads: thread { ... }, rv_thread_join (links -lpthread)
```

#### Dynamic Library Linking:
Link external C libraries seamlessly using `use lib`:
```runvoid
use lib "raylib"
use lib "sqlite3"
```
The compiler passes these directly as `-lraylib` and `-lsqlite3` to the linker.

#### Multi-File Imports:
Break programs into clean modules:
```runvoid
use "helpers.rv"
```
The AST of imported `.rv` files is resolved and combined at compile time.

### 10.3 Bare-Metal Freestanding Mode
For kernel, bootloader, or embedded programming where neither `libc` nor any runtime is permissible:
```runvoid
remove garbageC
remove Basic
remove Linux
add Advanced
add Freestanding

asm {
    mov rax, 60    # sys_exit
    xor rdi, rdi   # status 0
    syscall
}
```
In freestanding mode:
- The compiler emits `global _start` as the single entry point.
- Zero runtime functions or libc symbols are referenced.
- Linked directly with GNU `ld -s` into an ultra-minimal standalone ELF executable.

### 10.4 Hardware Inline Assembly
Execute raw x86_64 CPU instructions directly inside your Runvoid codebase:
```runvoid
asm {
    mov rax, 42
    imul rax, 10
}
```

### 10.5 CPU Cycle Profiling
Profile code execution speed directly in hardware CPU clock cycles using the x86 `RDTSC` instruction:
```runvoid
measure cycles {
    remember a: Int = 100
    remember b: Int = 200
    say a + b
}
```
Outputs execution time directly in exact CPU clock cycles: `⏱️ Executed in 142 CPU cycles`.

### 10.6 Raw Pointers & Manual Memory
Direct memory manipulation with C-equivalent speed and control:
```runvoid
use mem

remember val: Int = 42
remember ptr: Ptr = addr val     # Take memory address

say @ptr                         # Dereference read

@ptr = 100                       # Dereference write
say val                          # 100

remember heap: Ptr = alloc 64    # Allocate 64 bytes on heap
@heap = 777
free heap                        # Free heap allocation
```

### 10.7 C-Compatible POD Structs
Define 8-byte aligned Plain Old Data structures compatible with C ABI layout:
```runvoid
struct Point3D {
    x,
    y,
    z
}

remember pt: Point3D = Point3D(10, 20, 30)
say pt.x
say pt.y

pt.z = 99
say pt.z
```

### 10.8 Direct C Foreign Function Interface
Call any C standard library or third-party C library function directly without wrapper overhead:
```runvoid
extern "C" {
    action puts(s: String) -> Int
    action abs(n: Int) -> Int
}

puts("Hello from native C puts!")
remember neg: Int = 0 - 50
say abs(neg)
```
Runvoid strings automatically pass their internal null-terminated C string pointer (`char*`) to extern functions.

### 10.9 Multithreading & Atomic Operations
Spawn native Linux POSIX threads and perform hardware-locked atomic operations:
```runvoid
use thread

remember shared_counter: Int = 0

thread {
    atomic add shared_counter, 50
}

wait 1
say shared_counter
```
`atomic add` compiles directly to `lock add [reg], rax`, ensuring zero-race condition multithreaded synchronization.

### 10.10 Compiler Optimizations
Enabling `add Advanced` unlocks aggressive multi-tier optimization passes:
- **Constant Folding:** Expressions involving constants are evaluated at compile time (`10 * 10 + 50` -> `150`).
- **Dead Code Elimination (DCE):** Unreachable blocks and dead instructions after return/break/continue are pruned from the AST.
- **Peephole Optimizations:** Register zeroing idioms (`xor eax, eax`), increment/decrement substitutions, and redundant stack operations eliminated.
- **Link-Time Optimization (LTO) & Stripping:** Builds with `nasm -O3` and `gcc -O3 -flto -s -Wl,--gc-sections`, producing ultra-compact native binaries starting from ~15 KB.

---

## 11. Compiler Architecture

The Runvoid compiler is written in **Rust** targeting **NASM (x86_64 Linux)** following the System V AMD64 ABI:
1. **Frontend (Rust):**
   - [`src/lexer.rs`](file:///home/runvoid/Projects/runvoidLanguage/src/lexer.rs): Lexical analyzer with native string interpolation support.
   - [`src/parser.rs`](file:///home/runvoid/Projects/runvoidLanguage/src/parser.rs): Recursive descent parser building the AST.
   - [`src/typechecker.rs`](file:///home/runvoid/Projects/runvoidLanguage/src/typechecker.rs): Static type and directive validation pass.
   - [`src/optimizer.rs`](file:///home/runvoid/Projects/runvoidLanguage/src/optimizer.rs): Multi-pass AST constant folding and dead code elimination.
2. **Backend (NASM & C):**
   - [`src/codegen.rs`](file:///home/runvoid/Projects/runvoidLanguage/src/codegen.rs): Translates AST into high-performance x86_64 NASM assembly.
   - [`runtime/runtime.asm`](file:///home/runvoid/Projects/runvoidLanguage/runtime/runtime.asm): Direct Linux syscalls (`sys_write`, `sys_read`, `sys_nanosleep`, `sys_getrandom`, `sys_open`, `sys_clone`).
   - [`runtime/gui.c`](file:///home/runvoid/Projects/runvoidLanguage/runtime/gui.c): Minimalist X11/XWayland windowing and immediate-mode GUI engine.
3. **Linker:**
   - Links object files into a standalone Linux ELF executable.

---

## 12. Beginner Conversational Extensions

### 12.1 Natural Word Lists & Iteration
```runvoid
remember backpack = "Sword", "Health Potion", "Compass"
add "Magic Ring" to backpack
remove "Compass" from backpack

if backpack has "Sword" {
    say "Ready for battle!"
}

say "Total items: {count backpack}"

for every item in backpack {
    say " - {item}"
}
```

### 12.2 Colorful Console Output & Terminal Sounds
```runvoid
say green "Build Succeeded!"
say red "Critical Error!"
say yellow "Warning: disk almost full"
say blue "Notice: server listening"
say same "Progress: "
say same green "[100%]\n"

beep                     # System terminal bell sound
speak "Hello adventurer!" # Text-to-speech
```

### 12.3 2D Hardware Screen Canvas
```runvoid
screen "Game Canvas", 640, 480 {
    draw box at 0, 0, size 640, 480, color "black"
    draw circle at 320, 240, size 40, color "cyan"
    draw line from 0, 0 to 640, 480, color "red"
    draw text "Runvoid 2D Canvas", at 200, 50, color "yellow"
}
```

### 12.4 Interactive Console Menus & Popups
```runvoid
remember weapon = choose "Choose weapon:", "Sword", "Bow", "Magic Staff"
remember confirmed = ask user "Do you want to proceed?"
alert "Dungeon Gate Unlocked!"
```

### 12.5 File System, Web & String Utilities
```runvoid
create folder "backups"
copy file "save.dat" to "backups/save.dat"
delete file "old.log"

if file "save.dat" exists {
    say "Save found!"
}

download "https://runvoid.org/logo.png" into "logo.png"
remember data = read web "https://api.github.com"

make weapon uppercase
make weapon lowercase
make weapon trim
remember replaced = replace "Sword" with "Excalibur" in weapon
```

### 12.6 Performance Benchmarking
```runvoid
measure time {
    remember count = 0
    repeat 1000000 times as i {
        set count = count + 1
    }
}
```

---

## 13. Developer Tooling & Ecosystem

### 13.1 Formatter (`runvoid fmt`)
Automatically format indentation, spacing, and block structure:
```bash
runvoid fmt script.rv       # print formatted output
runvoid fmt -w script.rv    # format in-place
```

### 13.2 Starter Templates (`runvoid new`)
Scaffold runnable beginner projects instantly:
```bash
runvoid new game my_game.rv    # 2D Screen canvas game
runvoid new gui my_app.rv      # Native Desktop GUI
runvoid new script my_code.rv  # Conversational beginner script
```

### 13.3 Interactive Cheat Sheet (`runvoid cheat`)
Display a terminal quick-reference guide anytime:
```bash
runvoid cheat
```

### 13.4 Official VS Code Extension
Located at [`editors/vscode/`](file:///home/runvoid/Projects/runvoidLanguage/editors/vscode/):
- Full TextMate syntax highlighting for all Runvoid keywords, directives, colors, strings, and `{...}` interpolations.
- Auto-closing brackets and quotes.
- Automatic 4-space indentation and block folding.
