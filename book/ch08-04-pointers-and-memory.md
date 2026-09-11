# 8.4 Raw Pointers & Manual Memory

In high-level application code, variables and collections abstract away the physical addresses where bytes reside. But in systems software—such as memory allocators, network device drivers, packet serializers, and game engines—direct access to raw memory addresses is indispensable.

Runvoid Pro Systems Mode provides full, unconstrained access to 64-bit hardware pointers, address-of operators, memory dereferencing, and manual heap allocation.

---

## 1. The Pointer Type (`Ptr`) and Virtual Memory

A pointer in Runvoid has the type `Ptr`. Under x86_64 architecture, a `Ptr` is a 64-bit numerical value representing a byte address within the operating system process's virtual memory space.

A pointer can reference:
- Memory on the hardware stack (local variables).
- Memory in the data sections (`.rodata`, `.data`, `.bss`).
- Memory allocated dynamically on the heap.
- Memory-mapped hardware I/O registers or memory-mapped files (`mmap`).

---

## 2. Taking Memory Addresses with `addr`

To obtain the memory address of an existing variable, use the `addr` operator:

```runvoid
remove garbageC
remove Basic
add Advanced

use ior

remember counter: Int = 100
remember ptr_to_counter: Ptr = addr counter

say "Counter value: {counter}"
say "Stack memory address: {ptr_to_counter}"
```

### Under the Hood
In x86_64 NASM assembly, `addr counter` translates directly to the single-cycle Load Effective Address instruction:
```nasm
lea rax, [rbp - 8]
mov [rbp - 16], rax
```

---

## 3. Dereferencing Memory with `@`

The `@` operator dereferences a pointer, allowing you to read or write the value at the target memory location:

### Reading Memory via `@ptr`
```runvoid
remember value_read: Int = @ptr_to_counter
say "Read via pointer: {value_read}" // Prints: 100
```
Translates to:
```nasm
mov rax, [rbp - 16]   ; Load pointer address into rax
mov rbx, [rax]        ; Dereference 64-bit value at address
```

### Writing Memory via `@ptr = value`
```runvoid
@ptr_to_counter = 500

say "Updated via pointer: {counter}" // Prints: 500
say "Dereferenced again: {@ptr_to_counter}" // Prints: 500
```
Translates to:
```nasm
mov rax, [rbp - 16]   ; Load pointer address
mov qword [rax], 500  ; Store 64-bit value directly into address
```

---

## 4. Pointer Arithmetic

You can navigate contiguous memory blocks by applying integer arithmetic to pointers. Because addresses are byte-indexed, stepping to the next 64-bit (`Int`) element requires adding 8 bytes:

```runvoid
action read_element(base_ptr: Ptr, index: Int): Int {
    remember offset: Int = index * 8
    remember element_ptr: Ptr = base_ptr + offset
    give @element_ptr
}
```

---

## 5. Manual Heap Allocation: `alloc` and `free`

When you remove the garbage collector (`remove garbageC`), dynamic memory must be explicitly allocated and freed using `alloc` and `free` from the `mem` module:

```runvoid
remove garbageC
remove Basic
add Advanced

use ior
use mem

// Allocate a buffer of 256 contiguous bytes on the heap:
remember buffer_size: Int = 256
remember buffer: Ptr = alloc buffer_size

if buffer == 0 {
    say red "Fatal error: Out of memory!"
    exit(1)
}

// Write integers into the first two slots (8 bytes each):
@buffer = 1337
@(buffer + 8) = 2026

say "Slot 0: {@buffer}"        // 1337
say "Slot 1: {@(buffer + 8)}"  // 2026

// Explicitly free the allocated memory block back to the OS:
free buffer

// Hygiene: Null out the pointer to prevent accidental use-after-free:
buffer = 0
say green "Memory deallocated cleanly."
```

---

## 6. Systems Memory Safety Best Practices

Working with raw pointers grants absolute power, but requires engineering discipline:
1. **Always Check for Null:** `alloc` returns `0` (null) if the operating system runs out of virtual address space. Always verify `if ptr != 0` before dereferencing.
2. **One Owner per Allocation:** Clearly designate which function is responsible for calling `free` on dynamically allocated blocks.
3. **Null After Free:** Immediately assign `ptr = 0` after calling `free ptr` to prevent dangling pointer bugs.
4. **Bounds Discipline:** Ensure pointer offsets never exceed the allocated byte boundary.

