# 8.7 Inline Assembly & CPU Cycle Profiling

High-performance systems programming sometimes requires instructions that cannot be expressed in high-level languages: specialized vector intrinsics (AVX2 / AVX-512), hardware AES acceleration, true hardware random numbers (`rdrand`), single-cycle bit manipulation (`popcnt`), memory fences, or raw CPU model-specific registers.

Runvoid Pro Systems Mode gives you direct access to CPU silicon through **Hardware Inline Assembly (`asm`)** and cycle-accurate performance benchmarking with the **`measure cycles`** block.

---

## 1. Hardware Inline Assembly (`asm`)

The `asm { ... }` construct allows you to write raw x86_64 NASM instructions directly inside your Runvoid actions:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior

action fast_multiply_by_10(val: Int): Int {
    remember result: Int = 0

    asm {
        mov rax, [rbp - 8]   ; Load parameter val
        lea rax, [rax + rax * 4] ; Multiply by 5 in one cycle
        shl rax, 1           ; Shift left by 1 to multiply by 2 (total 10x)
        mov [rbp - 16], rax  ; Store into result
    }

    give result
}

say "7 * 10 = {fast_multiply_by_10(7)}" // Prints: 70
```

### How the Compiler Handles Inline Assembly
- The contents of the `asm` block are injected **verbatim** into the NASM translation unit at that exact instruction position.
- No abstraction penalty or function wrapper is generated.
- The assembler validates your instructions against the x86_64 instruction set during the build phase.

### Register Preservation Rules
To avoid corrupting the calling stack frame or local variables, adhere to the standard AMD64 ABI register preservation rules:
- **Scratch / Volatile Registers:** You may freely use `%rax`, `%rcx`, `%rdx`, `%rsi`, `%rdi`, `%r8`, `%r9`, `%r10`, `%r11`.
- **Callee-Saved Registers:** If your assembly modifies `%rbx`, `%r12`, `%r13`, `%r14`, or `%r15`, you must `push` them before modification and `pop` them before exiting the block. Never modify `%rsp` or `%rbp` directly!

---

## 2. Hardware Random Number Generation (`rdrand`)

Cryptographic key generation and monte-carlo simulations require cryptographically secure random entropy. Modern x86_64 processors provide an on-chip thermal noise entropy generator via the `rdrand` instruction:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior

action hardware_random_u64(): Int {
    remember val: Int = 0
    asm {
    .retry:
        rdrand rax          ; Query on-die hardware TRNG
        jnc .retry          ; Retry if hardware CF is 0 (underflow)
        mov [rbp - 8], rax  ; Store into val
    }
    give val
}

remember secret_seed: Int = hardware_random_u64()
say "Cryptographic Hardware Seed: {secret_seed}"
```

---

## 3. Single-Cycle Bit Manipulation: `popcnt` & `lzcnt`

Bit twiddling algorithms (e.g. chess engines, bitboards, Bloom filters) frequently need to count set bits:

```runvoid
action count_set_bits(mask: Int): Int {
    remember count: Int = 0
    asm {
        mov rax, [rbp - 8]
        popcnt rcx, rax     ; Single-cycle population count
        mov [rbp - 16], rcx
    }
    give count
}

say "Set bits in 0b10110101: {count_set_bits(181)}" // Prints: 5
```

---

## 4. Hardware Memory Fences

When coordinating lock-free data structures across multiple CPU cores, you can emit hardware memory barriers:

```runvoid
asm {
    mfence  ; Serialize all memory loads and stores
}
```

This prevents the out-of-order execution pipeline of modern superscalar processors from reordering memory accesses across the barrier.

---

## 5. High-Precision Cycle Profiling: `measure cycles`

While standard profiling tools measure wall-clock milliseconds via operating system timers, micro-optimizations—such as comparing two sorting algorithms or auditing cache misses—demand cycle-accurate hardware measurements.

Runvoid provides the `measure cycles` block, powered by the CPU's native `RDTSC` (Read Time-Stamp Counter) instruction:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior

measure cycles {
    remember sum: Int = 0
    remember i: Int = 0

    while i < 1000 {
        sum = sum + i
        i = i + 1
    }
}
```

When run, the program reports the exact elapsed CPU clock cycles:

```text
⏱️  Executed in 1,240 CPU cycles
```

### How `measure cycles` Works in Assembly
Behind the scenes, the compiler generates a serialized hardware benchmark sandwich:

```nasm
    ; Serialize pipeline and read starting TSC
    cpuid
    rdtsc
    shl rdx, 32
    or rax, rdx
    mov r12, rax         ; Store start cycles in callee-saved r12

    ; --- Your Runvoid code executes here ---

    ; Read ending TSC and serialize pipeline
    rdtscp
    shl rdx, 32
    or rax, rdx
    mov r13, rax         ; Store end cycles

    ; Calculate delta
    sub r13, r12
```

1. **Pipeline Serialization (`cpuid` / `rdtscp`):** Prevents the CPU from executing instructions outside the measured block ahead of time.
2. **Nanosecond Resolution:** On a modern 4.0 GHz processor, 1 CPU cycle corresponds to **0.25 nanoseconds**! This gives you unmatched precision when fine-tuning high-performance routines.

---

## 6. Hands-On Project: Querying CPU Model via `cpuid`

Let's write a program that uses inline assembly to query the raw CPU brand vendor string directly from the processor silicon:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior

action print_cpu_vendor(): Void {
    // Buffers to hold vendor string registers (EBX, EDX, ECX)
    remember b_reg: Int = 0
    remember d_reg: Int = 0
    remember c_reg: Int = 0

    asm {
        xor eax, eax        ; CPUID function 0: Vendor String
        cpuid
        mov [rbp - 8], rbx  ; Characters 0-3 (e.g. "Genu")
        mov [rbp - 16], rdx ; Characters 4-7 (e.g. "ineI")
        mov [rbp - 24], rcx ; Characters 8-11(e.g. "ntel")
    }

    say "CPU Vendor signature registers captured successfully!"
}

print_cpu_vendor()
```

---

## 7. Micro-Optimization Challenge: Loop Unrolling Benchmark

Compare standard loop execution against 4x manual unrolling using `measure cycles`:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior

say cyan "=== Micro-Benchmark: Standard Loop vs Unrolled Loop ==="

// Test 1: Standard Loop
measure cycles {
    remember total1: Int = 0
    remember i: Int = 0
    while i < 1000 {
        total1 = total1 + i
        i = i + 1
    }
}

// Test 2: 4x Unrolled Loop (Reduces branch checks by 75%)
measure cycles {
    remember total2: Int = 0
    remember j: Int = 0
    while j < 1000 {
        total2 = total2 + j + (j + 1) + (j + 2) + (j + 3)
        j = j + 4
    }
}
```

The 4x unrolled loop eliminates hundreds of branch instructions, reducing total CPU cycle latency significantly!
