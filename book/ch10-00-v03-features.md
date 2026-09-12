# Chapter 10: Modern Language Features (Runvoid 1.3)

Runvoid 1.3 represents a watershed release in the evolution of Runvoid 1. It introduces first-class associative data structures, expressive pattern matching, functional pipelines, hardware-level bitwise operations, built-in sound synthesis, and a unified automated testing framework—all while maintaining zero-overhead native machine code compilation for both **Linux x86_64** and **Windows x86_64**.

---

## The Vision for Runvoid 1.3

The goal of Runvoid 1.3 is to unite modern software engineering patterns with Runvoid's core design tenets:
1. **Conversational Clarity:** Code should read naturally without sacrificing precision.
2. **Deterministic Native Execution:** Every feature must compile into predictable x86_64 assembly with zero virtual machine baggage.
3. **Cross-Platform Parity:** Binaries running on Linux and Windows should have identical behavior, calling conventions, and native capabilities.

---

## Landmark Additions in Runvoid 1.3

```
  +-------------------------------------------------------------------------+
  |                        RUNVOID 1 (VERSION 1.3.0)                        |
  +-------------------------------------------------------------------------+
  |  - Key-Value Dictionaries & Natural Maps (name: "Alex", hp: 100)        |
  |  - Exhaustive Multi-Branch Pattern Matching (`match`)                   |
  |  - Functional Data Pipeline Operator (`|>`)                             |
  |  - Native Hardware Bitwise Operators (`bit and`, `bit or`, `shift left`)|
  |  - Square-Wave Sound Synthesizer (`play synth`)                         |
  |  - Built-in Unit Test Runner & Assertions (`runvoid test`, `verify that`)|
  |  - Hot-Reloading Watch Daemon (`runvoid watch`)                         |
  |  - Interactive Read-Eval-Print Loop (`runvoid repl`)                    |
  |  - Cross-Platform Windows x86_64 Target (`--target windows`)            |
  |  - Universal One-Step Installers (`install.sh`, `install.ps1`)          |
  +-------------------------------------------------------------------------+
```

---

## Organization of This Chapter

* **10.1 Key-Value Dictionaries & Maps:** How associative hash maps work, conversational declaration syntax, property access via bracket indexing and possessive notation (`player's hp`), and internal memory layouts.
* **10.2 Pattern Matching & Pipelines:** Replacing nested conditional logic with `match` blocks, compiler jump table generation, and chaining function calls cleanly with the pipeline operator (`|>`).
* **10.3 Bitwise Operations & Audio Synthesizer:** Low-level bit masking, binary arithmetic, bitwise shifts, and programmable tone synthesis.
* **10.4 Test Runner, Assertions & Hot-Reloading Watch:** Writing isolated test cases with `test "name" { ... }`, verifying invariants with `verify that`, running CI test suites via the CLI, and developing with hot reload.

