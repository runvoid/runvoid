# Chapter 10: Modern Language Features (Runvoid 1.3)

Runvoid 1.3 represents a watershed release in the evolution of Runvoid 1. It introduces first-class associative data structures, expressive pattern matching, functional pipelines, hardware-level bitwise operations, built-in sound synthesis, canonical project manifests (`runvoid.toml`), and a unified automated testing framework—all while maintaining zero-overhead native machine code compilation for both **Linux x86_64** and **Windows x86_64**.

---

## The Versioning Model: The Python-Style Paradigm

Runvoid adopts an explicit, long-horizon versioning scheme:

```
[ Runvoid 1 ] --------------------------------------------------> [ Runvoid 2 (Far Future) ]
    |
    +---> Version 1.0 (Foundational conversational compiler & x86_64 codegen)
    +---> Version 1.1 (Modular runtime & standard library expansions)
    +---> Version 1.2 (Pro Systems mode, pointers, C FFI, and atomics)
    +---> Version 1.3 (Dictionaries, matching, pipelines, audio, tests, runvoid.toml)
    +---> Future 1.x (Strict backward compatibility guarantees across Runvoid 1)
```

Just as Python 3 guarantees stable evolution across minor releases (3.10 -> 3.11 -> 3.12 -> 3.13), **Runvoid 1** maintains complete backward compatibility across all 1.x releases. Code written in Runvoid 1.0 continues to compile and execute without modification under Runvoid 1.3.0.

---

## Benchmark Showdown: Runvoid 1.3 vs. Scripting Runtimes

To measure the impact of native AOT compilation against interpreted and JIT scripting environments, here are real-world benchmark comparisons executed on modern x86_64 hardware:

| Benchmark Metric | Runvoid 1.3.0 (Native) | Python 3.12 | Node.js v20 (V8 JIT) |
| :--- | :--- | :--- | :--- |
| **Startup Latency** | **1.2 milliseconds** | 35 milliseconds | 42 milliseconds |
| **Resident Memory (RSS)** | **~2.8 Megabytes** | 18.5 Megabytes | 38.2 Megabytes |
| **Binary Output Size** | **16 Kilobytes** | N/A (50MB+ Runtime) | N/A (Bundle ~40MB) |
| **Fibonacci(35) CPU Time** | **0.065 seconds** | 1.840 seconds | 0.082 seconds |
| **Direct Hardware Syscalls** | **Zero Overhead (`syscall`)** | C FFI Wrapper | C++ Addon Wrapper |

Because Runvoid does not incur runtime initialization overhead, virtual machine warmup delays, or dynamic memory garbage collection pauses, it is ideally suited for command-line tools, microservices, and game loops.

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
  |  - Canonical Project Manifests (`runvoid.toml`) & Scaffolding (`init`)   |
  |  - Cross-Platform Windows x86_64 Target (`--target windows`)            |
  |  - Universal One-Step Installers & GUI Wizards (`install.sh`, setup.exe)|
  +-------------------------------------------------------------------------+
```

---

## Organization of This Chapter

* **10.1 Key-Value Dictionaries & Maps:** How associative hash maps work, conversational declaration syntax, property access via bracket indexing and possessive notation (`player's hp`), open-addressing hash table layouts, and FNV-1a hashing.
* **10.2 Pattern Matching & Pipelines:** Replacing nested conditional logic with `match` blocks, compiler jump table generation, and chaining function calls cleanly with the pipeline operator (`|>`).
* **10.3 Bitwise Operations & Audio Synthesizer:** Low-level bit masking, binary arithmetic, bitwise shifts, and programmable tone synthesis.
* **10.4 Test Runner, Assertions & Hot-Reloading Watch:** Writing isolated test cases with `test "name" { ... }`, verifying invariants with `verify that`, running CI test suites via the CLI, and developing with hot reload.
