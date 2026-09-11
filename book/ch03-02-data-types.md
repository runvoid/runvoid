# 3.2 Data Types & Expressions

Runvoid is a statically compiled language with automatic type inference in beginner mode. You do not need to annotate types; the compiler determines them for you.

## Scalar Types

Runvoid includes three primary scalar types:
1. **Integers (`Int`):** 64-bit signed integers (`-9223372036854775808` to `9223372036854775807`).
2. **Booleans (`Bool`):** `true` or `false`.
3. **Strings (`String`):** UTF-8 compatible null-terminated strings with an 8-byte length prefix.

```runvoid
remember count = 42
remember title = "Engine Core"
remember is_active = true
```

## String Interpolation

Embed any valid expression within double quotes using curly braces `{...}`:

```runvoid
remember a = 10
remember b = 25
say "The sum of {a} and {b} is {a + b}!"
```

## Conversational Comparisons & Logic

Runvoid accepts both natural English words and traditional programming symbols:

- `is` or `==`: Checks equality (`if score is 100`)
- `is not` or `!=`: Checks inequality (`if health is not 0`)
- `and`: Logical conjunction (`if ready and valid`)
- `or`: Logical disjunction (`if admin or owner`)
- `not`: Logical negation (`if not completed`)
