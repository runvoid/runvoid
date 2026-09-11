# 8.4 Raw Pointers & Manual Memory

In systems programming, direct memory manipulation is essential for interfacing with hardware, building custom data structures, and optimizing caches.

## Taking Memory Addresses (<code>addr</code>)

The `addr` keyword obtains the raw 64-bit stack address of any variable:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior
use mem

remember val: Int = 100
remember ptr: Ptr = addr val
```

## Dereferencing Read & Write (<code>@</code>)

Use the `@` symbol to read or write the value pointed to by a pointer:

```runvoid
# Read value at memory location:
say @ptr       // prints 100

# Write value to memory location:
@ptr = 250
say val        // prints 250
say @ptr       // prints 250
```

## Direct Heap Allocation (<code>alloc</code> and <code>free</code>)

Allocate and deallocate unmanaged heap memory with byte-level precision:

```runvoid
# Allocate 64 bytes of memory on the heap:
remember buffer: Ptr = alloc 64

# Write a 64-bit integer into the allocated block:
@buffer = 9999
say @buffer    // prints 9999

# Deallocate the memory:
free buffer
```
