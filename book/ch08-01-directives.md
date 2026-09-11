# 8.1 Systems Philosophy & Directives

Activating Pro Systems Mode requires explicitly stating your intention to opt out of beginner-level runtime conveniences.

## The Three Core Directives

Place these directives at the very top of your source file:

```runvoid
remove garbageC
remove Basic
add Advanced
```

What each directive means:
1. **`remove garbageC`**: Completely strips the Garbage Collection runtime and GC tracking metadata from the compiled executable.
2. **`remove Basic`**: Turns off permissive type inference. Requires all variables and functions to declare explicit, static types.
3. **`add Advanced`**: Enables systems-level intrinsics, raw memory pointers, C FFI, inline assembly, and aggressive compiler optimizations (Constant Folding, Dead Code Elimination, Peephole, LTO).
