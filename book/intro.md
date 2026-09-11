# Introduction

> Note: This book covers **Runvoid 0.2.0**.

Welcome to **The Runvoid Programming Language**, an official book on Runvoid. Runvoid is an innovative compiled programming language designed to unite the conversational readability of natural English with the raw performance and micro-footprint of pure x86_64 machine code.

## Why Runvoid?

For decades, the programming world has been divided into two camps:

1. **High-Level Interpreted Languages (Python, Ruby, JavaScript):** They offer readable syntax and rapid prototyping, but rely on heavy runtimes, consume hundreds of megabytes of RAM, and crawl through slow virtual machines.
2. **Low-Level Systems Languages (C, C++, Rust):** They produce blazing-fast machine code and 15 KB binaries, but feature steep learning curves, complex type systems, and dense syntax that intimidates newcomers.

**Runvoid bridges this divide.** 

- Its code reads like plain English sentences: `say "Hello"`, `remember score = 100`, `repeat 5 times`, `for every item in backpack`, `if file "save.dat" exists`.
- Yet Runvoid is **not** an interpreted script: its native compiler, written in **Rust**, translates code directly into **pure x86_64 NASM assembly**, compiling into standalone, dependency-free Linux ELF executables as small as **15 KB**.
- When you require low-level systems control, simple compiler directives disable the garbage collector (`remove garbageC`), enforce strict static typing (`remove Basic`), and unlock raw pointers, hardware inline assembly, CPU cycle benchmarking, and bare-metal freestanding binaries (`add Advanced`, `add Freestanding`).

## Who This Book Is For

- **Novices and Beginners:** Anyone learning to program can immediately express ideas without struggling with obscure punctuation, compiler quirks, or environment setup nightmares.
- **Educators:** Teach real programming concepts—variables, control flow, functions, memory, threads—in an intuitive language that produces real native machine code.
- **Systems Developers:** Build ultra-fast, dependency-free CLI utilities, microservices, or bare-metal operating system modules with direct hardware control.

Let's dive in!
