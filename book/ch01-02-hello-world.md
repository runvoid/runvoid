# 1.2 Hello, World!

Now that you have installed Runvoid and its toolchain prerequisites, let’s write your first program. It is a time-honored tradition when learning a new programming language to write a small program that prints the text `Hello, world!` to the screen. In this section, we will do exactly that!

---

## Creating a Project Directory

Start by making a directory for your first project:

```bash
mkdir -p ~/projects/hello_world
cd ~/projects/hello_world
```

Next, create a new source file named `main.rv`. The `.rv` extension is the standard file extension for all Runvoid source code.

Open `main.rv` in your preferred text editor (such as VS Code, Neovim, or Nano) and enter the following single line of code:

```runvoid
say "Hello, world!"
```

Save the file. That is the entire program! 

Notice what is **not** there:
- No boilerplate `fn main() { ... }` or `int main(int argc, char *argv[])`.
- No class wrappers like `public class Program { public static void main(String[] args) { ... } }`.
- No mandatory header includes like `#include <stdio.h>` or `use std::io;`.

In Runvoid, top-level code executes directly from top to bottom, while still allowing you to define modular functions, structures, and directives wherever needed.

---

## Compiling and Running the Program

Runvoid provides two ways to run your code:

### 1. Rapid Execution with `runvoid run`

During active development and rapid prototyping, you can compile and execute a program in a single step using the `run` command:

```bash
$ runvoid run main.rv
Hello, world!
```

If `Hello, world!` printed cleanly to your terminal, congratulations! You have officially written, compiled, and executed your first Runvoid program.

Under the hood, `runvoid run` compiles your source code into a temporary native binary in your system temp folder, executes it with any arguments you specify, and automatically cleans up the temporary files when done.

### 2. Standalone Binary Compilation with `runvoid build`

When you are ready to distribute your software or measure true bare-metal performance, compile a permanent, standalone native binary with the `build` command:

```bash
$ runvoid build main.rv -o hello
[runvoid] Compiling main.rv -> hello (target: linux)...
[runvoid] Build completed successfully: hello
```

Now execute the native binary directly from your shell without the `runvoid` compiler:

```bash
$ ./hello
Hello, world!
```

Let's check the size of the resulting binary:

```bash
$ ls -lh hello
-rwxr-xr-x 1 user user 16K hello
```

The resulting executable is a mere **16 kilobytes**! It requires no virtual machine, no Python runtime, and no Node.js engine. It is pure, raw x86_64 machine instructions.

---

## Anatomy of the Program

Let’s examine our one-line program in detail:

```runvoid
say "Hello, world!"
```

Here are the key elements:

1. **`say`**: This is a core statement in Runvoid. It instructs the program to output the evaluated expression or string to standard output (`stdout`), followed automatically by an operating system newline (`\n`).
   
   Runvoid also supports first-class terminal colors directly on `say` statements without importing any third-party terminal styling libraries:
   ```runvoid
   say green "Success: Operation completed."
   say red "Error: Invalid parameter."
   say cyan "Info: Connecting to server..."
   ```

2. **`"Hello, world!"`**: This is a string literal. In Runvoid, string literals are enclosed in double quotes. In the compiled binary, string literals are stored as null-terminated UTF-8 byte sequences in the read-only data section of the executable.

---

## Under the Hood: Dissecting the Assembly

What actually happens when Runvoid compiles `main.rv`? 

Unlike interpreted languages, Runvoid translates your code directly into **x86_64 NASM assembly**. You can inspect the exact assembly instructions emitted by the compiler at any time using the `emit-asm` command:

```bash
$ runvoid emit-asm main.rv
```

Let’s examine the emitted assembly:

```nasm
default rel
global main
extern printf

section .rodata
    str_0 db "Hello, world!", 0

section .text
main:
    push rbp
    mov rbp, rsp

    ; say "Hello, world!"
    lea rdi, [str_0]
    call print_string

    ; Clean exit with code 0
    xor eax, eax
    mov rsp, rbp
    pop rbp
    ret
```

Notice how clean and idiomatic the output is:
- `section .rodata`: Allocates our string `"Hello, world!"` in immutable read-only memory.
- `main:`: Establishes a standard 64-bit AMD64 stack frame (`push rbp; mov rbp, rsp`).
- `lea rdi, [str_0]`: Loads the 64-bit effective address of our string into the `%rdi` register (the first argument register in the standard System V AMD64 calling convention).
- `call print_string`: Calls the highly optimized runtime print routine.
- `xor eax, eax`: Sets the return exit code to 0 (indicating success) and restores the stack frame before returning.

---

## Cross-Compiling for Windows

What if you want to share your new program with a colleague running Windows? Runvoid makes cross-compiling as easy as adding a single flag:

```bash
runvoid build --target windows main.rv -o hello.exe
```

The compiler will automatically:
1. Target the Windows PE-COFF x86_64 binary format.
2. Emit Windows-compliant read-only data sections (`section .rdata`).
3. Link with the Windows C runtime (`ucrtbase`) and necessary subsystem libraries (`kernel32`, `user32`).
4. Output a standalone `hello.exe` that runs natively on Windows 10 and 11!

