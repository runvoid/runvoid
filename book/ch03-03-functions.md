# 3.3 Functions & Actions

Functions are the primary building blocks for structuring reusable logic in Runvoid. In keeping with Runvoid’s conversational philosophy, functions are introduced with the keyword `action`, and values are returned using `give`.

---

## Defining an Action

Here is how you declare a basic action in Runvoid:

```runvoid
action greet() {
    say "Hello from inside an action!"
}

greet()
```

An action definition starts with `action`, followed by the function name, a pair of parentheses `()`, and a body enclosed in curly braces `{ ... }`. You call an action by writing its name followed by parentheses.

---

## Parameters and Arguments

Actions can accept one or more parameters. Parameters are variables declared as part of a function's signature:

```runvoid
action introduce(name, title) {
    say "Presenting {title} {name}!"
}

introduce("Arthur", "King")
introduce("Merlin", "Archmage")
```

When an action is invoked, the values passed to it are called **arguments**. Arguments are evaluated from left to right and passed into the action.

---

## Returning Values with `give`

To send data back to the caller from an action, use the `give` statement:

```runvoid
action square(n) {
    give n * n
}

remember result = square(7)
say "7 squared is: {result}" // Prints: 49
```

### Early Returns
The `give` statement terminates the function immediately, returning control and the specified value to the caller. You can use early returns for guard clauses:

```runvoid
action absolute_value(n) {
    if n < 0 {
        give 0 - n
    }
    give n
}

say absolute_value(-42) // Prints: 42
say absolute_value(15)  // Prints: 15
```

If an action reaches the end of its body without encountering an explicit `give`, it returns `0` (or `void`) automatically.

---

## Recursive Actions

Runvoid fully supports recursive functions. Because the Runvoid compiler maintains a genuine x86_64 stack frame for each function call, recursive logic executes safely and efficiently.

Here is a classic recursive implementation of the factorial algorithm:

```runvoid
action factorial(n) {
    if n <= 1 {
        give 1
    }
    give n * factorial(n - 1)
}

say "5! = {factorial(5)}"   // Prints: 120
say "10! = {factorial(10)}" // Prints: 3628800
```

Here is a recursive Fibonacci generator:

```runvoid
action fibonacci(n) {
    if n <= 0 { give 0 }
    if n == 1 { give 1 }
    give fibonacci(n - 1) + fibonacci(n - 2)
}

say "Fibonacci(8) = {fibonacci(8)}" // Prints: 21
```

---

## How Actions Execute on Hardware (The x86_64 ABI)

When you define an action in Runvoid, the compiler generates a standard x86_64 subroutine:

1. **Stack Frame Setup:**
   ```nasm
   push rbp
   mov rbp, rsp
   sub rsp, 32          ; Allocate stack space for local variables
   ```
2. **Argument Passing:**
   Runvoid utilizes the high-performance **System V AMD64 ABI**:
   - First 6 integer/pointer arguments are passed directly in registers: `%rdi`, `%rsi`, `%rdx`, `%rcx`, `%r8`, `%r9`.
   - On Windows, the compiler adapts the runtime using `__attribute__((sysv_abi))` and strict 16-byte stack alignment, ensuring uniform calling conventions across all operating systems.
3. **Return Value:**
   - The expression passed to `give` is evaluated into the `%rax` accumulator register.
4. **Epilogue:**
   ```nasm
   mov rsp, rbp
   pop rbp
   ret
   ```

Because Runvoid adheres directly to the hardware calling convention, there is no interpreter overhead or stack virtualization—an action call in Runvoid is as fast as a function call in C or Rust.

---

## Piping Functions with the Pipeline Operator (`|>`)

Runvoid 1.3 introduces the forward pipeline operator (`|>`), allowing you to chain action calls from left to right without deeply nested parentheses:

```runvoid
action double_val(x) { give x * 2 }
action add_ten(x)    { give x + 10 }
action square_val(x) { give x * x }

// Traditional nested style (reads inside-out):
remember result_nested = square_val(add_ten(double_val(5)))
say "Nested: {result_nested}"

// Elegant pipeline style (reads left-to-right):
remember result_piped = 5 |> double_val |> add_ten |> square_val
say "Piped: {result_piped}"
```

Both styles compile to the exact same high-efficiency machine instructions, but the pipeline syntax mirrors human problem-solving workflows: start with data, transform it through a series of actions, and produce the final result.

---

## Recursion Depth vs. Iterative Loops

While recursive actions are elegant for mathematical definitions and tree traversal, deep recursion can consume stack space. On x86_64, each call consumes at least 16 bytes on the stack for the return address and saved frame pointer.

If you anticipate recursion depths exceeding tens of thousands of frames, prefer an iterative loop:

```runvoid
// High-performance iterative factorial (O(1) memory):
action factorial_iterative(n) {
    remember result = 1
    remember current = 1
    while current <= n {
        result = result * current
        current = current + 1
    }
    give result
}

say "Factorial 12 = {factorial_iterative(12)}"
```

---

## Hands-On Challenge: Building a Math Action Toolkit

Create a reusable utility file `math_utils.rv` implementing essential numerical helpers:

```runvoid
// 1. Calculate base raised to integer exponent
action power(base, exp) {
    remember accumulator = 1
    repeat exp times {
        accumulator = accumulator * base
    }
    give accumulator
}

// 2. Clamp a value between min and max bounds
action clamp(val, min_val, max_val) {
    if val < min_val { give min_val }
    if val > max_val { give max_val }
    give val
}

// 3. Test prime numbers
action is_prime(n) {
    if n <= 1 { give false }
    remember d = 2
    while d * d <= n {
        if n % d == 0 {
            give false
        }
        d = d + 1
    }
    give true
}

say "2^10 = {power(2, 10)}"         // Prints: 1024
say "Clamped 150 (0-100): {clamp(150, 0, 100)}" // Prints: 100
say "Is 29 prime? {is_prime(29)}"   // Prints: 1 (true)
say "Is 30 prime? {is_prime(30)}"   // Prints: 0 (false)
```


