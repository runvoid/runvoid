# 8.6 Multithreading & Hardware Atomics

Modern computers achieve high performance by distributing workloads across multiple CPU cores. In systems programming, taking advantage of symmetric multiprocessing (SMP) requires spawning operating system threads and carefully coordinating concurrent access to shared memory.

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

## 2. Data Races and Hardware Atomics

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

## 3. How Atomics Work on the Hardware Bus

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

## 4. Mutex Synchronization for Complex Critical Sections

When a critical section requires executing multiple non-atomic operations together (such as modifying several variables or updating a linked list), use a mutex:

```runvoid
remember lock_handle: Int = 0

action critical_section(): Void {
    mutex_lock(lock_handle)
    
    // Protected multi-step logic:
    say "Performing protected critical section work..."
    
    mutex_unlock(lock_handle)
}
```

By combining hardware atomics for counters with OS mutexes for complex critical sections, Runvoid gives you the exact concurrency tools needed for high-performance systems engineering.

