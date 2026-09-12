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

## 2. Virtual Memory, Paging & The TLB

Modern operating systems execute user applications inside isolated **Virtual Address Spaces**:

```
Virtual Address Space (48-bit Canonical on x86_64):
0x0000000000000000 to 0x00007FFFFFFFFFFF (User Space, 128 TB)
0xFFFF800000000000 to 0xFFFFFFFFFFFFFFFF (Kernel Space, 128 TB)
```

1. **Paging:** Memory is divided into fixed 4 KB chunks called **Pages**.
2. **Page Tables (4-Level or 5-Level Paging):** The CPU's Memory Management Unit (MMU) translates virtual addresses into physical RAM addresses using page tables referenced by the `%cr3` control register.
3. **Translation Lookaside Buffer (TLB):** A hardware cache of recent virtual-to-physical address translations. Memory access that hits the TLB takes $< 1\text{ ns}$; a TLB miss triggers a multi-level page table walk taking $10 - 30\text{ ns}$.
4. **Page Fault (`#PF` Exception 14):** Occurs when code accesses a virtual page that has not yet been committed to physical RAM. The OS kernel handles the page fault, allocates physical memory, updates page tables, and resumes the application.

---

## 3. The Conservative Mark-and-Sweep GC

In default mode, Runvoid includes an embedded, conservative Mark-and-Sweep collector:

### How It Works:
1. **Allocation Tracking:** Every time a string or collection is dynamically allocated, its memory block header is recorded in an active allocation table.
2. **Conservative Stack Scanning (Mark Phase):** When an allocation threshold is reached, the GC pauses briefly to inspect the active CPU registers (`%rax`, `%rbx`, `%rcx`, `%rdx`, `%rsi`, `%rdi`, `%r8`–`%r15`) and the stack memory between `%rsp` and the base of the call stack. Any pointer-like value referencing a managed heap block marks that block as **live**.
3. **Sweep Phase:** The collector iterates through the allocation table. Any blocks that were not marked during the scan are unreferenced garbage: their memory is freed immediately back to the OS or allocator pool.

Because the collector is conservative and native (written in optimized C and assembly), GC pause times are typically **under 50 microseconds**, several orders of magnitude faster than the Java or Python runtimes.

---

## 4. Disabling the Garbage Collector: `remove garbageC`

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

## 5. The Tri-Color Marking Abstraction

To visualize how the Mark-and-Sweep engine reasons about object graphs during the mark phase, computer science uses the **Tri-Color Marking Model**:

```
[ WHITE OBJECTS ]               [ GREY OBJECTS ]               [ BLACK OBJECTS ]
Unvisited candidates        Discovered, but children       Fully visited & verified live;
(garbaged if still white     not yet scanned.              will NOT be freed during sweep.
at the end of mark phase)
       O                                O                              O
      / \                              / \                            / \
     O   O                            O   O                          O   O
```

1. **White Set:** At the start of a collection cycle, all allocated heap objects are colored White.
2. **Grey Set:** All objects directly reachable from CPU registers and active stack frames are moved to the Grey set.
3. **Black Set:** The GC takes an object from the Grey set, scans its internal pointers, moves referenced White objects to Grey, and promotes itself to Black.
4. **Sweep Phase:** Once the Grey set is empty, all remaining White objects are unreferenced garbage and freed. All Black objects are reset to White for the next cycle.

---

## 6. Memory Fragmentation & Custom Slab Allocators

When an application repeatedly allocates and frees objects of varying sizes, the heap suffers from **External Memory Fragmentation**: memory contains plenty of total free bytes, but no single contiguous block large enough to satisfy new allocations.

### Custom Fixed-Size Slab Allocator in Runvoid:
To achieve zero fragmentation, high-frequency systems pre-allocate contiguous chunks (slabs) of uniform size:

```runvoid
say cyan "=== FIXED-SIZE SLAB ALLOCATOR ==="

remember SLAB_COUNT = 64
remember SLAB_SIZE = 128 // 128 bytes per item

remember free_slots = []
repeat SLAB_COUNT as slot_idx {
    add slot_idx to free_slots
}

action allocate_slab() {
    if count free_slots > 0 {
        remember slot = free_slots[count free_slots - 1]
        remove slot from free_slots
        say green "Allocated slab slot #{slot} (Zero fragmentation)"
        give slot
    }
    say red "Out of slab memory!"
    give -1
}

action free_slab(slot) {
    add slot to free_slots
    say yellow "Returned slot #{slot} to slab pool."
}

remember s1 = allocate_slab()
remember s2 = allocate_slab()
free_slab(s1)
```

---

## 7. Hands-On Experiment: Benchmarking Memory Allocation

Let's write a benchmarking program to measure allocation throughput and observe zero-pause determinism:

```runvoid
say cyan "=== RUNVOID MEMORY BENCHMARK ==="

measure time {
    remember count = 0
    remember list = []
    
    repeat 10000 times {
        add "Payload-Item-{count}" to list
        count = count + 1
    }

    say green "Successfully allocated {count} dynamic strings!"
    say "List capacity managed by native heap."
}
```

When run:
```bash
$ runvoid run mem_bench.rv
=== RUNVOID MEMORY BENCHMARK ===
Successfully allocated 10000 dynamic strings!
List capacity managed by native heap.
[Execution Time: 4.15 ms]
```

Over 10,000 strings allocated, tracked, and managed in just 4.15 milliseconds!

---

## 8. Manual Memory in Systems Mode

Once you have removed the garbage collector, how do you handle dynamic memory? 

In Chapter 8, we will explore **Pro Systems Mode**, where you can use raw pointers, allocate explicit memory blocks with `alloc`, inspect byte addresses, and explicitly reclaim memory with `free`—delivering the absolute control of C with the safety and readability of Runvoid!
