# 8.2 Strict Static Typing

In Pro Systems Mode, all variable bindings, function arguments, and return types must explicitly state their concrete data type.

## Explicit Type Annotations

```runvoid
remove garbageC
remove Basic
add Advanced
use ior

remember count: Int = 42
remember name: String = "Kernel Core"
remember is_ready: Bool = true
remember buffer: Ptr = 0

action compute_hash(seed: Int, salt: Int): Int {
    give seed * 31 + salt
}

say compute_hash(count, 7)
```

## Available Types

- **`Int`**: 64-bit signed integer.
- **`String`**: UTF-8 string with length prefix.
- **`Bool`**: Boolean flag.
- **`Ptr`**: 64-bit raw memory pointer.
- **`Void`**: Empty return type for procedures.
- **Custom Struct Names**: Defined using `struct`.
