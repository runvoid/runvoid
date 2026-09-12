# Appendix C: Troubleshooting & Error Encyclopedia

This appendix provides an exhaustive troubleshooting encyclopedia for compiler errors, syntax traps, assembler and linker issues, POSIX/Win32 runtime faults, and debugging strategies.

---

## 1. Compiler Front-End Errors & Diagnostics

### `error: Use of undeclared variable '<identifier>'`
- **Root Cause:** You attempted to read or assign to an identifier that was never bound in the current or parent scopes using `remember`.
- **Intelligent Diagnostic:** The Runvoid compiler calculates the Levenshtein edit distance against all in-scope symbols and language keywords:
  ```text
  error: Use of undeclared variable 'cunter'
    --> src/main.rv:14:9
     |
  14 |     cunter = cunter + 1
     |     ^^^^^^ unknown symbol
     |
     = help: did you mean 'counter'?
  ```
- **Resolution:** Check spelling or declare the variable beforehand: `remember counter = 0`.

---

### `error: Under 'remove Basic', explicit type annotations are required`
- **Root Cause:** The `remove Basic` directive disables dynamic typing and implicit coercion. Every variable declaration, action parameter, and return value must specify a concrete static type.
- **Resolution:**
  ```runvoid
  // INCORRECT in Pro Mode:
  remember buffer_size = 4096
  action process(item) { ... }

  // CORRECT in Pro Mode:
  remember buffer_size: Int = 4096
  action process(item: String) -> Bool { ... }
  ```

---

### `error: Cannot use low-level memory allocation without 'add Advanced'`
- **Root Cause:** Low-level features (`alloc`, `free`, raw pointers `@ptr`, memory addresses `addr x`, inline assembly `asm`) are guarded against accidental usage in casual scripts.
- **Resolution:** Include `add Advanced` in the directive header at the very top of your file.

---

### `error: Unterminated string literal`
- **Root Cause:** A string literal was opened with `"` but never closed before a line break or EOF.
- **Resolution:** Ensure string literals are closed with a matching quotation mark. If multiline strings are needed, concatenate them with `+ "\n"`.

---

## 2. Assembler & Linker Failures

### `nasm: command not found`
- **Root Cause:** The Netwide Assembler (`nasm`) is not installed or not discoverable in your system `PATH`.
- **Resolution:**
  - **Arch Linux:** `sudo pacman -S nasm`
  - **Ubuntu / Debian:** `sudo apt install nasm`
  - **Fedora / RHEL:** `sudo dnf install nasm`
  - **macOS:** `brew install nasm`
  - **Windows:** `winget install NASM.NASM` (or manually add `C:\Program Files\NASM` to environment variables).

---

### `relocation R_X86_64_32S against symbol 'msg' can not be used when making a PIE object`
- **Root Cause:** When generating Position-Independent Executables (PIE), modern linkers require RIP-relative addressing rather than absolute 32-bit immediate addresses.
- **Resolution:** In inline assembly blocks (`asm { ... }`), always reference memory labels using `rel`:
  ```nasm
  ; INCORRECT:
  lea rsi, [msg]

  ; CORRECT (Position-Independent):
  lea rsi, [rel msg]
  ```

---

### `undefined reference to 'main'` / `undefined reference to '_start'`
- **Root Cause:**
  - If compiling standard binaries: The C runtime entry point `main` was not generated.
  - If compiling freestanding bare-metal (`add Freestanding`): The linker expected a raw entry symbol `_start`.
- **Resolution:** When compiling freestanding kernels, specify `-nostdlib -e _start` in your linker flags or define the global `_start` label in assembly.

---

### `error while loading shared libraries: libfoo.so: cannot open shared object file`
- **Root Cause:** A dynamically linked C library (via `use lib "foo"`) exists on disk, but its directory is not in the system's runtime dynamic loader search path.
- **Resolution:**
  - Temporarily extend the library path: `export LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH`
  - Or place the library in `/usr/local/lib` and run `sudo ldconfig`.

---

## 3. Runtime Crashes & Signals

### `Segmentation Fault (core dumped)` / Crash on C `call`
- **Root Cause 1 (Stack Misalignment):** External C libraries (especially glibc, libc, and OpenSSL) use SIMD vector instructions (like `movaps` or `vmovdqa`) that require the stack pointer `%rsp` to be an exact multiple of 16. If your inline assembly pushed an odd number of quadwords onto the stack before executing `call`, the CPU triggers an immediate alignment fault (General Protection Fault, GPF #13).
  - *Fix:* Ensure `%rsp` is 16-byte aligned prior to any `call`.
- **Root Cause 2 (Null Pointer Dereference):** Reading from or writing to pointer address `0x0`.
  - *Fix:* Guard pointer accesses with `if ptr != 0`.

---

### `Floating Point Exception (SIGFPE)`
- **Root Cause:** An integer division or modulo operation was executed with a divisor equal to zero (`x / 0` or `x % 0`).
- **Resolution:** Check that divisors are non-zero before invoking mathematical operations.

---

### `Stack Overflow / Maximum Call Depth Exceeded`
- **Root Cause:** An uncontrolled recursive action called itself repeatedly until hardware stack memory was exhausted.
- **Resolution:** Ensure recursive functions include a well-defined base case, or refactor deep algorithms into iterative `while` loops.

---

## 4. Advanced Debugging Workflows

### 1. Interactive Debugging with GDB
To debug a compiled Runvoid executable step-by-step:
```bash
# Launch under GNU Debugger:
gdb ./my_app

# Inside GDB:
(gdb) break main            # Set breakpoint at main entry
(gdb) run                   # Run program
(gdb) info registers rax rsp # Inspect CPU registers
(gdb) x/16xg $rsp           # Examine 16 quadwords on stack in hex
(gdb) stepi                 # Step exactly one machine instruction
(gdb) backtrace             # Print active call stack
```

### 2. Leak & Memory Safety Checking with Valgrind
When running Pro systems code with manual memory allocations (`alloc` and `free`):
```bash
valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes ./my_app
```
Valgrind intercepts every `alloc`/`free` call and reports exact line numbers for any unreleased blocks or use-after-free conditions.

---

## 5. Windows Defender / Antivirus False Positives

### Antivirus Silently Quarantining `.exe`
- **Root Cause:** Freshly compiled binary executables without an established digital certificate signature from a commercial Certificate Authority (CA) may trigger heuristic warnings in Windows Defender SmartScreen.
- **Resolution:**
  - Add your development build directory (e.g. `C:\Users\You\Projects`) to Windows Defender Exclusions.
  - For official software distributions, sign release binaries with a valid Authenticode digital certificate (`signtool.exe`).
