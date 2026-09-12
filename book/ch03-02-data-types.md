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

## 4. Advanced String Transformation Methods

Runvoid includes built-in conversational statements for text transformation:

### 1. Case Conversions: `make ... uppercase` & `lowercase`
Transform strings directly in-place without invoking external libraries:

```runvoid
remember headline = "runvoid 1.3 released"
make headline uppercase
say headline // Prints: RUNVOID 1.3 RELEASED

make headline lowercase
say headline // Prints: runvoid 1.3 released
```

### 2. Trimming Whitespace: `make ... trim`
Remove leading and trailing spaces, tabs, and newlines:

```runvoid
remember user_input = "   clean data   "
make user_input trim
say "Result: '{user_input}'" // Prints: Result: 'clean data'
```

### 3. String Replacement: `replace ... with ... in ...`
Substitute occurrences of substrings:

```runvoid
remember template = "Hello, {USER}!"
replace "{USER}" with "Commander" in template
say template // Prints: Hello, Commander!
```

### 4. Substring Queries: `starts with` & `ends with`
```runvoid
remember filename = "backup_archive.tar.gz"

if filename starts with "backup_" {
    say "Processing automated backup file..."
}

if filename ends with ".gz" {
    say "Compressed archive detected."
}
```

---

## 5. Integer Arithmetic & Overflow Semantics

Under the hood on x86_64, all Runvoid integers are standard 64-bit two's complement integers stored in 8-byte QWORD memory slots.

```
Bit 63                                                       Bit 0
+---+---------------------------------------------------------+
| S | Magnitudes (63 bits: 0 to 9,223,372,036,854,775,807)    |
+---+---------------------------------------------------------+
  ^
  |-- Sign bit (0 = positive, 1 = negative)
```

- **Two's Complement Arithmetic:** Addition and subtraction work uniformly across negative and positive values with identical hardware ALU circuitry (`add`, `sub`).
- **Division by Zero Protection:** The runtime traps division by zero cleanly, preventing CPU unhandled exception faults (`SIGFPE`) and emitting clear source line diagnostic messages.

---

## 6. Hands-On Challenge: Building a Text Sanitizer

Put your data type knowledge to the test! Write a program that takes a dirty input string, trims it, checks if it starts with a command prefix, and replaces keywords:

```runvoid
remember raw_command = "   !deploy staging_server   "

// 1. Sanitize
make raw_command trim

// 2. Validate prefix
if raw_command starts with "!" {
    say green "Valid bot command detected: {raw_command}"
    
    // 3. Transform targets
    replace "staging_server" with "production_cluster_1" in raw_command
    make raw_command uppercase
    say yellow "Prepared payload: {raw_command}"
} otherwise {
    say red "Invalid command: missing '!' prefix."
}
```

---

## 7. Compound Types: Lists and Dictionaries

Beyond scalar values, Runvoid provides powerful compound data structures:

- **Lists:** Ordered sequences of values (`[1, 2, 3, 4]`).
- **Dictionaries:** Key-value mappings (`{ "name": "Aria", "score": 950 }`).

We will explore lists and collections in Chapter 4, and dictionaries in Chapter 10!


