# 8.6 Multithreading & Hardware Atomics

Modern computers achieve high performance by distributing workloads across multiple CPU cores. In systems programming, taking advantage of symmetric multiprocessing (SMP) requires spawning operating system threads, pinning thread affinity, and carefully coordinating concurrent access to shared memory.

Runvoid provides first-class, cross-platform multithreading backed by native OS threads (`pthread` on Linux, `CreateThread` on Windows), complemented by hardware-level bus-locked atomics.

---

## 1. Spawning Native OS Threads

To spawn a concurrent thread of execution, import the `thread` module and use the `thread { ... }` block:

```runvoid
remove garbageC
remove Basic
add Advanced

use ior
use thread

action background_worker(): Void {
    say "Worker thread running in parallel on core..."
}

say "Main thread initializing worker..."
thread {
    background_worker()
}

say "Main thread continues non-blocking execution."
```

### Cross-Platform Threading Engine
The Runvoid runtime automatically adapts to the host operating system:
- **On Linux:** Calls `pthread_create(&thread_id, NULL, thread_wrapper, arg)` and links with `-lpthread`.
- **On Windows:** Calls the Win32 `CreateThread(NULL, 0, thread_wrapper, arg, 0, &thread_id)` API, avoiding external POSIX translation layers.

Each thread receives an independent 2MB hardware stack frame with full access to the CPU's general-purpose registers.

---

## 2. Thread-Local Storage (TLS) & Segment Registers

In high-concurrency software, passing context through every function parameter hurts ergonomics and performance. Modern x86_64 systems allocate **Thread-Local Storage (TLS)**:

```
Linux (System V AMD64):
Base address of thread-local block is pointed to by the %fs segment register:
  mov rax, [fs:0x00]   ; Loads thread's self pointer

Windows (x64 ABI):
Base address of the Thread Environment Block (TEB) is in %gs segment register:
  mov rax, [gs:0x30]   ; Loads TEB pointer
```

Variables declared with thread-local lifetime reside at fixed offsets from these segment registers, providing thread-private state with zero locking overhead!

---

## 3. Data Races and Hardware Atomics

When multiple threads read and write to the same memory location concurrently without synchronization, a **data race** occurs. This results in lost updates, corrupted data, and undefined behavior.

Traditional programming languages solve this by acquiring heavyweight operating system mutex locks. While correct, mutexes introduce system call overhead, kernel transitions, and thread context switches.

Runvoid provides hardware-level atomics via the `atomic add` statement:

```runvoid
remove garbageC
remove Basic
add Advanced

use ior
use thread

remember global_counter: Int = 0

// Spawn 4 concurrent worker threads:
repeat 4 times {
    thread {
        repeat 100000 times {
            atomic add global_counter, 1
        }
    }
}

// Wait for workers to complete:
wait 500  // 500ms

say "Final synchronized counter: {global_counter}"
```

---

## 4. How Atomics Work on the Hardware Bus

When the Runvoid code generator encounters `atomic add target, value`, it emits the x86_64 hardware bus-lock prefix:

```nasm
    ; atomic add global_counter, 1
    mov rax, 1
    lock add qword [rbp - 8], rax
```

### What Happens Inside the Silicon?
1. **The `lock` Prefix:** When the CPU decodes the `lock` prefix, it directs the memory controller and L1/L2 cache coherency hardware (the MESI protocol) to acquire exclusive ownership of the cache line containing `[rbp - 8]`.
2. **Atomic Read-Modify-Write:** The CPU performs the memory read, arithmetic addition, and memory write-back in an indivisible transaction.
3. **Zero Kernel Overhead:** No operating system thread is suspended, no context switch occurs, and no kernel spinlock is allocated. The synchronization completes in single-digit nanoseconds directly in silicon!

---

## 5. Deadlock Prevention: The Lock Hierarchy Principle

A **deadlock** occurs when Thread A holds Lock 1 and waits for Lock 2, while Thread B holds Lock 2 and waits for Lock 1. Neither can make progress.

### The Canonical Solution: Global Lock Ordering
Assign every lock an ordinal integer rank. Threads must **always acquire locks in strictly increasing order**:
$$\text{Lock}_1 < \text{Lock}_2 < \text{Lock}_3$$
By enforcing this invariant across the codebase, circular wait conditions become mathematically impossible!

---

## 6. CPU Cache Coherence: The MESI Protocol

When multi-core CPUs execute concurrent threads, each core maintains its own private L1 and L2 caches. To guarantee that writes on Core 0 are instantly visible on Core 3, hardware utilizes the **MESI Cache Protocol**:

```
      Core 0                   Core 1                   Core 2
  +------------+           +------------+           +------------+
  |  L1 Cache  |           |  L1 Cache  |           |  L1 Cache  |
  | State: [M] |           | State: [I] |           | State: [I] |
  +------------+           +------------+           +------------+
        ^                        ^                        ^
        |                        |                        |
        +------------------------+------------------------+
                     Hardware Interconnect Bus
                     (Cache-Line Invalidation)
```

- **[M] Modified:** Line is present only in current cache and dirty (different from main RAM).
- **[E] Exclusive:** Line is present only in current cache and clean (matches RAM).
- **[S] Shared:** Line is present in multiple caches in read-only state.
- **[I] Invalid:** Line contains stale data and must be refetched from RAM or another cache.

When Runvoid issues `lock add`, Core 0 broadcasts an invalidation request across the bus, transitioning all other cores' copies to **Invalid [I]** before writing the new value!

---

## 7. Hands-On Project: Implementing a Lock-Free Spinlock

A **spinlock** is a lightweight synchronization primitive that repeatedly polls in a tight CPU loop until a lock bit becomes free, avoiding operating system context switch latency:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior
use thread

remember spinlock_flag: Int = 0
remember shared_bank_balance: Int = 1000

action acquire_spinlock(lock_ptr: Ptr): Void {
    // Loop until we successfully swap 0 to 1
    while @lock_ptr != 0 {
        // Pause CPU briefly to save power (REP NOP / PAUSE)
    }
    @lock_ptr = 1
}

action release_spinlock(lock_ptr: Ptr): Void {
    @lock_ptr = 0
}

remember lock_address: Ptr = addr spinlock_flag

// Thread 1: Deduct 250
thread {
    acquire_spinlock(lock_address)
    shared_bank_balance = shared_bank_balance - 250
    release_spinlock(lock_address)
}

// Thread 2: Deposit 500
thread {
    acquire_spinlock(lock_address)
    shared_bank_balance = shared_bank_balance + 500
    release_spinlock(lock_address)
}

wait 100
say "Synchronized Balance: {shared_bank_balance}" // Guaranteed 1250!
```
