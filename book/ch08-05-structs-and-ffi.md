# 8.5 C-Compatible Structs & C FFI

Systems programming rarely happens in an isolated vacuum. Operating system kernels, hardware drivers, cryptography suites, and graphical engines (such as Vulkan, DirectX, SQLite, and OpenSSL) expose standard **C Application Binary Interfaces (ABIs)**.

Runvoid Pro Systems Mode provides binary-level compatibility with C:
1. **C-Compatible POD Structs:** Define composite types whose physical memory layout matches C structures byte-for-byte.
2. **Native C FFI (`extern "C"`):** Call any C function with zero marshalling or wrapper overhead.

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

## 4. String Passing Convention

Runvoid strings are null-terminated in memory. When you pass a Runvoid `String` to an `extern "C"` function expecting a `const char*` (such as `puts`, `fopen`, or `printf`), the compiler automatically supplies the 64-bit memory pointer to the null-terminated byte sequence in the register `%rdi`.

There is **zero heap allocation, zero string copying, and zero marshalling overhead**.

