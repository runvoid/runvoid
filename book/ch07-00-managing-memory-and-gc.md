# 7. Managing Memory & The Garbage Collector

Understanding memory management is vital for systems programming. Runvoid offers two distinct memory paradigms: an automatic Garbage Collector for beginners, and a zero-runtime manual model for systems engineers.

## Automatic Mark-and-Sweep Garbage Collector

By default, all dynamically allocated strings and word lists in Runvoid are managed by a conservative **Mark-and-Sweep Garbage Collector** built directly into the native runtime:
- Allocations are tracked in a lightweight pool.
- When memory pressure increases, the GC traverses the call stack and data references to free unused allocations.
- Beginners never need to worry about memory leaks, double-frees, or segmentation faults.

## Disabling the Garbage Collector: `remove garbageC`

For programs requiring deterministic zero-pause execution, you can completely remove the garbage collection runtime:

```runvoid
remove garbageC
```

When `remove garbageC` is declared at the top of your program:
- The entire GC tracking subsystem is stripped from the compiled binary.
- Allocations bypass GC tracking and use a fast direct heap allocator.
- Binaries become smaller and exhibit predictable execution latency.
