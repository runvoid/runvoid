# 3.3 Functions & Actions

Functions are prevalent in Runvoid code. You’ve already seen one of the most important built-in actions in the language: `say`.

## Defining Functions

In Runvoid, you define functions using the `action` keyword. Values are returned using `give`:

```runvoid
action add_numbers(x, y) {
    give x + y
}

remember total = add_numbers(15, 30)
say "15 + 30 = {total}"
```

## Functions with Parameters

Parameters allow you to pass values into functions. When a function executes, its arguments are passed using standard x86_64 System V AMD64 ABI registers (`rdi`, `rsi`, `rdx`, etc.), ensuring zero overhead:

```runvoid
action greet(user) {
    say "Hello, {user}! Welcome back."
}

greet("Alex")
greet("Sarah")
```
