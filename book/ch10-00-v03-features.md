# Chapter 10: Modern Language Features (v0.3.0)

Runvoid 0.3.0 introduces a major leap in language expressiveness, functional tooling, and developer productivity while maintaining zero-overhead native compilation to machine code.

## What's New in v0.3.0?

1. **First-Class Key-Value Dictionaries & Maps:**
   Define structured data effortlessly using natural syntax (`remember hero = name: "Alex", hp: 100`) or standard JSON-style syntax (`{ "name": "Alex", "hp": 100 }`). Access properties using bracket indexing `hero["name"]` or natural English possessive syntax `hero's name`.

2. **Pattern Matching (`match`):**
   Exhaustive and expressive multi-branch pattern matching replacing cumbersome conditional ladders for integers, booleans, and strings.

3. **Function Pipeline Operator (`|>`):**
   Chain transformations naturally from left to right: `data |> parse |> validate |> save`.

4. **Low-Level Bitwise Operators & Hardware Synthesizer:**
   High-performance bit manipulation (`bit and`, `bit or`, `bit xor`, `shift left`, `shift right`, `~`) mapping directly to single CPU instructions, alongside a built-in square-wave frequency sound generator (`play synth`).

5. **Built-in Test Runner, Verification & Interactive Tooling:**
   First-class test blocks (`test "name" { ... }`), assertion expressions (`verify that condition`), the `runvoid test` command, an interactive `runvoid repl`, and instant file watching with `runvoid watch`.

Let's dive deep into each feature throughout this chapter!
