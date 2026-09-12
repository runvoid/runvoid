# 3.4 Control Flow: if, otherwise, while, repeat

The ability to run specific blocks of code depending on runtime conditions and to repeat instructions until a goal is met are essential building blocks of any software system.

Runvoid provides rich, readable control flow constructs that eliminate syntactic noise while compiling into optimized x86_64 conditional jumps and loop structures.

---

## 1. Branching with `if` and `otherwise`

An `if` statement allows your program to branch dynamically based on conditions:

```runvoid
remember temperature = 28

if temperature > 30 {
    say yellow "It's hot outside! Stay hydrated."
} otherwise if temperature < 10 {
    say cyan "It's chilly! Wear a jacket."
} otherwise {
    say green "The weather is pleasant."
}
```

### Understanding the Branching Flow
1. Runvoid evaluates the condition expression immediately following `if`.
2. If the condition evaluates to `true` (or non-zero), the statements inside the first `{ ... }` block execute, and the remaining branches are skipped.
3. If the condition is `false`, execution falls through to check the first `otherwise if` condition.
4. If none of the conditions evaluate to `true`, the `otherwise` block executes as a fallback.

### Conversational Equality Checkers
You can use `is` and `is not` in place of `==` and `!=` for enhanced human readability:

```runvoid
remember status_code = 404

if status_code is 200 {
    say green "OK: Resource found."
} otherwise if status_code is 404 {
    say red "Not Found: The requested resource does not exist."
} otherwise {
    say yellow "Unhandled status code: {status_code}"
}
```

---

## 2. Fixed Repetition: `repeat`

When you know in advance exactly how many times a block of code needs to execute, use the `repeat` statement. This eliminates manual index initialization and increment boilerplate:

```runvoid
repeat 3 times {
    say "Pinging server..."
}
```

### Accessing the Iteration Index
If your loop body needs access to the current loop counter, bind it with `as <identifier>`:

```runvoid
repeat 5 as step {
    say "Processing batch chunk #{step}"
}
```

*Note: The loop counter starts at index `0` and increments up to `N - 1`.*

---

## 3. Condition-Driven Loops: `while`

When the number of iterations cannot be known beforehand (for example, reading data until EOF, waiting for a socket, or simulating game frames), use `while`:

```runvoid
remember countdown = 5

while countdown > 0 {
    say "{countdown}..."
    countdown = countdown - 1
}

say green "LIFTOFF!"
```

The condition is evaluated before every iteration. If the condition is false upon reaching the loop for the first time, the body never executes.

---

## 4. Loop Control Statements: `stop` and `skip`

Runvoid provides two imperative statements to alter loop execution flow from inside the body:

- **`stop`**: Immediately terminates the current loop and transfers control to the statement following the loop's closing brace (identical to `break` in C and Rust).
- **`skip`**: Immediately terminates the current iteration and jumps directly to the evaluation of the next iteration (identical to `continue` in C and Rust).

### Example: Combining `stop` and `skip`

```runvoid
repeat 10 as number {
    // Skip odd numbers:
    if number % 2 != 0 {
        skip
    }

    // Stop early once we reach 8:
    if number == 8 {
        say "Reached limit: {number}. Stopping loop."
        stop
    }

    say "Even number: {number}"
}
```

Output:
```text
Even number: 0
Even number: 2
Even number: 4
Even number: 6
Reached limit: 8. Stopping loop.
```

---

## 5. Under the Hood: Emitted Assembly Branches

In assembly language, control flow is implemented using comparisons (`cmp` or `test`) and conditional jump instructions (`je`, `jne`, `jl`, `jg`).

Consider this Runvoid statement:
```runvoid
if count > 10 {
    say "High"
} otherwise {
    say "Low"
}
```

The Runvoid compiler produces the following clean NASM sequence:

```nasm
    ; Evaluate condition: count > 10
    mov rax, [rbp - 8]
    cmp rax, 10
    jle .L_otherwise_0

    ; If true:
    lea rdi, [str_high]
    call print_string
    jmp .L_end_0

.L_otherwise_0:
    ; If false:
    lea rdi, [str_low]
    call print_string

.L_end_0:
```

Because branches map directly to hardware jump targets, Runvoid loops and conditionals benefit from modern CPU branch predictors with zero intermediate interpretation cost.

---

## CPU Branch Prediction & Optimization Insights

Modern x86_64 microprocessors feature speculative execution engines and Branch Target Buffers (BTBs). When a branch (`jle`, `jg`) is encountered:
1. The CPU guesses whether the branch is taken based on execution history.
2. If the guess is correct, execution continues with zero stall cycles.
3. If mispredicted, the CPU pipeline is flushed, causing a 10–20 cycle latency hit.

**Optimization Tip:** Order your `if / otherwise if` branches so that the most frequent condition comes first! This keeps the primary branch predictor biased favorably.

---

## Hands-On Challenges

### Challenge 1: The Canonical FizzBuzz
Iterate from 1 to 30:
- If divisible by 3 and 5, print `"FizzBuzz"` in magenta.
- If divisible by 3, print `"Fizz"` in yellow.
- If divisible by 5, print `"Buzz"` in cyan.
- Otherwise, print the number.

```runvoid
remember i = 1
while i <= 30 {
    if (i % 3 == 0) and (i % 5 == 0) {
        say magenta "FizzBuzz"
    } otherwise if i % 3 == 0 {
        say yellow "Fizz"
    } otherwise if i % 5 == 0 {
        say cyan "Buzz"
    } otherwise {
        say "{i}"
    }
    i = i + 1
}
```

### Challenge 2: Prime Number Finder with `while` and `skip`
Find all primes between 2 and 50:

```runvoid
say cyan "=== Prime Numbers Under 50 ==="

remember candidate = 2
while candidate < 50 {
    remember is_p = true
    remember divisor = 2
    
    while divisor * divisor <= candidate {
        if candidate % divisor == 0 {
            is_p = false
            stop
        }
        divisor = divisor + 1
    }

    if is_p {
        say green "Prime: {candidate}"
    }
    
    candidate = candidate + 1
}
```


