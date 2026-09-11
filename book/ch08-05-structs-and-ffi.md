# 8.5 C-Compatible Structs & C FFI

Pro Systems Mode allows defining C-compatible composite data types and calling native C libraries with zero overhead.

## C-Compatible POD Structs

Define 8-byte aligned Plain Old Data (POD) structures matching standard C struct memory layouts:

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

# Instantiate struct:
remember vec: Vector3D = Vector3D(10, 20, 30)

# Field access:
say vec.x   // 10
say vec.y   // 20
say vec.z   // 30

# Field mutation:
vec.x = 99
say vec.x   // 99
```

## Native C Foreign Function Interface (FFI)

Call external C functions directly from libc or linked libraries using `extern "C"` blocks:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior

extern "C" {
    action puts(s: String) -> Int
    action abs(n: Int) -> Int
}

puts("Hello from native C puts via FFI!")

remember diff: Int = 0 - 55
remember positive: Int = abs(diff)
say positive   // 55
```

> **Note on Strings:** Runvoid strings automatically pass their internal null-terminated C string pointer (`char*`) to extern functions.
