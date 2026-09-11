# 3.1 Variables and Mutability

In Runvoid, you declare new variables using the `remember` keyword. This conversational keyword reflects the intuitive idea of asking the computer to keep a value in mind for future use:

```runvoid
remember count = 5
say "The count is: {count}"
```

## Changing Variable Values

Once a variable has been declared with `remember`, you can update its value by simply assigning to it:

```runvoid
remember health = 100
say "Initial health: {health}"

health = health - 20
say "Health after taking damage: {health}"
```

> **Note:** You only use `remember` when first introducing a variable. When modifying an existing variable, write `name = new_value`.

## Comments

Runvoid supports both C-style `//` comments and scripting-style `#` comments:

```runvoid
// This is a line comment
# This is also a line comment

remember speed = 60 // kilometers per hour
```
