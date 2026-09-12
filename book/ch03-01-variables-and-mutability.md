# 3.1 Variables and Mutability

In Runvoid, you declare new variables using the conversational keyword `remember`. This reflects the intuitive idea of asking the runtime to hold a value in memory for future access:

```runvoid
remember count = 5
say "The count is: {count}"
```

---

## Declaring and Initializing Variables

Every variable in Runvoid is bound to an initial value at declaration time:

```runvoid
remember player_name = "Aria"
remember current_level = 1
remember is_online = true
```

The Runvoid compiler automatically infers the variable's type from the right-hand expression. Behind the scenes in the compiler's code generator, each local variable is assigned an 8-byte aligned slot on the active stack frame (`[rbp - 8]`, `[rbp - 16]`, etc.).

---

## Mutating Existing Variables

Once a variable has been declared with `remember`, you can reassign its value without repeating the `remember` keyword:

```runvoid
remember score = 100
say "Initial score: {score}"

score = score + 25
say "After bonus: {score}"

score = score * 2
say "After combo multiplier: {score}"
```

> **Important Rule:** Use `remember` **only** when introducing a variable into the current scope for the first time. Re-assigning an already declared variable requires only the variable name and an assignment operator (`name = new_value`). Attempting to re-declare the same variable with `remember` within the same scope is flagged by the compiler.

---

## Variable Scoping & Shadowing

Variables in Runvoid are lexically scoped to the block in which they are declared. A block is delimited by curly braces `{ ... }`, such as those in `if` statements, loops, or functions:

```runvoid
remember total = 10

if total > 5 {
    // This variable exists only inside this if-block
    remember bonus = 50
    say "Total with bonus: {total + bonus}"
}

// Accessing bonus here would produce a compile-time error:
// say bonus
```

### Shadowing
If you declare a variable with `remember` inside an inner block with the same name as an outer variable, the inner variable **shadows** the outer one until the block terminates:

```runvoid
remember count = 10

if true {
    remember count = 99
    say "Inner count: {count}" // Prints 99
}

say "Outer count: {count}" // Prints 10
```

---

## How Variables Work Under the Hood

When you run `runvoid emit-asm`, you can see exactly how the compiler manages your variables on the hardware stack:

```runvoid
remember x = 42
remember y = 10
remember z = x + y
```

Emitted assembly:
```nasm
; remember x = 42
mov rax, 42
mov [rbp - 8], rax

; remember y = 10
mov rax, 10
mov [rbp - 16], rax

; remember z = x + y
mov rax, [rbp - 8]
add rax, [rbp - 16]
mov [rbp - 24], rax
```

Because variables are mapped directly to native CPU stack offsets (`rbp - offset`), variable access and arithmetic execute at full native microprocessor speed, with zero virtual machine dispatch overhead!

```
         High Memory Addresses
       +-------------------------+
       | Return Address (RIP)    | [rbp + 8]
       +-------------------------+
       | Saved Base Pointer(RBP) | [rbp] <--- Base Pointer (RBP)
       +-------------------------+
       | Variable `x` (8 bytes)  | [rbp - 8]  (mov [rbp - 8], 42)
       +-------------------------+
       | Variable `y` (8 bytes)  | [rbp - 16] (mov [rbp - 16], 10)
       +-------------------------+
       | Variable `z` (8 bytes)  | [rbp - 24] (mov [rbp - 24], rax)
       +-------------------------+
       | 16-Byte Stack Alignment | [rsp] <--- Stack Pointer (RSP)
       +-------------------------+
         Low Memory Addresses (Stack grows downward)
```

---

## Compile-Time Constant Folding

If an expression contains only literal numbers and arithmetic operators known at compile time, the Runvoid optimizer evaluates the result during compilation:

```runvoid
// Compile-time evaluation: 60 * 60 * 24 = 86400
remember SECONDS_PER_DAY = 60 * 60 * 24
say "Seconds in a day: {SECONDS_PER_DAY}"
```

In the emitted assembly, the multiplication instructions are eliminated entirely:
```nasm
mov rax, 86400
mov [rbp - 8], rax
```
This zero-cost constant folding ensures your code remains readable without suffering runtime calculation penalties.

---

## Common Pitfalls & Compiler Gotchas

### 1. Typo in Variable Names
If you misspell an identifier:
```runvoid
remember counter = 10
cunter = counter + 1
```
The compiler catches this and suggests the closest known variable name:
```text
error: Use of undeclared variable 'cunter'
  = help: did you mean 'counter'?
  --> app.rv:2:1
   |
 2 | cunter = counter + 1
   | ^^^^^^
```

### 2. Re-declaring with `remember` in the Same Scope
```runvoid
remember speed = 50
// Error: 'speed' already declared in this scope!
remember speed = 75
```
Fix: Simply reassign without `remember`:
```runvoid
speed = 75
```

---

## Comments

Runvoid provides flexible comment styles to annotate your code:

1. **Double slash (`//`):** Traditional C/Rust-style single line comments.
2. **Hash symbol (`#`):** Scripting/Python-style single line comments.
3. **Semicolon (`;`):** Assembly-style single line comments.

```runvoid
// Standard single-line comment
# Unix shell-style comment
; Assembly-style comment

remember timeout_seconds = 30 // Wait up to 30 seconds for connection
```


