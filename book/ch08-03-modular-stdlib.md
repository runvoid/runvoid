# 8.3 The Modular Standard Library

In traditional programming environments, even an empty program often compiles into a multi-megabyte binary because the standard library bundles a monolithic runtime full of unneeded features.

In Runvoid Pro Systems Mode, the monolithic runtime is discarded. The standard library is decomposed into **isolated, zero-overhead modules**. You explicitly declare the subsystems your program requires using the `use` statement, and the compiler’s dead-code elimination ensures not a single unnecessary byte enters your binary.

---

## 1. Standard Library Modules Reference

| Directive | Purpose | Unlocked Keywords & Primitives | Linker Flags |
| :--- | :--- | :--- | :--- |
| **`use ior`** | High-performance console I/O | `say`, `ask`, terminal colors | None (Header-only) |
| **`use math`** | Hardware mathematical intrinsics | `sqrt`, `sin`, `cos`, `pow`, `abs`, `floor`, `ceil` | `-lm` |
| **`use sys`** | Low-level OS process management | `exit`, `getpid`, `get_env`, `exec_cmd` | None |
| **`use mem`** | Raw manual memory allocators | `alloc`, `free`, `realloc`, `copy_mem` | None |
| **`use fs`** | High-throughput direct file I/O | `read_bytes`, `write_bytes`, `open_file`, `close_file` | None |
| **`use net`** | Cross-platform BSD / Winsock2 networking | `tcp_listen`, `tcp_accept`, `tcp_connect`, `tcp_send`, `tcp_recv` | `-lws2_32` (Win) |
| **`use thread`** | Native OS multithreading | `thread { ... }`, `mutex_lock`, `mutex_unlock` | `-lpthread` (Linux) |

---

## 2. Granular Module Inclusion Example

Here is a lean systems service using only the I/O and process control modules:

```runvoid
remove garbageC
remove Basic
add Advanced

use ior
use sys

remember pid: Int = getpid()
say "Process initialized under PID: {pid}"

if pid <= 0 {
    say red "Critical error: Failed to obtain valid PID."
    exit(1)
}

say green "Daemon ready."
exit(0)
```

Because neither `use math`, `use net`, nor `use thread` were imported, the linker excludes those libraries entirely. The resulting executable is a razor-thin **14 KB** binary that launches in less than a single millisecond!

---

## 3. Linking External C Libraries (`use lib`)

When your systems application needs to interface with existing third-party C or C++ shared libraries—such as SQLite, Raylib, OpenSSL, or zlib—use the `use lib` directive:

```runvoid
remove garbageC
remove Basic
add Advanced

use ior
use lib "sqlite3"
use lib "ssl"
use lib "crypto"
```

### Compiler Linker Integration
When the compiler encounters `use lib "name"`, it instructs the linker to pass:
- `-lname` to GCC on Linux.
- `-lname` with MinGW-w64 on Windows.

This makes integrating external C libraries completely seamless, requiring no complex CMake or Makefile configurations.

---

## 4. Multi-File Project Architecture (`use "file.rv"`)

As your systems software grows, keeping all code in a single file becomes unmaintainable. Runvoid supports file-level modularization via `use "relative/path.rv"`:

```
my_project/
|-- main.rv
|-- network/
|   `-- socket.rv
`-- utils/
    |-- crypto.rv
    `-- logger.rv
```

### In `utils/logger.rv`:
```runvoid
action log_event(tag: String, msg: String): Void {
    say "[{tag}] {msg}"
}
```

### In `main.rv`:
```runvoid
remove garbageC
remove Basic
add Advanced

use ior
use "utils/logger.rv"

log_event("SYS", "System bootstrap sequence started.")
```

During compilation, the Runvoid compiler:
1. Recursively traces all `use "..."` file paths relative to the calling source file.
2. Resolves and deduplicates AST declarations to prevent cyclic dependency loops.
3. Inlines and type-checks the unified AST in a single compile pass.

