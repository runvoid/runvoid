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

