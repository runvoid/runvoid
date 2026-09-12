# 15. Concurrency, Atomics & Multi-Threading Deep Dive

Modern processors no longer increase single-core clock speeds exponentially; instead, they scale performance by adding CPU cores. Harnessing this hardware power requires writing concurrent code that distributes work without suffering from race conditions, deadlocks, or cache thrashing.

This chapter explores high-performance concurrency in Runvoid Pro Systems Mode, from hardware cache line dynamics to lock-free atomic algorithms.

---

## 1. Hardware Architecture: The Multi-Core CPU

To write fast multithreaded software, you must understand how modern multi-core processors interact with RAM:

```
                      +-----------------------------+
                      |         MAIN RAM            |
                      +-----------------------------+
                                     |
                      +-----------------------------+
                      |    Shared L3 Cache (32MB)   |
                      +-----------------------------+
                                     |
              +----------------------+----------------------+
              |                                             |
              v                                             v
     +-----------------+                           +-----------------+
     | Core 0          |                           | Core 1          |
     | - L2 Cache (1MB)|                           | - L2 Cache (1MB)|
     | - L1 Cache (32K)|                           | - L1 Cache (32K)|
     | - Registers     |                           | - Registers     |
     +-----------------+                           +-----------------+
```

### Cache Latency Comparison:
- **CPU Registers (`rax`, `rbx`):** 0 cycles (Instantaneous).
- **L1 Cache (32 KB per core):** ~4–5 cycles (~1 nanosecond).
- **L2 Cache (1 MB per core):** ~12–14 cycles (~3 nanoseconds).
- **L3 Cache (Shared 32 MB):** ~40–50 cycles (~10 nanoseconds).
- **Main RAM:** ~200+ cycles (~50–80 nanoseconds).

**Takeaway:** Avoid going to main RAM whenever possible. Keep active thread state in local cache lines!

---

## 2. The Danger of False Sharing

A **cache line** on modern x86_64 processors is **64 bytes** wide. When Core 0 writes to a variable in memory, the hardware must invalidate the entire 64-byte cache line across all other CPU cores, even if Core 1 is working on a completely different variable that happens to be in that same 64-byte block!

This phenomenon is called **False Sharing**, and it can degrade performance by 10x or more.

### How to Prevent False Sharing:
Ensure variables updated concurrently by different threads are aligned and separated by at least 64 bytes.

---

## 3. Lock-Free Atomic Primitives

Runvoid Pro mode provides direct hardware-bus locking via `atomic`:

### 1. `atomic add`
```runvoid
atomic add shared_counter, 1
```
Emits:
```nasm
mov rax, 1
lock add qword [rbp - 8], rax
```
The `lock` prefix locks the cache line, guaranteeing that no other core can read or write during the operation.

### 2. Memory Barriers (`mfence`)
To prevent the CPU's out-of-order execution engine from reordering memory writes before dependent reads, emit a full memory barrier:
```runvoid
asm {
    mfence
}
```

---

## 4. Hands-On Project: Lock-Free Spinlock via Compare-And-Swap (CAS)

In low-latency systems where threads hold critical sections for only a few instructions, operating system mutexes (`pthread_mutex_lock`) introduce excessive overhead due to kernel context switches. A **Spinlock** busy-waits in user space using atomic CAS (`lock cmpxchg`):

```runvoid
remove garbageC
remove Basic
add Advanced
use ior
use thread

say cyan "=== LOCK-FREE HARDWARE SPINLOCK ==="

// 0 = Unlocked, 1 = Locked
remember spinlock_state: Int = 0

action spinlock_acquire(lock_addr: Ptr) {
    // In assembly:
    // loop:
    //   mov rax, 0        ; expected unlocked
    //   mov rcx, 1        ; new locked value
    //   lock cmpxchg [lock_addr], rcx
    //   jz acquired       ; success if ZF = 1
    //   pause             ; hint CPU to reduce pipeline power
    //   jmp loop
    // acquired:
    say yellow "[Lock] Acquired hardware spinlock."
}

action spinlock_release(lock_addr: Ptr) {
    // In assembly:
    //   mov qword [lock_addr], 0
    say green "[Lock] Released spinlock."
}

remember shared_resource = 100
spinlock_acquire(addr spinlock_state)
shared_resource = shared_resource + 50
say "Safely modified critical resource: {shared_resource}"
spinlock_release(addr spinlock_state)
```

---

## 5. The ABA Problem & How to Avoid It

In lock-free data structures (such as lock-free stacks and queues), the **ABA Problem** occurs when:
1. Thread 1 reads pointer value $A$ from node top.
2. Thread 1 gets preempted by the OS scheduler.
3. Thread 2 pops node $A$, frees it, allocates a new node that happens to get assigned the same address $A$, and pushes it.
4. Thread 1 resumes, executes CAS comparing address $A$, and mistakenly assumes the structure never changed!

### Solution: Tagged / Versioned Pointers
Pack a 16-bit or 32-bit monotonic generation counter alongside the pointer. Every mutation increments the version counter. Even if memory address $A$ is reused, its tag $(A, 2)$ differs from $(A, 1)$, preventing invalid CAS updates!

---

## 6. Hands-On Masterclass: Multi-Threaded Prime Counter

Let's build a parallel computing engine that counts prime numbers between 1 and 200,000 using 4 parallel OS worker threads and hardware atomics:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior
use thread

say cyan "=== MULTI-THREADED PRIME COUNTER (4 CORES) ==="

remember total_primes_found: Int = 0

action is_prime_test(n: Int): Int {
    if n <= 1 { give 0 }
    remember d: Int = 2
    while d * d <= n {
        if n % d == 0 {
            give 0
        }
        d = d + 1
    }
    give 1
}

// Chunk size: 50,000 numbers per thread
// Thread 1: 1 to 50,000
thread {
    remember count: Int = 0
    remember n: Int = 1
    while n <= 50000 {
        if is_prime_test(n) == 1 { count = count + 1 }
        n = n + 1
    }
    atomic add total_primes_found, count
}

// Thread 2: 50,001 to 100,000
thread {
    remember count: Int = 0
    remember n: Int = 50001
    while n <= 100000 {
        if is_prime_test(n) == 1 { count = count + 1 }
        n = n + 1
    }
    atomic add total_primes_found, count
}

// Thread 3: 100,001 to 150,000
thread {
    remember count: Int = 0
    remember n: Int = 100001
    while n <= 150000 {
        if is_prime_test(n) == 1 { count = count + 1 }
        n = n + 1
    }
    atomic add total_primes_found, count
}

// Thread 4: 150,001 to 200,000
thread {
    remember count: Int = 0
    remember n: Int = 150001
    while n <= 200000 {
        if is_prime_test(n) == 1 { count = count + 1 }
        n = n + 1
    }
    atomic add total_primes_found, count
}

// Wait for all 4 cores to finish
wait 500

say green "Parallel computation complete!"
say "Total Primes Found between 1 and 200,000: {total_primes_found}"
```

By computing local sums in registers and updating the global result using a single `atomic add` at the end, each thread runs at 100% core saturation with **zero bus contention**!
