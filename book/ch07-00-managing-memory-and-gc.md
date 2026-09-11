# 7. Managing Memory & The Garbage Collector

Memory management is the cornerstone of systems engineering. In languages like Python, Java, and Go, developers are separated from the metal by heavy, stop-the-world garbage collectors that consume significant RAM. In C and C++, developers must manually manage every byte with `malloc` and `free`, inviting catastrophic bugs like use-after-free, double-frees, and buffer overflows.

Runvoid provides the best of both worlds:
1. **Application Mode:** A conservative, lightweight **Mark-and-Sweep Garbage Collector** built directly into the native runtime that prevents memory leaks without developer intervention.
2. **Pro Systems Mode:** The ability to strip the garbage collector entirely with a single directive (`remove garbageC`), granting total deterministic control over raw memory.

---

## 1. Stack vs. Heap in Runvoid

To understand memory management, you must understand the distinction between the hardware stack and the operating system heap.

```
       +------------------------------------+
       |          HARDWARE STACK            |
       |  - Managed via %rsp / %rbp         |
       |  - Ultra-fast (single instruction) |
       |  - Stores: integers, booleans,     |
       |    pointers, function frames       |
       +------------------------------------+
                         |
                         | Points to dynamic buffers
                         v
       +------------------------------------+
       |              HEAP                  |
       |  - Managed by Runtime Allocator    |
       |  - Stores: dynamic strings, lists, |
       |    dictionaries, user structs      |
       |  - Cleaned by GC or manual `free`  |
       +------------------------------------+
```

### The Stack
When you declare scalar variables (`remember count = 10`, `remember active = true`), the Runvoid compiler allocates slots directly in the CPU stack frame (`[rbp - 8]`). When the enclosing function finishes and executes `ret`, the stack pointer `%rsp` is restored in a single CPU clock cycle, instantly reclaiming the memory.

### The Heap
When you allocate dynamically sized data—such as concatenated strings, growing lists, or dictionaries—their size cannot always be known at compile time. The runtime requests memory from the OS kernel using `mmap`/`brk` on Linux or `HeapAlloc` on Windows.

---

## 2. The Conservative Mark-and-Sweep GC

In default mode, Runvoid includes an embedded, conservative Mark-and-Sweep collector:

### How It Works:
1. **Allocation Tracking:** Every time a string or collection is dynamically allocated, its memory block header is recorded in an active allocation table.
2. **Conservative Stack Scanning (Mark Phase):** When an allocation threshold is reached, the GC pauses briefly to inspect the active CPU registers (`%rax`, `%rbx`, `%rcx`, `%rdx`, `%rsi`, `%rdi`, `%r8`–`%r15`) and the stack memory between `%rsp` and the base of the call stack. Any pointer-like value referencing a managed heap block marks that block as **live**.
3. **Sweep Phase:** The collector iterates through the allocation table. Any blocks that were not marked during the scan are unreferenced garbage: their memory is freed immediately back to the OS or allocator pool.

Because the collector is conservative and native (written in optimized C and assembly), GC pause times are typically **under 50 microseconds**, several orders of magnitude faster than the Java or Python runtimes.

---

## 3. Disabling the Garbage Collector: `remove garbageC`

When writing real-time audio synthesizers, embedded operating system drivers, or high-throughput network proxies, even a 50-microsecond pause can cause buffer underruns or latency spikes.

Runvoid allows you to eliminate the garbage collector completely with a single top-level directive:

```runvoid
remove garbageC

action compute_hash(data) {
    // Allocations here bypass the GC tracker
    say "Processing data in zero-pause mode..."
}
```

### What Happens When You Declare `remove garbageC`?
1. **Zero Runtime Overhead:** The compiler instructs the linker to eliminate the garbage collection scan loops, allocation headers, and thread-suspension machinery.
2. **Direct System Allocations:** Memory allocations bypass the tracking table and call the operating system's raw memory allocator directly.
3. **Smaller Binaries:** Stripping the GC machinery shrinks the compiled binary footprint by an additional 12–18 KB.
4. **Deterministic Latency:** Your code executes with predictable instruction timings, critical for game engines and hard real-time systems.

---

## 4. Manual Memory in Systems Mode

Once you have removed the garbage collector, how do you handle dynamic memory? 

In Chapter 8, we will explore **Pro Systems Mode**, where you can use raw pointers, allocate explicit memory blocks with `allocate`, inspect byte addresses, and explicitly reclaim memory with `free`—delivering the absolute control of C with the safety and readability of Runvoid!

