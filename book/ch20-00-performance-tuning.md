# 20. Systems Profiling, Cache Locality & Performance Tuning

In systems programming, code is not just an abstract algorithm—it is a physical process executed on physical silicon. High-performance software extracts every cycle of work from the CPU, minimizes cache misses, eliminates memory allocations in hot loops, and enables vectorization.

In this chapter, you will master systems optimization in Runvoid:
1. The hardware cache hierarchy: L1, L2, L3 caches, cache lines, and false sharing.
2. Data-Oriented Design (DOD): Array of Structures (AoS) vs. Structure of Arrays (SoA).
3. Profiling native binaries using Linux `perf` and generating Flamegraphs.
4. Scientific benchmarking using `hyperfine` with statistical variance analysis.
5. Zero-cost optimization patterns in Runvoid codebases.

---

## 1. Hardware Architecture: The Memory Wall

Modern CPU cores execute instructions in fractions of a nanosecond (clock speeds exceeding $4.0\text{ GHz}$). However, dynamic RAM (DRAM) access takes roughly $50 - 80\text{ nanoseconds}$.

This disparity is known as **The Memory Wall**:

```
CPU Cycle Clock (~0.25 ns):
|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|*|

Main Memory Latency (~60 ns = ~240 CPU clock cycles of idle stalling!):
[==============================================================================]
```

Whenever the CPU accesses data not present in cache (a **Cache Miss**), instruction execution stalls for hundreds of cycles.

### The 64-Byte Cache Line Rule

CPUs never fetch single bytes from RAM. Hardware memory controllers always fetch contiguous **64-byte chunks** called **Cache Lines**.

```
Physical Memory:
[ byte 0 | byte 1 | ... | byte 62 | byte 63 ]  <-- One Cache Line (64 Bytes)
```

- **Sequential Access (Spatial Locality):** Reading consecutive elements in memory means 1 cache miss fetches 8 consecutive 64-bit integers. The next 7 reads hit the L1 cache in $0.5\text{ ns}$!
- **Pointer Chasing (Linked Lists, Deep Tree Graphs):** Every pointer hop jumps to an arbitrary RAM address, triggering a fresh cache miss every time.

---

## 2. Data Layout: AoS vs. SoA

Consider a game engine with 10,000 active particles:

```
Array of Structures (AoS) - Poor Cache Locality for Physics:
[ Particle 0: x, y, z, r, g, b, alpha, lifetime ] [ Particle 1: x, y, ... ]
   ^^^^^^^^          ^^^^^^^^^^^^^^^^^^^^^^^^^^
   Needed for        Wasted cache line space during
   position update   movement calculations!

Structure of Arrays (SoA) - Optimal Spatial Locality:
xs:        [ x0, x1, x2, x3, x4, x5, x6, x7, ... ]  <-- 8 coordinates per cache line!
ys:        [ y0, y1, y2, y3, y4, y5, y6, y7, ... ]
lifetimes: [ l0, l1, l2, l3, l4, l5, l6, l7, ... ]
```

When calculating particle movement, the CPU loads only `x` and `y` coordinates. With SoA, 100% of bytes loaded into L1 cache are relevant to the calculation.

---

## 3. Profiling with Linux `perf`

Linux provides the low-level `perf` subsystem to inspect CPU Performance Monitoring Counters (PMCs).

### 1. Recording Hardware Events
Compile your Runvoid program with optimizations:
```bash
runvoid build --release engine.rv -o engine_bin
```

Profile CPU execution:
```bash
perf stat ./engine_bin
```

Sample output:
```text
 Performance counter stats for './engine_bin':

         12.45 msec task-clock                       #    0.985 CPUs utilized
             3      context-switches                 #  240.964 /sec
             0      cpu-migrations                   #    0.000 /sec
           482      page-faults                      #   38.715 K/sec
    48,129,402      cycles                           #    3.866 GHz
    92,410,211      instructions                     #    1.92  insn per cycle
     1,204,115      branches                         #   96.716 M/sec
        18,402      branch-misses                    #    1.53% of all branches
       310,400      L1-dcache-loads                  #   24.932 M/sec
         4,120      L1-dcache-load-misses            #    1.33% of all L1-dcache hits

       0.012642289 seconds time elapsed
```

Key Metrics:
- **Instructions per cycle (IPC):** Values $\ge 2.0$ indicate efficient execution; values $\le 0.8$ indicate cache stalls or branch mispredictions.
- **Branch-misses:** Ideally $< 3\%$. High branch miss rates degrade the hardware pipeline.
- **L1-dcache-load-misses:** Low values indicate excellent spatial and temporal memory locality.

---

## 4. Benchmarking with `hyperfine`

Never measure code speed with a single `time` run. Background OS scheduling, disk caches, and thermal CPU throttling skew individual runs.

Use **hyperfine**, the modern statistical benchmarking CLI:

```bash
hyperfine --warmup 3 --runs 20 './bin_unoptimized' './bin_optimized'
```

Output:
```text
Benchmark 1: ./bin_unoptimized
  Time (mean ± σ):      145.2 ms ±   2.1 ms    [User: 140.1 ms, System: 4.8 ms]
  Range (min … max):    141.8 ms … 149.6 ms    20 runs

Benchmark 2: ./bin_optimized
  Time (mean ± σ):       34.8 ms ±   0.6 ms    [User: 33.2 ms, System: 1.5 ms]
  Range (min … max):     33.9 ms …  36.2 ms    20 runs

Summary
  ./bin_optimized ran
    4.17 ± 0.10 times faster than ./bin_unoptimized
```

---

## 5. High-Performance Coding Checklist in Runvoid

1. **Pre-allocate Collections:** If you know an array will contain 5,000 items, allocate upfront rather than repeatedly resizing.
2. **Prefer Iterative Loops over Deep Recursion:** Function calls incur call-frame setup and register spilling. Unrolled `while` loops execute in unspilled CPU registers.
3. **Use Primitive Bitwise Operations for Math:**
   - Multiplications by powers of two: `n << 3` instead of `n * 8`.
   - Modulo by power of two: `n & 7` instead of `n % 8`.
4. **Leverage Pro Systems Mode for Hot Paths:** For compute-bound simulation kernels, enable `add Advanced` to compile direct unboxed register operations with zero garbage collection overhead.
5. **Keep Branch Conditions Predictable:** Sort data before filtering loops so the CPU's Branch Target Buffer (BTB) achieves $> 98\%$ prediction accuracy!
