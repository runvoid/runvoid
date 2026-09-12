# Appendix A: Complete Keywords & Directives Reference

This appendix provides an exhaustive reference of all keywords, directives, operators, and CLI commands supported by **Runvoid 1 (version 1.3.0)**.

---

## 1. Keywords & Statements Reference

| Keyword / Statement | Category | Syntax Example | Description |
| :--- | :--- | :--- | :--- |
| **`say`** | Console I/O | `say "Hello"` | Prints expression followed by newline to stdout |
| **`say same`** | Console I/O | `say same "Count: "` | Prints expression without trailing newline |
| **`say <color>`** | Console I/O | `say green "Success"` | Prints colored text (`green`, `red`, `yellow`, `cyan`, etc.) |
| **`ask`** | Console I/O | `remember name = ask "Name: "` | Prompts user and reads line from stdin |
| **`ask hidden`** | Console I/O | `remember pw = ask hidden "Key: "`| Reads password from stdin with echoing disabled |
| **`ask user`** | GUI Dialog | `remember ok = ask user "Proceed?"`| Native modal Yes/No confirmation dialog |
| **`choose`** | Menu | `choose "Pick:", "A", "B"` | Interactive terminal arrow-key menu |
| **`alert`** | GUI Dialog | `alert "Operation completed!"` | Native modal information alert dialog |
| **`remember`** | Variables | `remember score = 100` | Declares a new variable in current scope |
| **`remember <name>: <type>`**| Variables | `remember count: Int = 10` | Declares strictly typed variable (Pro Mode) |
| **`if`** | Control Flow | `if x > 0 { ... }` | Conditional branch |
| **`otherwise if`** | Control Flow | `otherwise if x < 0 { ... }` | Alternative conditional branch |
| **`otherwise`** | Control Flow | `otherwise { ... }` | Fallback branch |
| **`match`** | Control Flow | `match status { when 200 -> ... }`| Multi-branch pattern matching |
| **`when`** | Control Flow | `when 404 -> say "Not found"` | Branch arm inside `match` |
| **`repeat`** | Loops | `repeat 5 times { ... }` | Fixed-iteration loop |
| **`repeat ... as`** | Loops | `repeat 5 as i { ... }` | Fixed-iteration loop with bound index variable |
| **`while`** | Loops | `while condition { ... }` | Condition-driven loop |
| **`stop`** | Loops | `stop` | Breaks out of active loop (C `break`) |
| **`skip`** | Loops | `skip` | Skips to next loop iteration (C `continue`) |
| **`for every ... in`**| Collections | `for every item in list { ... }`| Iterates over all elements in collection |
| **`add ... to`** | Collections | `add "item" to backpack` | Appends item to list or inserts key into map |
| **`remove ... from`**| Collections | `remove "item" from backpack` | Deletes element or key from collection |
| **`count`** | Collections | `{count backpack}` | Returns element count of collection or map |
| **`has`** | Collections | `if backpack has "Torch"` | Checks membership or key existence |
| **`keys`** | Maps | `remember k = keys player` | Returns list of all keys in dictionary |
| **`values`** | Maps | `remember v = values player` | Returns list of all values in dictionary |
| **`action`** | Functions | `action greet(name) { ... }` | Defines reusable subroutine |
| **`give`** | Functions | `give result` | Returns value from function |
| **`beep`** | Audio | `beep` or `beep 440 for 200` | System alert chime or tone |
| **`play synth`** | Audio | `play synth 523, 150` | Square-wave frequency tone generator |
| **`speak`** | Audio | `speak "Task done"` | Native OS text-to-speech engine |
| **`read`** | File I/O | `remember txt = read "file.txt"`| Reads entire file into string |
| **`write ... into`** | File I/O | `write data into "file.txt"` | Writes string to file |
| **`create folder`** | Filesystem | `create folder "dist"` | Creates directory path |
| **`delete file`** | Filesystem | `delete file "temp.dat"` | Removes file from disk |
| **`delete folder`** | Filesystem | `delete folder "tmp"` | Removes directory from disk |
| **`copy file ... to`** | Filesystem | `copy file "a" to "b"` | Copies file across filesystem |
| **`file ... exists`** | Filesystem | `if file "cfg.json" exists` | Verifies file existence on disk |
| **`download ... into`**| Web | `download url into "app.zip"` | Downloads remote file via HTTP |
| **`read web`** | Web | `remember res = read web url` | Performs HTTP GET request |
| **`open web`** | Web | `open web "https://..."` | Opens URL in default desktop browser |
| **`screen`** | Graphics | `screen "Game", 800, 600 { ... }`| Accelerated 2D canvas viewport |
| **`window`** | GUI | `window "Panel", 400, 300 { ... }`| Native desktop GUI window |
| **`wait`** | Timers | `wait 1000` | Pauses thread execution (ms) |
| **`random ... to`** | Math | `random 1 to 100` | Generates cryptographically secure random integer |
| **`measure time`** | Profiling | `measure time { ... }` | Measures wall-clock time in milliseconds |
| **`measure cycles`**| Profiling | `measure cycles { ... }` | Measures execution time in CPU clock cycles (`RDTSC`) |
| **`test`** | Testing | `test "name" { ... }` | Defines automated unit test suite |
| **`verify that`** | Testing | `verify that a is b` | Asserts condition in test suite |

---

## 2. Compiler Directives Reference

| Directive | Placement | Effect |
| :--- | :--- | :--- |
| **`remove garbageC`** | Top of file | Strips GC runtime; allocates via direct OS allocator |
| **`remove Basic`** | Top of file | Enforces strict compile-time static typing |
| **`remove Linux`** | Top of file | Strips standard Linux libc runtime |
| **`add Advanced`** | Top of file | Enables raw pointers, atomics, FFI, and inline assembly |
| **`add Freestanding`**| Top of file | Emits bare-metal `_start` entry point with zero runtime |
| **`use <module>`** | Top of file | Imports stdlib module (`ior`, `math`, `sys`, `mem`, `fs`, `net`, `thread`) |
| **`use lib "<name>"`**| Top of file | Links external C dynamic library (`-lname`) |
| **`use "<file.rv>"`** | Top of file | Recursively imports another Runvoid source file |

---

## 3. Systems Mode Primitives

| Primitive | Syntax Example | Description |
| :--- | :--- | :--- |
| **`addr`** | `addr variable` | Returns 64-bit hardware memory address of variable |
| **`@` (Read)** | `remember val = @ptr` | Dereferences memory pointer to read 64-bit value |
| **`@` (Write)** | `@ptr = 500` | Stores 64-bit value directly into memory address |
| **`alloc`** | `remember p: Ptr = alloc 64` | Allocates unmanaged heap block of N bytes |
| **`free`** | `free p` | Deallocates unmanaged heap block |
| **`struct`** | `struct Point { x, y }` | Defines C-compatible 8-byte aligned POD struct |
| **`extern "C"`** | `extern "C" { action puts(...) }` | Declares external C ABI foreign function signatures |
| **`asm`** | `asm { mov rax, 42 }` | Injects verbatim x86_64 NASM instructions |
| **`thread`** | `thread { ... }` | Spawns concurrent operating system thread |
| **`atomic add`** | `atomic add counter, 1` | Emits hardware bus-locked `lock add` CPU instruction |

---

## 4. CLI Subcommands & Flags Reference

| Subcommand | Usage | Options / Flags |
| :--- | :--- | :--- |
| **`runvoid init`** | `runvoid init [app_name]` | Scaffolds multi-file project with `runvoid.toml` |
| **`runvoid run`** | `runvoid run [file.rv] [args...]` | `--target <linux\|windows>`, `--verbose` |
| **`runvoid build`** | `runvoid build [file.rv] -o <name>` | `-o <path>`, `--target <linux\|windows>`, `--verbose` |
| **`runvoid clean`** | `runvoid clean` | Removes `bin/`, `*.o`, `*.asm`, and build artifacts |
| **`runvoid emit-asm`**| `runvoid emit-asm [file.rv]` | `--target <linux\|windows>` |
| **`runvoid test`** | `runvoid test [path]` | `--target <linux\|windows>` |
| **`runvoid watch`** | `runvoid watch [file.rv]` | `--target <linux\|windows>` |
| **`runvoid fmt`** | `runvoid fmt <file.rv>` | `-w` (overwrite in-place) |
| **`runvoid repl`** | `runvoid repl` | Interactive shell with state persistence |
| **`runvoid new`** | `runvoid new <game\|gui\|script> <file>`| Project scaffolding |
| **`runvoid cheat`** | `runvoid cheat` | Terminal syntax cheatsheet |

---

## 5. Binary Operators & Precedence Table

| Precedence | Operator | Conversational Equivalent | Meaning |
| :--- | :--- | :--- | :--- |
| **1 (Highest)** | `*`, `/`, `%` | - | Multiplication, division, remainder |
| **2** | `+`, `-` | - | Addition, subtraction, string concatenation |
| **3** | `<<`, `>>` | `shift left`, `shift right` | Logical bitwise shifts |
| **4** | `&` | `bit and` | Bitwise conjunction |
| **5** | `^` | `bit xor` | Bitwise exclusive OR |
| **6** | `\|` | `bit or` | Bitwise disjunction |
| **7** | `==`, `!=`, `<`, `>`, `<=`, `>=` | `is`, `is not` | Relational equality and ordering |
| **8** | `and`, `or`, `not` | `and`, `or`, `not` | Logical boolean operations |
| **9 (Lowest)** | `\|>` | - | Forward function pipeline |

---

## 6. Standard Environment Variables

| Variable | Default Value | Description |
| :--- | :--- | :--- |
| **`RUNVOID_TARGET`** | Host OS architecture | Overrides compilation target (`linux` or `windows`) |
| **`RUNVOID_RUNTIME`**| `/usr/local/share/runvoid/runtime` | Path to `gui.c` and native runtime headers |
| **`NO_COLOR`** | Unset | Disables ANSI terminal colors when set |


