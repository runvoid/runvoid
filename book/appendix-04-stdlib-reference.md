# Appendix D: Standard Library & Built-in Action API Reference

This appendix provides a complete, exhaustive reference for all standard modules, global actions, keywords, string functions, collection utilities, and built-ins in the Runvoid 1.3 language ecosystem.

---

## 1. Global Built-in Actions & Directives

Global built-ins are available in standard mode without requiring explicit `use` declarations.

### `say <expression>`
Prints an expression or interpolated string to standard output (`stdout`), followed by a newline.
- **Color Modifiers:** `say red`, `say green`, `say yellow`, `say blue`, `say magenta`, `say cyan`, `say white`.
- **String Interpolation:** Expressions enclosed in `{}` are evaluated and formatted dynamically.
- **Example:**
  ```runvoid
  remember user = "Alice"
  say green "Welcome back, {user}!"
  say cyan "Calculation: 5 * 20 = {5 * 20}"
  ```

### `ask <prompt> into <variable>`
Displays an interactive prompt on standard output and reads a single line of input from standard input (`stdin`) into the target variable as a `String`.
- **Example:**
  ```runvoid
  ask "Enter server port: " into port_input
  say "Configuring listener on port {port_input}..."
  ```

### `remember <identifier> = <expression>`
Declares a new variable in the active lexical scope. In systems mode (`remove Basic`), explicit type annotations are required: `remember <identifier>: <Type> = <expression>`.
- **Example:**
  ```runvoid
  remember count = 42
  remember timeout_seconds: Int = 120
  ```

### `count <collection>`
Returns an `Int` representing the number of elements in a list, characters in a string, or keys in a dictionary.
- **Example:**
  ```runvoid
  remember items = [10, 20, 30]
  say "Items count: {count items}" // Output: 3
  
  remember greeting = "Hello"
  say "String length: {count greeting}" // Output: 5
  ```

### `wait <milliseconds>`
Pauses execution of the current thread for the specified duration in milliseconds using operating system high-resolution timers (`nanosleep` on POSIX, `Sleep` on Windows).
- **Example:**
  ```runvoid
  say yellow "Waiting 250 milliseconds..."
  wait 250
  say green "Resumed execution."
  ```

### `play tone <freq_hz> for <duration_ms>`
Emits a synthesized audio frequency tone through the default system sound hardware.
- **Example:**
  ```runvoid
  // Concert Pitch A4 (440 Hz) for 300 ms
  play tone 440 for 300
  ```

---

## 2. String Manipulation API

Runvoid 1.3 provides native methods on string values:

### `.length` / `count str`
Returns the integer byte count of the UTF-8 string.
```runvoid
remember s = "Antigravity"
say "Length: {s.length}" // 11
```

### `.uppercase()` & `.lowercase()`
Returns a new string with all ASCII characters converted to uppercase or lowercase.
```runvoid
remember input = "Runvoid Engine"
say input.uppercase() // "RUNVOID ENGINE"
say input.lowercase() // "runvoid engine"
```

### `.trim()`
Removes leading and trailing whitespace, carriage returns, and newlines.
```runvoid
remember raw = "   clean token   \n"
say "[{raw.trim()}]" // "[clean token]"
```

### `.replace(search, replacement)`
Replaces all occurrences of a substring with another string.
```runvoid
remember template = "Hello, {NAME}!"
remember message = template.replace("{NAME}", "Commander")
say message // "Hello, Commander!"
```

### `.split(delimiter)`
Splits a string into a list of substrings separated by the delimiter.
```runvoid
remember csv = "cpu,ram,disk,network"
remember columns = csv.split(",")
for every col in columns {
    say "Column: {col}"
}
```

### `.starts_with(prefix)` & `.ends_with(suffix)`
Returns `true` if the string begins with prefix or terminates with suffix.
```runvoid
remember file = "kernel.bin"
if file.ends_with(".bin") {
    say "Binary artifact detected."
}
```

---

## 3. List & Array API

Lists in Runvoid are dynamic, auto-growing contiguous sequences.

### `add <item> to <list>`
Appends an element to the end of the collection ($O(1)$ amortized).
```runvoid
remember tasks = ["Init", "Configure"]
add "Start Daemon" to tasks
```

### `remove <item> from <list>`
Searches and removes the first occurrence of the item from the list.
```runvoid
remove "Configure" from tasks
```

### `list[index]`
Accesses an item by zero-based integer index.
```runvoid
remember first = tasks[0]
tasks[0] = "Bootstrap"
```

### `keys dict` & `values dict`
Retrieves a list containing all keys or values of a dictionary.
```runvoid
remember config = {"host": "localhost", "port": 8080}
remember all_keys = keys config // ["host", "port"]
```

---

## 4. Standard Module: `ior` (Input / Output Runtime)

Activated via `use ior`. Provides zero-overhead raw standard stream I/O.

```runvoid
remove garbageC
remove Basic
add Advanced
use ior

// Raw character printing without newlines:
ior.print("Loading sector ")
repeat 5 times as i {
    ior.print(".")
    wait 50
}
ior.println(" DONE")
```

| Action Signature | Return Type | Description |
| :--- | :--- | :--- |
| `print(s: String)` | `Void` | Writes string directly to `stdout` without trailing newline. |
| `println(s: String)` | `Void` | Writes string followed by `\n` to `stdout`. |
| `eprintln(s: String)` | `Void` | Writes string followed by `\n` to standard error (`stderr`). |
| `read_line() -> String` | `String` | Reads an unbuffered line from `stdin`. |
| `flush_stdout()` | `Void` | Forces immediate flushes of kernel display buffers. |

---

## 5. Standard Module: `math` (High-Precision Mathematics)

Activated via `use math`. Provides hardware-accelerated math routines.

```runvoid
use math

remember hypotenuse = math.sqrt(3.0 * 3.0 + 4.0 * 4.0)
say "Hypotenuse: {hypotenuse}" // 5.0

remember clamped = math.clamp(140, 0, 100)
say "Clamped: {clamped}" // 100
```

| Action Signature | Return Type | Description |
| :--- | :--- | :--- |
| `abs(n: Int) -> Int` | `Int` | Absolute value of integer $n$. |
| `sqrt(n: Float) -> Float` | `Float` | Square root using hardware FPU instruction (`sqrtsd`). |
| `pow(base: Float, exp: Float) -> Float` | `Float` | Power calculation ($\text{base}^{\text{exp}}$). |
| `sin(rad: Float) -> Float` | `Float` | Trigonometric sine. |
| `cos(rad: Float) -> Float` | `Float` | Trigonometric cosine. |
| `floor(n: Float) -> Int` | `Int` | Floor rounding to nearest lower integer. |
| `ceil(n: Float) -> Int` | `Int` | Ceiling rounding to nearest higher integer. |
| `clamp(val: Int, min: Int, max: Int) -> Int` | `Int` | Clamps integer within boundaries $[min, max]$. |

---

## 6. Standard Module: `mem` (Manual Memory Management)

Activated via `use mem` when `remove garbageC` is declared.

```runvoid
remove garbageC
remove Basic
add Advanced
use mem

remember block: Ptr = mem.alloc(512)
mem.set(block, 0, 512) // Zero-fill memory
mem.free(block)
```

| Action Signature | Return Type | Description |
| :--- | :--- | :--- |
| `alloc(bytes: Int) -> Ptr` | `Ptr` | Allocates raw contiguous heap memory. |
| `free(ptr: Ptr)` | `Void` | Deallocates heap memory. |
| `realloc(ptr: Ptr, new_bytes: Int) -> Ptr` | `Ptr` | Resizes existing heap allocation. |
| `copy(dst: Ptr, src: Ptr, bytes: Int)` | `Void` | Hardware memory block copy (`memcpy`). |
| `set(dst: Ptr, byte_val: Int, count: Int)` | `Void` | Hardware memory fill (`memset`). |

---

## 7. Standard Module: `sys` (Operating System Integration)

Activated via `use sys`.

```runvoid
use sys

remember epoch = sys.current_time_ms()
say "System Time: {epoch} ms"

remember cores = sys.num_cores()
say "Detected {cores} logical CPU hardware cores."
```

| Action Signature | Return Type | Description |
| :--- | :--- | :--- |
| `exit(code: Int)` | `Void` | Exits process with integer return code. |
| `get_env(key: String) -> String` | `String` | Retrieves environment variable value. |
| `set_env(key: String, val: String)` | `Void` | Sets environment variable. |
| `current_time_ms() -> Int` | `Int` | Returns UNIX epoch in milliseconds. |
| `cpu_cycles() -> Int` | `Int` | Reads CPU timestamp counter (`rdtsc`). |
| `num_cores() -> Int` | `Int` | Returns count of logical CPU cores. |

---

## 8. Standard Module: `thread` (Concurrency & Multi-Core Primitives)

Activated via `use thread`.

| Action Signature | Return Type | Description |
| :--- | :--- | :--- |
| `spawn(action_ptr: Ptr) -> Ptr` | `Ptr` | Spawns a new native OS thread. |
| `join(thread_handle: Ptr)` | `Void` | Blocks until worker thread completes. |
| `yield()` | `Void` | Relinquishes remaining CPU slice. |
| `atomic_add(ptr: Ptr, delta: Int) -> Int` | `Int` | Hardware lock-free addition (`lock xadd`). |
| `atomic_cas(ptr: Ptr, exp: Int, new: Int) -> Bool` | `Bool` | Atomic Compare-And-Swap (`lock cmpxchg`). |

---

## 9. 2D Graphics Canvas Subsystem

Activated by calling `open window <title> size <w> by <h>`.

| Primitive Signature | Description |
| :--- | :--- |
| `clear canvas <color>` | Fills backbuffer with solid background color. |
| `draw rect at <x>, <y> size <w> by <h> color <c>` | Renders a solid rectangle. |
| `draw circle at <x>, <y> size <radius> color <c>` | Renders a filled circle. |
| `draw line from <x1>, <y1> to <x2>, <y2> color <c>` | Renders a 1-pixel line. |
| `draw text <s> at <x>, <y> color <c>` | Renders anti-aliased bitmap text. |
| `render canvas` | Flips backbuffer to screen (vsync-synchronized). |
