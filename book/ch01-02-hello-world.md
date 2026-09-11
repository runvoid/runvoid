# 1.2 Hello, World!

Now that you’ve installed Runvoid, let’s write your first program. It’s traditional when learning a new language to write a little program that prints the text `Hello, world!` to the screen, so we’ll do the same here!

## Writing the Program

Create a new file named `main.rv`:

```runvoid
say "Hello, world!"
```

Save the file. That’s the entire program! No `fn main()`, no headers, no class wrappers.

## Running the Program

To run your program immediately without generating lingering binary files, use the `run` command:

```bash
$ runvoid run main.rv
Hello, world!
```

If `Hello, world!` printed to your terminal, congratulations! You have officially written and run your first Runvoid program.

## Anatomy of a Runvoid Program

Let’s review the `Hello, world!` program in detail.

```runvoid
say "Hello, world!"
```

- `say` is a built-in statement in Runvoid. It tells the computer to print whatever follows to the screen, followed by a new line.
- `"Hello, world!"` is a string literal.
- When compiled, the Runvoid compiler transforms `say "Hello, world!"` into an optimized sequence of x86_64 assembly instructions that call the runtime write subroutine via Linux syscalls.
