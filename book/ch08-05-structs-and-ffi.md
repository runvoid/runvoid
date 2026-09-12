# 8.5 C-Compatible Structs & C FFI

Systems programming rarely happens in an isolated vacuum. Operating system kernels, hardware drivers, cryptography suites, and graphical engines (such as Vulkan, DirectX, SQLite, and OpenSSL) expose standard **C Application Binary Interfaces (ABIs)**.

Runvoid Pro Systems Mode provides binary-level compatibility with C:
1. **C-Compatible POD Structs:** Define composite types whose physical memory layout matches C structures byte-for-byte.
2. **Native C FFI (`extern "C"`):** Call any C function with zero marshalling or wrapper overhead.
3. **Cross-Platform OS Bindings:** Direct access to POSIX `libc` on Linux and `kernel32.dll` / `user32.dll` on Windows.
4. **C Callbacks:** Pass Runvoid function pointers to C subroutines like `qsort`.

---

## 1. C-Compatible Plain Old Data (POD) Structs

In Runvoid, you declare composite data structures using the `struct` keyword:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior

struct Vector3D {
    x,
    y,
    z
}
```

### Memory Layout & Alignment
Every field in a standard Runvoid struct occupies an 8-byte aligned word in memory:

```
Offset:   +0x00               +0x08               +0x10               +0x18
          +-------------------+-------------------+-------------------+
Field:    |      x (8B)       |      y (8B)       |      z (8B)       |
          +-------------------+-------------------+-------------------+
Total Size: 24 bytes (0x18)
```

This layout matches the C language layout:
```c
// Equivalent C definition:
typedef struct {
    int64_t x;
    int64_t y;
    int64_t z;
} Vector3D;
```

Because there is no hidden object header, vtable pointer, or garbage collection metadata, you can pass a Runvoid struct directly to C libraries without translation!

---

## 2. Instantiating and Accessing Structs

### Stack Allocation
You can instantiate a struct directly on the stack:

```runvoid
remember position: Vector3D = Vector3D(100, 250, -50)

// Accessing fields:
say "Position X: {position.x}"
say "Position Y: {position.y}"
say "Position Z: {position.z}"

// Mutating fields in-place:
position.z = 0
say "Grounded Z: {position.z}"
```

### Pointer to Struct
You can take the memory address of a struct using `addr` to pass it by reference:

```runvoid
remember pos_ptr: Ptr = addr position

action translate_x(target: Ptr, delta: Int): Void {
    // Offset 0 corresponds to field x:
    remember current_x: Int = @target
    @target = current_x + delta
}

translate_x(pos_ptr, 25)
say "Updated X: {position.x}" // Prints: 125
```

---

## 3. Foreign Function Interface (`extern "C"`)

Runvoid allows you to import external C symbols directly into your program using `extern "C"` blocks:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior

extern "C" {
    action puts(s: String) -> Int
    action abs(n: Int) -> Int
    action time(tloc: Ptr) -> Int
}

// Call standard C library functions directly:
puts("Direct greeting via libc puts()!")

remember current_epoch: Int = time(0)
say "Current Unix timestamp: {current_epoch}"

remember diff: Int = 0 - 42
remember absolute: Int = abs(diff)
say "Absolute value of -42 is: {absolute}"
```

### Type Mapping Table
When declaring C function signatures in Runvoid, use this mapping:

| C Type | Runvoid Pro Type | Hardware Representation |
| :--- | :--- | :--- |
| `int64_t`, `long long`, `ssize_t` | `Int` | 64-bit register (`%rax`, `%rdi`, etc.) |
| `const char*`, `char*` | `String` or `Ptr` | 64-bit pointer to null-terminated UTF-8 |
| `void*`, `size_t*`, opaque struct pointers | `Ptr` | 64-bit memory address |
| `bool`, `int` (as flag) | `Bool` or `Int` | 64-bit register (`1` / `0`) |
| `void` (no return) | `Void` | Absence of return value |

---

## 4. Interfacing with the Win32 API on Windows

When compiling for Windows x86_64, Runvoid binds directly into Microsoft system dynamic-link libraries:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior
use lib "user32"
use lib "kernel32"

extern "C" {
    action MessageBoxA(hwnd: Ptr, text: String, caption: String, type: Int) -> Int
    action GetTickCount64() -> Int
    action Beep(dwFreq: Int, dwDuration: Int) -> Int
}

remember start_ticks: Int = GetTickCount64()
say "System Milliseconds Since Boot: {start_ticks}"

// Play 750 Hz hardware beep for 200 ms
Beep(750, 200)

// Spawn native Windows modal dialog box (MB_OK | MB_ICONINFORMATION = 0x40):
MessageBoxA(0, "Runvoid running natively on Windows x64!", "System Alert", 0x40)
```

---

## 5. Function Pointers & C Callbacks (`qsort`)

Many C APIs require passing a callback function pointer (e.g. event listeners, custom sorting predicates). In Runvoid, the `addr` operator retrieves the memory entry address of any declared `action`:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior

extern "C" {
    action qsort(base: Ptr, num: Int, size: Int, comparator: Ptr) -> Void
}

// Comparator signature: int (*comp)(const void *, const void *)
action int_comparator(a_ptr: Ptr, b_ptr: Ptr) -> Int {
    remember a_val: Int = @a_ptr
    remember b_val: Int = @b_ptr
    if a_val < b_val { give -1 }
    if a_val > b_val { give 1 }
    give 0
}

remember numbers = [95, 12, 88, 3, 42]
qsort(addr numbers, 5, 8, addr int_comparator)

say green "Sorted numbers via native C qsort callback!"
```

---

## 6. Hands-On Project: Calling a Custom C Shared Library

Want to write a custom C algorithm and call it from Runvoid? Here is how:

### Step 1: Write the C code (`fast_crypto.c`)
```c
// fast_crypto.c
#include <stdint.h>

int64_t compute_xor_checksum(const uint8_t* buffer, int64_t len) {
    int64_t sum = 0;
    for (int64_t i = 0; i < len; ++i) {
        sum ^= buffer[i];
    }
    return sum;
}
```

Compile to a shared library:
```bash
gcc -shared -fPIC -O3 fast_crypto.c -o libfastcrypto.so
```

### Step 2: Call it from Runvoid (`crypto_app.rv`)
```runvoid
remove garbageC
remove Basic
add Advanced
use ior
use lib "fastcrypto"

extern "C" {
    action compute_xor_checksum(buf: String, len: Int) -> Int
}

remember message: String = "RUNVOID_CRYPTOGRAPHIC_PAYLOAD"
remember result: Int = compute_xor_checksum(message, 30)
say "Hardware XOR Checksum: {result}"
```

Compile and run:
```bash
runvoid run -L. crypto_app.rv
```
You get native C execution speed with conversational Runvoid syntax!
