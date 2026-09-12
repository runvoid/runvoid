# 8.2 Strict Static Typing

In high-level scripting, dynamic type inference provides agility and rapid development. However, in systems programming, software reliability and predictable memory layout require uncompromising precision.

When `remove Basic` is declared, the compiler disables permissive inference and enforces **Strict Static Typing**. Every variable, function parameter, and return value must declare an explicit, concrete type known at compile time.

---

## 1. Concrete System Types

In Pro Systems Mode, all types map directly to native machine representations:

| Type | Size | Description | CPU Register / Representation |
| :--- | :--- | :--- | :--- |
| **`Int`** | 8 bytes | 64-bit two's complement signed integer | Direct 64-bit general purpose register (`%rax`, `%rcx`) |
| **`Bool`** | 8 bytes | Boolean logical flag (`true` = 1, `false` = 0) | 64-bit aligned boolean condition |
| **`String`** | 8 bytes | Pointer to immutable UTF-8 bytes | 64-bit memory pointer to `.rodata` or heap |
| **`Ptr`** | 8 bytes | Unconstrained 64-bit raw hardware address | Native 64-bit pointer (`uintptr_t`) |
| **`Void`** | 0 bytes | Unit / absence of return value | No register returned |
| **Struct Types** | Arbitrary | User-defined composite data structures | Contiguous memory block matching C ABI alignment |

---

## 2. Explicit Variable Declarations

Under strict typing, omitting a type annotation on a `remember` declaration results in a compile-time error:

```runvoid
remove garbageC
remove Basic
add Advanced

// Valid strictly typed declarations:
remember worker_count: Int = 8
remember service_name: String = "ProxyRouter"
remember debug_enabled: Bool = false
remember heap_buffer: Ptr = 0
```

If you attempt to write:
```runvoid
remember count = 10  // ERROR: In strict mode, variables must have an explicit type annotation!
```
The compiler catches the missing annotation before code generation ever begins.

---

## 3. Strictly Typed Function Signatures

Every action must explicitly state the type of every parameter and declare its return type:

```runvoid
action calculate_checksum(data: Ptr, length: Int): Int {
    remember checksum: Int = 0
    remember index: Int = 0

    while index < length {
        remember byte_val: Int = @(data + index)
        checksum = checksum + byte_val
        index = index + 1
    }

    give checksum
}
```

### Void Actions (Procedures)
If a function performs an action without returning a value, declare its return type as `: Void`:

```runvoid
action log_warning(code: Int, message: String): Void {
    say yellow "[WARN {code}] {message}"
}
```

---

## 4. Zero-Cost Performance Guarantees

Why does strict typing matter for systems development?

1. **No Runtime Tagging:** In high-level scripting languages, numbers and objects are wrapped in 16-byte "boxed" descriptors (NaN-boxing or pointer tags) to record their dynamic type. In Runvoid Pro mode, an `Int` is pure raw 64-bit data—it occupies exactly 8 bytes of stack space with zero overhead.
2. **Deterministic Calling Conventions:** Because parameter types are static, function calls compile directly into single `mov` instructions loading the appropriate registers (`rdi`, `rsi`, `rdx`) followed by a hardware `call` instruction.
3. **Early Error Detection:** Type mismatches are caught during compiler semantic analysis, completely eliminating runtime `TypeError` crashes in production servers.

```
   Boxed Scripting Value (Python / JS / Ruby):
   +-----------------------+-----------------------+
   | Type Tag (8 bytes)    | Payload / Ptr (8 bytes) |  <--- 16 bytes overhead + indirection
   +-----------------------+-----------------------+

   Runvoid Pro Systems Value (Int / Ptr / Bool):
   +-----------------------------------------------+
   | Raw 64-bit Integer / Pointer in %rax (8 bytes)|  <--- Direct ALU register access
   +-----------------------------------------------+
```

---

## 5. Hands-On Project: Strictly Typed FNV-1a Hash Generator

Let's write a strictly typed, zero-allocation implementation of the Fowler–Noll–Vo 64-bit hash algorithm:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior

action fnv1a_hash_byte(hash: Int, octet: Int): Int {
    remember FNV_PRIME: Int = 1099511628211
    remember mixed: Int = hash bit xor octet
    give mixed * FNV_PRIME
}

action hash_string_strict(input: String, length: Int): Int {
    remember FNV_OFFSET_BASIS: Int = 14695981039346656037
    remember current_hash: Int = FNV_OFFSET_BASIS
    remember i: Int = 0

    while i < length {
        remember byte_code: Int = @(input + i)
        current_hash = fnv1a_hash_byte(current_hash, byte_code)
        i = i + 1
    }

    give current_hash
}

remember message: String = "RUNVOID_SYSTEMS"
remember hash_val: Int = hash_string_strict(message, 15)
say hash_val
```

This entire algorithm compiles to tight arithmetic in CPU registers without touching memory or allocating a single heap byte!


