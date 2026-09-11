# 3.2 Data Types & Expressions

Runvoid is a statically compiled language with powerful automatic type inference. In standard mode, you do not need to decorate your code with noisy type annotations—the compiler infers the exact hardware representation of each variable and expression at compile time.

Every value in Runvoid has a concrete type. Let’s explore the primary data types and expressions supported by the language.

---

## 1. Scalar Types

A scalar type represents a single value. Runvoid provides three foundational scalar types:

### 64-bit Signed Integers (`Int`)
All numerical integer literals in Runvoid default to **64-bit two's complement signed integers** (`int64_t`), capable of storing numbers from `-9,223,372,036,854,775,808` to `9,223,372,036,854,775,807`:

```runvoid
remember count = 42
remember negative_val = -1500
remember large_number = 1000000000
```

All arithmetic operations (`+`, `-`, `*`, `/`, `%`) compile directly to single-cycle x86_64 ALU instructions (`add`, `sub`, `imul`, `idiv`).

### Booleans (`Bool`)
Runvoid booleans have two possible values: `true` and `false`:

```runvoid
remember is_running = true
remember game_over = false
```

In the compiled binary, booleans occupy 64-bit register space for optimal CPU cache alignment, where `1` represents `true` and `0` represents `false`.

### Strings (`String`)
Strings in Runvoid represent UTF-8 encoded text:

```runvoid
remember greeting = "Welcome to Runvoid!"
```

In compiled machine code, string literals are placed in the read-only data section (`.rodata` on Linux, `.rdata` on Windows) and null-terminated. Dynamic strings allocated at runtime are stored in heap memory and automatically managed.

---

## 2. String Interpolation & Manipulation

Runvoid provides first-class string interpolation. Any valid expression placed inside curly braces `{...}` within a double-quoted string is evaluated and converted to text:

```runvoid
remember width = 20
remember height = 10
say "A rectangle of {width}x{height} has an area of {width * height}."
```

### String Concatenation
You can concatenate strings using the standard `+` operator:

```runvoid
remember first = "Super"
remember second = "sonic"
remember combined = first + " " + second
say combined // Prints: Super sonic
```

### String Length
You can measure the length of any string using either the conversational prefix or property syntax:

```runvoid
remember message = "Runvoid"
say "Length: {message.length}" // Prints: 7
```

---

## 3. Arithmetic & Comparison Expressions

Runvoid supports all standard arithmetic and comparison operations, with both symbolic and natural conversational keywords:

### Arithmetic Operators
| Operation | Operator | Example | Emitted x86_64 Instruction |
| :--- | :--- | :--- | :--- |
| Addition | `+` | `a + b` | `add rax, rbx` |
| Subtraction | `-` | `a - b` | `sub rax, rbx` |
| Multiplication | `*` | `a * b` | `imul rax, rbx` |
| Division | `/` | `a / b` | `cqo; idiv rbx` |
| Modulo | `%` | `a % b` | `cqo; idiv rbx; mov rax, rdx` |

### Comparison Operators
Runvoid allows you to write either traditional mathematical symbols or natural English words:

```runvoid
remember score = 100

// Symbolic comparisons:
if score == 100 { say "Perfect!" }
if score != 0   { say "Not zero" }
if score > 50   { say "High score" }

// Conversational equivalents:
if score is 100     { say "Perfect!" }
if score is not 0 { say "Not zero" }
```

### Logical Operators
Compose complex conditions using `and`, `or`, and `not`:

```runvoid
remember health = 85
remember has_shield = true
remember is_poisoned = false

if health > 50 and has_shield {
    say green "Ready for combat!"
}

if is_poisoned or health < 20 {
    say red "Danger: Seek medical attention!"
}

if not is_poisoned {
    say "Player is healthy."
}
```

---

## 4. Compound Types: Lists and Dictionaries

Beyond scalar values, Runvoid provides powerful compound data structures:

- **Lists:** Ordered sequences of values (`[1, 2, 3, 4]`).
- **Dictionaries:** Key-value mappings (`{ "name": "Aria", "score": 950 }`).

We will explore lists and collections in Chapter 4, and dictionaries in Chapter 10!

