# 8.3 The Modular Standard Library

To ensure minimal binary size and zero unneeded code, Pro Mode does not link a monolithic runtime. Instead, you explicitly import only the modules your program requires.

## Available Modules

| Module | Purpose | Linked Libraries |
|---|---|---|
| `use ior` | Console input/output: `say`, `ask` | Built-in |
| `use math` | High-performance math: `sqrt`, `sin`, `cos`, `pow`, `abs` | `-lm` |
| `use sys` | System primitives: `exit`, `getpid` | Built-in |
| `use mem` | Raw heap allocations: `alloc`, `free` | Built-in |
| `use fs` | Direct file operations: `read`, `write`, `copy`, `delete` | Built-in |
| `use net` | Low-level TCP sockets: `tcp_listen`, `tcp_send`, etc. | Built-in |
| `use thread` | Native POSIX threads: `thread { ... }` | `-lpthread` |

## Linking External C Libraries (<code>use lib</code>)

Link third-party C dynamic libraries directly into your executable:

```runvoid
use lib "raylib"
use lib "sqlite3"
```

The compiler passes these as `-lraylib` and `-lsqlite3` to the linker.

## Multi-File Modular Code (<code>use "file.rv"</code>)

Split large codebases across multiple files:

```runvoid
use "engine/physics.rv"
use "engine/renderer.rv"
```

The compiler recursively parses imported files and merges their Abstract Syntax Tree definitions at compile time.
