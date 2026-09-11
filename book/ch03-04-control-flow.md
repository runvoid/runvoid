# 3.4 Control Flow: if, otherwise, while, repeat

The ability to run some code depending on whether a condition is true and to run some code repeatedly while a condition is true are basic building blocks in most programming languages. The most common constructs that let you control the flow of execution of Runvoid code are `if` expressions and loops.

## Branching: `if` and `otherwise`

An `if` block allows you to branch your code depending on conditions:

```runvoid
remember number = 6

if number % 4 is 0 {
    say "number is divisible by 4"
} otherwise if number % 3 is 0 {
    say "number is divisible by 3"
} otherwise if number % 2 is 0 {
    say "number is divisible by 2"
} otherwise {
    say "number is not divisible by 4, 3, or 2"
}
```

## Fixed Repetition: `repeat`

To run a block of code a set number of times:

```runvoid
repeat 3 times {
    say "Processing..."
}
```

You can optionally bind the current iteration index (0-indexed):

```runvoid
repeat 5 as index {
    say "Step {index}"
}
```

## Conditional Loops: `while`

```runvoid
remember count = 3
while count > 0 {
    say "{count}..."
    count = count - 1
}
say "LIFTOFF!"
```

## Loop Controls: `stop` and `skip`

- `stop`: Breaks out of the current loop immediately.
- `skip`: Skips the rest of the current iteration and begins the next.

```runvoid
repeat 10 as step {
    if step is 2 {
        skip
    }
    if step is 5 {
        stop
    }
    say "Step: {step}"
}
```
