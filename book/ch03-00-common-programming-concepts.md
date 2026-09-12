# 3. Common Programming Concepts

This chapter covers concepts that appear in almost every programming language and explores how they work in Runvoid. Many programming languages share foundational ideas—variables, data types, function calls, and control flow. None of these concepts are entirely foreign, but Runvoid expresses them through a conversational, human-centric philosophy while guaranteeing zero-overhead compilation to native x86_64 assembly.

---

## The Conversational Grammar Model

Programming languages historically adopted mathematical notations (`let x = ...`, `int f(int x)`, `if (x) { ... }`). While concise for mathematicians, this syntax creates artificial friction for human cognition.

Runvoid models code as an active conversation between the programmer and the computing machine:

| Conversational Statement | Human Meaning | Target Machine Operation |
| :--- | :--- | :--- |
| **`remember x = 42`** | "Commit this value to memory" | Allocate 8-byte stack slot (`mov [rbp - 8], 42`) |
| **`say "Hello"`** | "Communicate this message" | Write string to `stdout` (`call print_string`) |
| **`ask "Name: "`** | "Inquire information from user" | Read buffer from `stdin` (`call read_line`) |
| **`choose "A", "B"`** | "Present interactive choices" | Render arrow-key terminal menu |
| **`repeat 5 times`** | "Perform this action iteratively" | Emit decrementing loop counter (`dec rcx; jnz`) |
| **`give result`** | "Hand back the computed value" | Store return value in `%rax` register (`ret`) |
| **`verify that ...`** | "Confirm this truth assertion" | Assert condition and report test failure if false |

---

## The Rosetta Stone: Cross-Language Syntax Matrix

To see how Runvoid compares directly with languages you may already know:

```
+-----------------------------------+-----------------------------------+
| Python                            | Runvoid                           |
+-----------------------------------+-----------------------------------+
| print(f"Score: {score}")          | say "Score: {score}"              |
| x = 10                            | remember x = 10                   |
| name = input("Enter name: ")      | remember name = ask "Enter name: "|
| for item in backpack:             | for every item in backpack { ... }|
| def square(n): return n * n       | action square(n) { give n * n }   |
| assert x == 10                    | verify that x is 10               |
+-----------------------------------+-----------------------------------+
| C / C++                           | Runvoid                           |
+-----------------------------------+-----------------------------------+
| printf("Score: %ld\n", score);    | say "Score: {score}"              |
| int64_t x = 10;                   | remember x: Int = 10              |
| for (int i = 0; i < 5; ++i)       | repeat 5 times as i { ... }       |
| if (x == 10) { ... } else { ... } | if x is 10 { ... } otherwise {...}|
+-----------------------------------+-----------------------------------+
```

---

## Chapter Roadmap

In this chapter, you will learn:

* **3.1 Variables and Mutability:** Declaring state with `remember`, reassigning values, scoping rules, stack frame memory layouts, compile-time constant folding, and comments.
* **3.2 Data Types & Expressions:** 64-bit signed integers, booleans, strings, advanced text transformations (`uppercase`, `trim`, `replace`), integer bit layouts, and error resilience.
* **3.3 Functions & Actions:** Defining reusable actions, parameter passing, returning data with `give`, pipeline operator (`|>`), recursion depth vs. iterative loops, and x86_64 calling conventions.
* **3.4 Control Flow:** Conditional branching with `if` and `otherwise`, conversational operators (`is`, `is not`), fixed loops with `repeat`, condition loops with `while`, loop control with `stop` and `skip`, and CPU branch prediction optimization.

Mastering these core concepts provides the foundation for building anything from command-line utilities to interactive games and systems daemons.
