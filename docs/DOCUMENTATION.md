# The Runvoid Programming Language Reference & Guide (v0.3.0)

Welcome to the official **Runvoid** documentation! 
Whether you are writing your very first line of code or building high-performance bare-metal systems, Runvoid is crafted to make programming intuitive, lightning fast, and joyfully productive.

---

## Table of Contents

1. [Introduction & Philosophy](#1-introduction--philosophy)
   - [Core Principles](#11-core-principles)
   - [The Dual-Mode Architecture](#12-the-dual-mode-architecture)
   - [Compiler Pipeline (How Runvoid Works)](#13-compiler-pipeline-how-runvoid-works)
2. [Quickstart: My First 5 Minutes](#2-quickstart-my-first-5-minutes)
   - [Step 1: Your First Program](#step-1-your-first-program)
   - [Step 2: Remembering Values](#step-2-remembering-values)
   - [Step 3: Asking Questions](#step-3-asking-questions)
   - [Step 4: Making Decisions](#step-4-making-decisions)
   - [Step 5: Repeating Actions](#step-5-repeating-actions)
   - [Step 6: Reusable Actions](#step-6-reusable-actions)
3. [Installation & Setup](#3-installation--setup)
4. [Command-Line Interface (CLI)](#4-command-line-interface-cli)
   - [Running Scripts (`run`)](#41-running-scripts-run)
   - [Building Standalone Binaries (`build`)](#42-building-standalone-binaries-build)
   - [Inspecting Assembly (`emit-asm`)](#43-inspecting-assembly-emit-asm)
   - [Formatting Code (`fmt`)](#44-formatting-code-fmt)
   - [Scaffolding Projects (`new`)](#45-scaffolding-projects-new)
   - [Interactive Terminal Cheat Sheet (`cheat`)](#46-interactive-terminal-cheat-sheet-cheat)
   - [Automated Test Runner (`test`)](#47-automated-test-runner-test)
   - [Interactive REPL (`repl`)](#48-interactive-repl-repl)
   - [Hot-Reloading Watcher (`watch`)](#49-hot-reloading-watcher-watch)
5. [Beginner Language Guide](#5-beginner-language-guide)
   - [5.1 Output & Colors (`say`, `say same`, ANSI)](#51-output--colors-say-say-same-ansi)
   - [5.2 Variables & Expressions (`remember`, `{...}`)](#52-variables--expressions-remember-)
   - [5.3 Natural Math, Comparisons & Logic](#53-natural-math-comparisons--logic)
   - [5.4 Control Flow (`if`, `otherwise`, `while`, `repeat`)](#54-control-flow-if-otherwise-while-repeat)
   - [5.5 Conversational Word Lists](#55-conversational-word-lists)
   - [5.6 User Interaction & Menus (`ask`, `choose`, `alert`)](#56-user-interaction--menus-ask-choose-alert)
   - [5.7 Terminal Audio & Speech (`beep`, `speak`)](#57-terminal-audio--speech-beep-speak)
   - [5.8 File System, Web & String Utilities](#58-file-system-web--string-utilities)
   - [5.9 2D Hardware Screen Canvas & Desktop GUI](#59-2d-hardware-screen-canvas--desktop-gui)
   - [5.10 Key-Value Dictionaries & Maps (v0.3.0)](#510-key-value-dictionaries--maps-v030)
   - [5.11 Pattern Matching & Pipelines (v0.3.0)](#511-pattern-matching--pipelines-v030)
   - [5.12 Low-Level Bitwise Operators & Hardware Audio Synthesizer (v0.3.0)](#512-low-level-bitwise-operators--hardware-audio-synthesizer-v030)
   - [5.13 Test Assertions & Test Blocks (v0.3.0)](#513-test-assertions--test-blocks-v030)
6. [Memory Management & Garbage Collection](#6-memory-management--garbage-collection)
   - [Automatic Mark-and-Sweep GC](#61-automatic-mark-and-sweep-gc)
   - [Disabling GC (`remove garbageC`)](#62-disabling-gc-remove-garbagec)
7. [Pro Systems Mode (`add Advanced`)](#7-pro-systems-mode-add-advanced)
   - [7.1 Directives & Setup](#71-directives--setup)
   - [7.2 Strict Static Typing (`remove Basic`)](#72-strict-static-typing-remove-basic)
   - [7.3 Modular Standard Library (`use ior`, `math`, etc.)](#73-modular-standard-library-use-ior-math-etc)
   - [7.4 Dynamic C Libraries & Multi-File Imports](#74-dynamic-c-libraries--multi-file-imports)
   - [7.5 Bare-Metal Freestanding Mode (`add Freestanding`)](#75-bare-metal-freestanding-mode-add-freestanding)
   - [7.6 Hardware Inline Assembly (`asm`)](#76-hardware-inline-assembly-asm)
   - [7.7 CPU Clock Cycle Benchmarking (`measure cycles`)](#77-cpu-clock-cycle-benchmarking-measure-cycles)
   - [7.8 Raw Pointers & Heap Management (`addr`, `@`, `alloc`, `free`)](#78-raw-pointers--heap-management-addr--alloc-free)
   - [7.9 C-Compatible POD Structs](#79-c-compatible-pod-structs)
   - [7.10 C Foreign Function Interface (FFI)](#710-c-foreign-function-interface-ffi)
   - [7.11 Native Multithreading & Hardware Atomics](#711-native-multithreading--hardware-atomics)
   - [7.12 Compiler Optimizations](#712-compiler-optimizations)
8. [Complete Keyword & Directive Reference](#8-complete-keyword--directive-reference)
9. [Troubleshooting & Common Pitfalls](#9-troubleshooting--common-pitfalls)

---

## 1. Introduction & Philosophy

### 1.1 Core Principles

Runvoid was born from a simple question: **Why must programming languages force developers to choose between readable English syntax and raw machine performance?**

Traditionally:
- High-level languages like **Python** or **Ruby** offer intuitive syntax, but run through heavy interpreters or virtual machines, consuming hundreds of megabytes of RAM with slow execution speeds.
- Low-level languages like **C**, **C++**, or **Rust** provide incredible execution speed and zero overhead, but come with complex syntax, steep learning curves, and cumbersome boilerplate.

**Runvoid bridges this gap.**
1. **Reads Like Conversational English:** Instructions are expressed naturally: `say "Hello"`, `remember score = 100`, `repeat 5 times`, `for every item in backpack`, `if file "save.dat" exists`.
2. **Pure Native Compilation:** Runvoid is **not** an interpreted language. The compiler, written in **Rust**, compiles code directly into **pure x86_64 NASM assembly**, which is linked into standalone, dependency-free ELF binaries as small as **15 KB**.
3. **Smooth Path from Novice to Systems Engineer:** Beginners can start writing code without knowing about pointers, stack frames, or type declarations. As projects grow in complexity, single directives unlock bare-metal control, raw pointers, C FFI, and hardware assembly.

---

### 1.2 The Dual-Mode Architecture

Runvoid features two seamlessly integrated modes of operation:

| Feature | Beginner Mode (Default) | Pro Systems Mode (`add Advanced`) |
|---|---|---|
| **Syntax Style** | Conversational, natural English | Systems-oriented, strict syntax |
| **Type System** | Automatic Type Inference | Strict Static Typing (`: Int`, `: String`, `: Ptr`) |
| **Memory Management** | Automatic Mark-and-Sweep GC | Zero-GC / Explicit Allocator (`alloc`, `free`) |
| **Pointers & Hardware** | Abstracted away | Raw pointers (`addr`, `@`), Inline x86_64 Asm |
| **I/O & Modules** | Batteries included (monolithic stdlib) | Zero-overhead modular (`use ior`, `math`, `mem`) |
| **Target Platforms** | Standard Linux OS Desktop & Server | Linux OS or Freestanding Bare-Metal (`_start`) |
| **Compilation Speed** | Instant feedback | High optimizations (`-O3`, LTO, section GC) |

```
+--------------------------------------------------------------------------+
|                        RUNVOID PROGRAMMING LANGUAGE                      |
+--------------------------------------------------------------------------+
                                    |
          +-------------------------+-------------------------+
          |                                                   |
          v                                                   v
  [ BEGINNER MODE ]                                   [ PRO SYSTEMS MODE ]
  - Natural English words                             - Directives: remove garbageC
  - Dynamic type inference                                          remove Basic
  - Automatic Garbage Collector                                     add Advanced
  - Word lists & Canvas                               - Strict static types (: Int, : Ptr)
  - Zero boilerplate setup                            - Modular stdlib (use ior, math)
          |                                           - Raw pointers & heap alloc
          |                                           - C Structs & C FFI (extern "C")
          |                                           - Bare-metal freestanding mode
          |                                           - Inline Assembly & RDTSC cycles
          +-------------------------+-------------------------+
                                    |
                                    v
                     +------------------------------+
                     |   Native x86_64 NASM Stream  |
                     +------------------------------+
                                    |
                                    v
                     +------------------------------+
                     |  Standalone Linux ELF Binary |
                     +------------------------------+
```

---

### 1.3 Compiler Pipeline (How Runvoid Works)

When you invoke `runvoid build` or `runvoid run`, your source code undergoes a multi-stage compilation pipeline:

```
[ Source Code (.rv) ]
         |
         v
+------------------+
| 1. Lexer         | -> Breaks source into semantic Tokens (ident, strings, numbers)
+------------------+
         |
         v
+------------------+
| 2. Parser        | -> Builds the Abstract Syntax Tree (AST) & handles imports
+------------------+
         |
         v
+------------------+
| 3. TypeChecker   | -> Validates directives, types, module rules & struct schemas
+------------------+
         |
         v
+------------------+
| 4. Optimizer     | -> Constant Folding, Dead Code Elimination (DCE)
+------------------+
         |
         v
+------------------+
| 5. Code Generator| -> Translates AST nodes into pure x86_64 NASM Assembly
+------------------+
         |
         v
+------------------+
| 6. Assembler     | -> NASM compiles .asm to ELF64 object files (.o)
+------------------+
         |
         v
+------------------+
| 7. Linker        | -> GCC or GNU ld creates final standalone executable binary
+------------------+
```

---

## 2. Quickstart: My First 5 Minutes

Here is everything you need to know to write your first programs in Runvoid.

### Step 1: Your First Program
Create a file named `hello.rv`:
```runvoid
say "Hello, brave new world!"
```
Run it instantly from your terminal:
```bash
runvoid run hello.rv
```
> **Output:**
> `Hello, brave new world!`

---

### Step 2: Remembering Values
In Runvoid, you don't declare cryptic `var` or `let`. You tell the computer to **remember** something:
```runvoid
remember player = "Alex"
remember score = 100

say "Player: {player}, Score: {score}"
```
To change a value later, assign it directly:
```runvoid
score = score + 50
say "Updated Score: {score}"
```

---

### Step 3: Asking Questions
Interact with the user using `ask`:
```runvoid
remember name = ask "What is your character's name? "
say "Welcome to the realm, {name}!"
```

---

### Step 4: Making Decisions
Use natural English words: `if`, `otherwise if`, and `otherwise`:
```runvoid
remember gold = 75

if gold >= 100 {
    say "You can purchase the legendary sword!"
} otherwise if gold >= 50 {
    say "You can purchase a sturdy iron shield."
} otherwise {
    say "You need more gold, adventurer."
}
```

---

### Step 5: Repeating Actions
Repeat a task a specific number of times, or until a condition changes:
```runvoid
# Fixed repetition:
repeat 3 times {
    say "Hip hip hooray!"
}

# With an index counter:
repeat 5 as step {
    say "Step number: {step}"
}

# While condition holds true:
remember energy = 3
while energy > 0 {
    say "Running! Energy remaining: {energy}"
    energy = energy - 1
}
```

---

### Step 6: Reusable Actions
Define functions with `action`, and return values with `give`:
```runvoid
action calculate_damage(attack, defense) {
    give attack * 2 - defense
}

remember dmg = calculate_damage(20, 5)
say "Inflicted damage: {dmg}"
```

---

## 3. Installation & Setup

### Prerequisites
Runvoid compiles to native Linux x86_64 machine code. Ensure you have the following packages installed:

```bash
# Arch Linux:
sudo pacman -S rust cargo nasm gcc libx11

# Ubuntu / Debian:
sudo apt update && sudo apt install -y rustc cargo nasm gcc libx11-dev

# Fedora:
sudo dnf install -y rust cargo nasm gcc libX11-devel
```

### Compiling from Source
```bash
git clone https://github.com/runvoid/runvoid.git
cd runvoid
cargo build --release
sudo cp target/release/runvoid /usr/local/bin/
```
Verify the installation:
```bash
runvoid --version
# runvoid 0.2.0
```

---

## 4. Command-Line Interface (CLI)

### 4.1 Running Scripts (`run`)
Compiles and executes source code directly in memory with instant feedback:
```bash
runvoid run game.rv
```

### 4.2 Building Standalone Binaries (`build`)
Produces an optimized, stripped standalone ELF executable:
```bash
runvoid build game.rv -o my_game
./my_game
```
Add `--verbose` to inspect compiler stages:
```bash
runvoid build game.rv -o my_game --verbose
```

### 4.3 Inspecting Assembly (`emit-asm`)
Inspect the generated x86_64 NASM assembly:
```bash
runvoid emit-asm game.rv
```

### 4.4 Formatting Code (`fmt`)
The built-in formatter automatically structures your indentation, spacing, and braces:
```bash
runvoid fmt game.rv         # View formatted output
runvoid fmt -w game.rv      # Write changes in-place
```

### 4.5 Scaffolding Projects (`new`)
Quickly generate boilerplate starter templates:
```bash
runvoid new game flappy.rv   # 2D Screen canvas game template
runvoid new gui dashboard.rv # Native desktop GUI window template
runvoid new script app.rv    # Conversational script template
```

### 4.6 Interactive Terminal Cheat Sheet (`cheat`)
Open a color-coded reference cheat sheet directly in your terminal anytime:
```bash
runvoid cheat
```

### 4.7 Automated Test Runner (`test`)
Execute built-in assertions and unit test blocks across your project:
```bash
runvoid test                # Run all test suites in tests/ or current dir
runvoid test tests/suite.rv # Run specific test file
```

### 4.8 Interactive REPL (`repl`)
Start an interactive read-eval-print session with preserved variable state:
```bash
runvoid repl
```

### 4.9 Hot-Reloading Watcher (`watch`)
Monitor a source file and automatically recompile and run on every save:
```bash
runvoid watch app.rv
```

---

## 5. Beginner Language Guide

### 5.1 Output & Colors (`say`, `say same`, ANSI)

- `say` prints an expression followed by a newline.
- `say same` prints an expression **without** adding a newline (staying on the same line).
- Color keywords allow instant ANSI terminal coloring without cryptic escape sequences:

```runvoid
say "Standard text"
say green "Success: All operations completed!"
say red "Error: File not found."
say yellow "Warning: Low battery."
say blue "Info: Connecting to server..."
say cyan "Notice: Check email."
say magenta "Special: Level up!"

# Combining same-line printing:
say same "Progress: "
say same green "[OK] "
say same "(100%)\n"
```

---

### 5.2 Variables & Expressions (`remember`, `{...}`)

Variables are dynamically typed by default. You can store integers, booleans, and strings:
```runvoid
remember count = 10
remember ratio = 5
remember greeting = "Welcome"
remember is_active = true
```

#### String Interpolation
Embed variables and expressions directly into double-quoted strings using `{...}`:
```runvoid
say "{greeting}! Double count is {count * 2}."
```

---

### 5.3 Natural Math, Comparisons & Logic

Runvoid supports both natural English phrases and standard programming symbols:

| Operation | Natural English | Symbolic Alternative | Example |
|---|---|---|---|
| Addition | `+` | `+` | `a + b` |
| Subtraction | `-` | `-` | `a - b` |
| Multiplication | `*` | `*` | `a * b` |
| Division | `/` | `/` | `a / b` |
| Modulo | `%` | `%` | `a % b` |
| Equality | `is` | `==` | `if score is 100` |
| Inequality | `is not` | `!=` | `if health is not 0` |
| Less Than | `<` | `<` | `if age < 18` |
| Greater Than | `>` | `>` | `if speed > 60` |
| Logical AND | `and` | `and` | `if a and b` |
| Logical OR | `or` | `or` | `if a or b` |
| Logical NOT | `not` | `not` | `if not finished` |

---

### 5.4 Control Flow (`if`, `otherwise`, `while`, `repeat`)

#### Branching:
```runvoid
remember temp = 22

if temp > 30 {
    say "It is hot outside!"
} otherwise if temp > 15 {
    say "The weather is pleasant."
} otherwise {
    say "Bring a jacket!"
}
```

#### Loops:
```runvoid
# Fixed repetition:
repeat 4 times {
    say "Processing..."
}

# With an index counter (starts at 0):
repeat 3 as i {
    say "Index: {i}"
}

# Conditional while loop:
remember n = 1
while n <= 3 {
    say "Number {n}"
    n = n + 1
}

# Loop interruption:
repeat 10 as step {
    if step is 2 {
        skip    # Continue to next iteration
    }
    if step is 5 {
        stop    # Break out of loop
    }
    say step
}
```

---

### 5.5 Conversational Word Lists

Runvoid makes working with collections as intuitive as reading a grocery list:

```runvoid
# Create a word list:
remember inventory = "Sword", "Shield", "Health Potion"

# Add items:
add "Magic Ring" to inventory

# Remove items:
remove "Shield" from inventory

# Check membership:
if inventory has "Sword" {
    say "You are armed and ready!"
}

# Count elements:
say "Total gear: {count inventory}"

# Iterate through elements:
for every item in inventory {
    say " - {item}"
}
```

---

### 5.6 User Interaction & Menus (`ask`, `choose`, `alert`)

```runvoid
# 1. Standard text input:
remember hero = ask "Name your hero: "

# 2. Hidden password input (characters are masked):
remember pass = ask hidden "Enter secret code: "

# 3. Interactive terminal arrow-key menu:
remember weapon = choose "Choose weapon:", "Iron Sword", "Elven Bow", "Fire Staff"
say "Equipped: {weapon}"

# 4. Confirmation dialog:
remember proceed = ask user "Do you wish to enter the dungeon?"
if proceed {
    say "Entering dungeon..."
}

# 5. Graphical alert box:
alert "Victory! You defeated the dragon!"
```

---

### 5.7 Terminal Audio & Speech (`beep`, `speak`)

```runvoid
# Sound the system terminal bell:
beep

# Text-to-speech output (uses system speech synthesizer):
speak "Welcome to Runvoid, adventurer!"
```

---

### 5.8 File System, Web & String Utilities

```runvoid
# File writing & reading:
write "Highscore: 9999" into "score.txt"
remember record = read "score.txt"
say record

# Directory & file operations:
create folder "saves"
copy file "score.txt" to "saves/backup.txt"
delete file "score.txt"

if file "saves/backup.txt" exists {
    say "Backup confirmed."
}

# Web requests & downloads:
download "https://runvoid.org/banner.png" into "banner.png"
remember json_data = read web "https://api.github.com"

# String transformations:
remember title = "   The Great Adventure   "
make title uppercase    # "   THE GREAT ADVENTURE   "
make title lowercase    # "   the great adventure   "
make title trim         # "the great adventure"

remember updated = replace "great" with "epic" in title

if updated starts with "the" {
    say "Starts with 'the'"
}
if updated ends with "adventure" {
    say "Ends with 'adventure'"
}

# Execution & Timers:
say "Waiting 1 second..."
wait 1

remember dice = random 1 to 20
say "Rolled a D20: {dice}"

# Run shell commands:
run "ls -la"

# Benchmark execution time in milliseconds:
measure time {
    remember total = 0
    repeat 100000 as k {
        total = total + k
    }
}
```

---

### 5.9 2D Hardware Screen Canvas & Desktop GUI

#### Hardware 2D Canvas:
Runvoid includes a built-in 2D hardware graphics canvas powered by X11:
```runvoid
screen "Arcade 2D", 640, 480 {
    draw box at 0, 0, size 640, 480, color "black"
    draw circle at 320, 240, size 50, color "cyan"
    draw line from 0, 0 to 640, 480, color "red"
    draw text "Player 1 Ready", at 240, 50, color "yellow"
}
```

#### Native Desktop GUI Windows:
Create native GUI dialogs and settings panels with zero complex frameworks:
```runvoid
window "Runvoid Preferences", 450, 320 {
    label "Configure Engine Settings:"
    checkbox "Enable Fullscreen", 1
    checkbox "High Performance Mode", 0

    button "Save Changes" {
        say "Preferences saved!"
    }
}
```

### 5.10 Key-Value Dictionaries & Maps (v0.3.0)
Define associative dictionaries using conversational pairs or JSON-style braces:
```runvoid
# Conversational declaration
remember hero = name: "Alex", hp: 100, class: "Mage"

# JSON-style declaration
remember config = { "host": "127.0.0.1", "port": 8080 }

# Bracket indexing or possessive notation:
say "Hero name: {hero['name']}"
say "Hero HP: {hero's hp}"

# Add and remove keys:
add "shield": 50 to hero
remove "class" from hero

# Membership & counts:
if hero has "shield" {
    say "Hero has shield!"
}
say "Attributes count: {count hero}"
```

### 5.11 Pattern Matching & Pipelines (v0.3.0)
Replace nested conditional trees with expressive pattern matching and clean functional pipelines:
```runvoid
remember status = 200

match status {
    when 200 -> say green "OK"
    when 404 -> say red "Not Found"
    otherwise -> say "Other Status"
}

action double(n: Int): Int {
    give n * 2
}

action inc(n: Int): Int {
    give n + 1
}

# Left-to-right functional pipeline:
remember result = 10 |> double |> inc
say "Pipelined result: {result}" # 21
```

### 5.12 Low-Level Bitwise Operators & Hardware Audio Synthesizer (v0.3.0)
Perform single-cycle bitwise operations using English keywords or standard symbols:
```runvoid
remember a = 12 bit and 10     # 8
remember b = 12 bit or 10      # 14
remember c = 12 bit xor 10     # 6
remember d = 1 shift left 4    # 16
remember e = 32 shift right 2  # 8
remember f = ~0                # -1

# Generate raw square-wave audio frequency tone (Hz, duration ms):
play synth 440, 250
```

### 5.13 Test Assertions & Test Blocks (v0.3.0)
Write first-class automated tests and assertions:
```runvoid
test "math operations" {
    verify that 2 + 2 is 4
    verify that (10 > 5)
}

test "player stats" {
    remember player = name: "Hero", hp: 100
    verify that player's hp is 100
    verify that player has "name"
}
```

---

## 6. Memory Management & Garbage Collection

### 6.1 Automatic Mark-and-Sweep GC
In standard beginner mode, all dynamically allocated strings and list objects are tracked by a lightweight, conservative **Mark-and-Sweep Garbage Collector**.
- Automatically collects unreachable objects during allocations.
- Zero manual memory management required.
- Memory safe against double-frees and dangling pointers.

### 6.2 Disabling GC (`remove garbageC`)
For applications requiring deterministic zero-pause execution, you can disable the garbage collector by placing this directive at the top of your file:
```runvoid
remove garbageC
```
When GC is removed, memory allocations bypass the GC tracking pool and use direct heap allocation.

---

## 7. Pro Systems Mode (`add Advanced`)

When maximum performance, absolute control, and minimal binary footprint are required, Runvoid transforms into an uncompromising systems programming language.

### 7.1 Directives & Setup
To activate Pro Systems Mode, declare your intent at the top of your program:
```runvoid
remove garbageC
remove Basic
add Advanced
```
These directives instruct the compiler:
1. `remove garbageC`: Strip the runtime GC and GC metadata entirely.
2. `remove Basic`: Disable dynamic type inference and permissive conveniences.
3. `add Advanced`: Enforce strict static typing, unlock raw hardware pointers, enable inline assembly, and engage high-tier optimizations.

---

### 7.2 Strict Static Typing (`remove Basic`)
In Pro Mode, every variable declaration, function parameter, and return type must declare a concrete type:
```runvoid
remember count: Int = 42
remember title: String = "Engine Core"
remember active: Bool = true

action compute(base: Int, multiplier: Int): Int {
    give base * multiplier
}
```
Supported types:
- `Int`: 64-bit signed integer.
- `String`: Null-terminated string buffer with 8-byte length prefix.
- `Bool`: Boolean flag (0 or 1).
- `Ptr`: Raw 64-bit memory address.
- `Void`: Empty return type.
- Custom Struct Names (e.g. `Vector2D`, `Node`).

---

### 7.3 Modular Standard Library (`use ior`, `math`, etc.)
Pro Mode binaries do not link unwanted code by default. Developers explicitly import only what is required:

```runvoid
use ior       # Console I/O: say, ask, user dialogs
use math      # High-speed math: sqrt, sin, cos, pow, abs (links -lm)
use sys       # System primitives: exit, getpid
use mem       # Direct memory allocations: alloc, free
use fs        # Direct file operations: read, write, copy, delete
use net       # TCP sockets: tcp_listen, tcp_accept, tcp_connect, tcp_send, tcp_recv, tcp_close
use thread    # POSIX multithreading: thread { ... }, rv_thread_join (links -lpthread)
```

Example using Math:
```runvoid
remove garbageC
remove Basic
add Advanced
use ior
use math

remember root: Int = sqrt(144)
say root   # 12
```

---

### 7.4 Dynamic C Libraries & Multi-File Imports

#### Dynamic Libraries:
Link external C libraries directly into your executable with `use lib`:
```runvoid
use lib "raylib"
use lib "sqlite3"
```
The compiler automatically passes `-lraylib` and `-lsqlite3` to the linker.

#### Multi-File Code Organization:
Organize your codebase into modules:
```runvoid
use "engine/physics.rv"
use "engine/graphics.rv"
```
The compiler recursively parses imported `.rv` files and merges their AST declarations at compile time.

---

### 7.5 Bare-Metal Freestanding Mode (`add Freestanding`)

For operating system kernels, bootloaders, embedded systems, or micro-containers where **neither `libc` nor any runtime library** is allowed:

```runvoid
remove garbageC
remove Basic
remove Linux
add Advanced
add Freestanding

# The compiler generates 'global _start' as the entry point.
# Issue Linux syscall directly:
asm {
    mov rax, 60    ; sys_exit syscall number
    xor rdi, rdi   ; status code 0
    syscall
}
```
In freestanding mode:
- The binary is linked directly with GNU `ld -s` (no GCC, no CRT start files).
- Produces an ultra-lean binary containing **pure machine instructions**.

---

### 7.6 Hardware Inline Assembly (`asm`)

Execute raw x86_64 instructions directly within your Runvoid code:
```runvoid
remove garbageC
remove Basic
add Advanced
use ior

asm {
    mov rax, 42
    imul rax, 10
}

say 420
```

---

### 7.7 CPU Clock Cycle Benchmarking (`measure cycles`)

Measure the exact hardware CPU execution time of a code block using the processor's `RDTSC` (Read Time-Stamp Counter) instruction:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior

measure cycles {
    remember x: Int = 100
    remember y: Int = 200
    remember z: Int = x * y
    say z
}
```
> **Output:**
> `20000`
> `⏱️ Executed in 130 CPU cycles`

---

### 7.8 Raw Pointers & Heap Management (`addr`, `@`, `alloc`, `free`)

Runvoid provides full pointer arithmetic and manual memory management:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior
use mem

# 1. Stack pointer addressing:
remember val: Int = 500
remember ptr: Ptr = addr val

# 2. Dereferencing read:
say @ptr        # 500

# 3. Dereferencing write:
@ptr = 750
say val         # 750

# 4. Manual heap allocation:
remember heap_buf: Ptr = alloc 64
@heap_buf = 12345
say @heap_buf   # 12345

# 5. Freeing memory:
free heap_buf
```

---

### 7.9 C-Compatible POD Structs

Define 8-byte aligned Plain Old Data (POD) structs with C ABI compatibility:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior

# Struct definition:
struct Point3D {
    x,
    y,
    z
}

# Instantiation:
remember pt: Point3D = Point3D(10, 20, 30)

# Field reading:
say pt.x    # 10
say pt.y    # 20
say pt.z    # 30

# Field mutation:
pt.z = 99
say pt.z    # 99
```

---

### 7.10 C Foreign Function Interface (FFI)

Call external C functions directly without glue code or wrappers:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior

extern "C" {
    action puts(s: String) -> Int
    action abs(n: Int) -> Int
}

puts("Hello from native C puts!")

remember negative: Int = 0 - 42
remember positive: Int = abs(negative)
say positive    # 42
```
Runvoid strings automatically pass their underlying null-terminated `char*` pointer to external C functions.

---

### 7.11 Native Multithreading & Hardware Atomics

Spawn native POSIX threads with shared memory access and hardware-locked atomic operations:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior
use thread

remember shared_counter: Int = 0

# Spawn thread:
thread {
    atomic add shared_counter, 10
}

wait 1
say shared_counter    # 10
```
`atomic add` emits the hardware `lock add [reg], rax` instruction, guaranteeing race-free concurrency across CPU cores.

---

### 7.12 Compiler Optimizations

When `add Advanced` is enabled, the compiler engages multiple optimization passes:
1. **Constant Folding:** Evaluates static expressions at compile time (`10 * 10 + 50` is emitted directly as `150`).
2. **Dead Code Elimination (DCE):** Discards statements immediately following `give`, `stop`, or `skip`.
3. **Peephole Assembly Rewriter:** Replaces expensive instructions with faster idioms (`xor eax, eax`, `inc/dec`).
4. **Link-Time Optimization (LTO) & Stripping:** Links with `nasm -O3` and `gcc -O3 -flto -s -Wl,--gc-sections` for maximum execution speed and minimum binary size.

---

## 8. Complete Keyword & Directive Reference

| Keyword / Directive | Category | Syntax / Usage | Description |
|---|---|---|---|
| `say` | I/O | `say <expr>` | Prints expression followed by newline |
| `say same` | I/O | `say same <expr>` | Prints expression without newline |
| `ask` | I/O | `ask <prompt>` | Reads line from standard input |
| `ask hidden` | I/O | `ask hidden <prompt>` | Reads password with masked input |
| `ask user` | Dialog | `ask user <prompt>` | Graphical yes/no confirmation dialog |
| `choose` | Menu | `choose <title>, <opt1>, ...` | Interactive arrow-key selection menu |
| `alert` | Dialog | `alert <message>` | Graphical alert box |
| `remember` | Variables | `remember <name> = <val>` | Declares a new variable |
| `if` | Control | `if <cond> { ... }` | Conditional branch |
| `otherwise if` | Control | `otherwise if <cond> { ... }` | Alternative conditional branch |
| `otherwise` | Control | `otherwise { ... }` | Fallback branch |
| `repeat` | Loop | `repeat <N> [as <var>] { ... }`| Fixed count loop |
| `while` | Loop | `while <cond> { ... }` | Conditional loop |
| `stop` | Loop | `stop` | Breaks out of loop |
| `skip` | Loop | `skip` | Continues to next iteration |
| `for every ... in`| List | `for every <item> in <list>` | Iterates over list elements |
| `add ... to` | List | `add <item> to <list>` | Appends item to word list |
| `remove ... from`| List | `remove <item> from <list>`| Removes item from word list |
| `count` | List | `{count <list>}` | Returns number of items in list |
| `has` | List | `if <list> has <item>` | Checks if item exists in list |
| `action` | Function | `action <name>(<args>) { ... }`| Defines a function |
| `give` | Function | `give <expr>` | Returns value from function |
| `beep` | Audio | `beep` | Emits system terminal bell |
| `speak` | Audio | `speak <text>` | Text-to-speech engine |
| `read` | File I/O | `read <path>` | Reads entire file into string |
| `write ... into`| File I/O | `write <content> into <path>` | Writes string into file |
| `create folder` | Filesystem| `create folder <dir>` | Creates a directory |
| `delete file` | Filesystem| `delete file <path>` | Deletes a file |
| `delete folder` | Filesystem| `delete folder <dir>` | Deletes a directory |
| `copy file ... to`| Filesystem| `copy file <src> to <dst>` | Copies file |
| `file ... exists`| Filesystem| `if file <path> exists` | Checks file existence |
| `download ... into`| Web | `download <url> into <path>` | Downloads URL to file |
| `read web` | Web | `read web <url>` | HTTP GET request |
| `open web` | Web | `open web <url>` | Opens URL in default browser |
| `make uppercase`| String | `make <var> uppercase` | Converts string to uppercase |
| `make lowercase`| String | `make <var> lowercase` | Converts string to lowercase |
| `make trim` | String | `make <var> trim` | Trims leading/trailing whitespace |
| `replace ... with`| String | `replace <a> with <b> in <str>`| Replaces substrings |
| `screen` | Graphics | `screen <title>, <w>, <h> { ... }` | 2D hardware graphics window |
| `window` | GUI | `window <title>, <w>, <h> { ... }` | Desktop GUI window |
| `wait` | Timers | `wait <seconds>` | Pauses execution |
| `random ... to` | Math | `random <min> to <max>` | Generates random integer |
| `measure time` | Profiling| `measure time { ... }` | Profiles time in milliseconds |
| `measure cycles`| Profiling| `measure cycles { ... }` | Profiles time in CPU clock cycles |
| `remove garbageC`| Directive| `remove garbageC` | Disables garbage collector |
| `remove Basic` | Directive| `remove Basic` | Enforces strict static typing |
| `remove Linux` | Directive| `remove Linux` | Freestanding mode (no libc) |
| `add Advanced` | Directive| `add Advanced` | Enables Pro Systems Mode |
| `add Freestanding`| Directive| `add Freestanding` | Emits `_start` entry point |
| `use` | Module | `use <module>` | Imports stdlib module or file |
| `use lib` | Library | `use lib <name>` | Links external dynamic C library |
| `asm` | Hardware | `asm { ... }` | Emits verbatim x86_64 assembly |
| `addr` | Pointer | `addr <var>` | Returns memory address of variable |
| `@` | Pointer | `@<ptr>` or `@<ptr> = <val>` | Pointer dereference read/write |
| `alloc` | Memory | `alloc <size>` | Allocates heap memory |
| `free` | Memory | `free <ptr>` | Deallocates heap memory |
| `struct` | Data | `struct <Name> { ... }` | Defines C-compatible POD struct |
| `extern "C"` | FFI | `extern "C" { action ... }` | Imports C functions |
| `thread` | Concurrency| `thread { ... }` | Spawns OS thread |
| `atomic add` | Concurrency| `atomic add <var>, <val>` | Hardware atomic addition |

---

## 9. Troubleshooting & Common Pitfalls

### Q1: Why did `{my_var}` print literally instead of showing its value?
**A:** String interpolation only occurs inside double-quoted string literals:
```runvoid
# Correct:
say "Your score is {my_var}"

# Incorrect:
say my_var    # If printing variable directly, omit quotes altogether: say my_var
```

---

### Q2: Why does `add Advanced` fail with an error?
**A:** Pro Mode requires explicitly confirming that you are opting out of beginner conveniences:
```runvoid
# Must include both removals:
remove garbageC
remove Basic
add Advanced
```

---

### Q3: Why does `say` not work in Pro Mode?
**A:** In Pro Mode, standard I/O is not linked by default to eliminate binary bloat. Add:
```runvoid
use ior
```

---

### Q4: How do I compile a completely bare-metal binary without libc?
**A:** Use freestanding directives:
```runvoid
remove garbageC
remove Basic
remove Linux
add Advanced
add Freestanding

asm {
    mov rax, 60
    xor rdi, rdi
    syscall
}
```
Runvoid links this using GNU `ld -s` directly with no C runtime or libc dependencies.

---

### Q5: How do I link with Raylib or SQLite?
**A:** Use `use lib`:
```runvoid
use lib "raylib"
use lib "sqlite3"
```
Ensure the library is installed on your Linux system (`/usr/lib/libraylib.so` or `/usr/lib/libsqlite3.so`).
