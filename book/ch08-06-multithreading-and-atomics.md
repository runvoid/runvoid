# 8.6 Multithreading & Hardware Atomics

Runvoid provides native OS multithreading capabilities backed by POSIX threads (`pthread_create`), accompanied by hardware-locked atomic synchronization.

## Spawning Native Threads (<code>thread</code>)

```runvoid
remove garbageC
remove Basic
add Advanced
use ior
use thread

remember shared_counter: Int = 0

# Spawn a concurrent thread:
thread {
    atomic add shared_counter, 10
}

wait 1
say shared_counter   // prints 10
```

## Hardware Atomic Operations (<code>atomic add</code>)

Multithreaded operations require memory synchronization to prevent race conditions. The `atomic add` statement emits a hardware bus-locked instruction directly onto the CPU:

```text
lock add [r12 + offset], rax
```

This guarantees atomic read-modify-write semantics across multi-core CPU architectures without the performance penalty of heavyweight mutex locks.
