# Appendix A: Complete Keywords & Directives Reference

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
