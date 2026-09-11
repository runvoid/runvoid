# 8.7 Inline Assembly & CPU Cycle Profiling

For ultimate performance tuning and hardware interaction, Runvoid allows inline x86_64 assembly code and direct cycle counting via the CPU's `RDTSC` instruction.

## Hardware Inline Assembly (<code>asm</code>)

Execute raw assembly instructions directly inside your Runvoid codebase:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior

asm {
    mov rax, 42
    imul rax, 10
}

say 420
```

The assembly statements are injected verbatim into the NASM code stream.

## CPU Clock Cycle Benchmarking (<code>measure cycles</code>)

While `measure time` measures wall-clock elapsed time in milliseconds, `measure cycles` uses the x86 `RDTSC` (Read Time-Stamp Counter) instruction to count the exact number of CPU clock cycles consumed:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior

measure cycles {
    remember a: Int = 10
    remember b: Int = 20
    remember c: Int = a + b
    say c
}
```

Sample output:
```text
30
⏱️  Executed in 130 CPU cycles
```
