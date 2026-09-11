# 2. Programming a Guessing Game

Let’s jump into Runvoid by building a complete, interactive project together! 

This chapter introduces you to practical Runvoid programming by walking through a real-world program step by step. You will learn about:
- Declaring and mutating variables with `remember`.
- Prompting for user input with `ask`.
- Generating secure pseudo-random numbers with `random ... to ...`.
- String interpolation with `{expression}`.
- Conditional branching with `if`, `otherwise if`, and `otherwise`.
- Loop control with `while` and `stop`.
- Adding polish with colored terminal output and audio feedback.
- Cross-compiling the finished game into native executables for Linux and Windows.

---

## What We Are Building

We will implement a classic beginner programming challenge: a **number guessing game**. 

Here is how the game behaves:
1. The game chooses a secret random integer between 1 and 100.
2. It prompts the player to enter a guess.
3. If the guess is too low, the game outputs a warning in yellow: `"Too small! Try higher."`
4. If the guess is too high, it outputs: `"Too big! Try lower."`
5. When the player guesses the exact number, the game plays a celebratory victory beep, prints a victory message in green with the total number of attempts, and exits.

---

## Step 1: Setting Up the Project

Create a new directory for your game and create a source file named `guessing_game.rv`:

```bash
mkdir -p ~/projects/guessing_game
cd ~/projects/guessing_game
touch guessing_game.rv
```

Let's write the first draft of our program to welcome the player and prompt for an initial guess. Open `guessing_game.rv` in your editor:

```runvoid
say cyan "=== Welcome to the Runvoid Guessing Game! ==="
say "I have chosen a secret number between 1 and 100."

remember guess = ask "Please input your guess: "
say "You guessed: {guess}"
```

### Testing the First Draft

Run the program using the `runvoid run` command:

```bash
$ runvoid run guessing_game.rv
=== Welcome to the Runvoid Guessing Game! ===
I have chosen a secret number between 1 and 100.
Please input your guess: 42
You guessed: 42
```

The game printed our header, waited for keyboard input, captured our typed value into the variable `guess`, and echoed it back using string interpolation `{guess}`.

---

## Step 2: Processing Input and Variables

Let’s dissect the new code we just wrote:

```runvoid
remember guess = ask "Please input your guess: "
```

### The `remember` Statement
In Runvoid, variables are declared using the conversational keyword `remember`. 
- By default in scripting mode, variables are dynamically typed and mutable.
- You can reassign a variable later in the code simply by referencing its name: `guess = 50`.

### The `ask` Statement
The `ask` keyword combines printing a prompt with reading a line of text from standard input (`stdin`).
- It prints the prompt message without an automatic trailing newline, keeping the user's cursor on the same line.
- It captures the text entered by the user up to the return key (`\n`), strips trailing newline characters, and returns the result.

### String Interpolation
```runvoid
say "You guessed: {guess}"
```
Any expression wrapped in curly braces `{...}` inside a string literal is evaluated at runtime and converted into human-readable text.

---

## Step 3: Generating a Secret Number

Next, we need to generate a secret number between 1 and 100 that the player will try to guess. Runvoid provides a built-in random range operator:

```runvoid
remember secret_number = random 1 to 100
```

Under the hood, `random A to B` queries the underlying operating system's cryptographic entropy source (`/dev/urandom` or `getrandom` syscall on Linux, `CryptGenRandom` / `BCryptGenRandom` on Windows) and scales the value to the inclusive range `[A, B]`.

Let’s update `guessing_game.rv`:

```runvoid
say cyan "=== Welcome to the Runvoid Guessing Game! ==="
say "I have chosen a secret number between 1 and 100."

remember secret_number = random 1 to 100
remember guess = ask "Please input your guess: "

if guess < secret_number {
    say yellow "Too small! The secret was {secret_number}."
} otherwise if guess > secret_number {
    say yellow "Too big! The secret was {secret_number}."
} otherwise {
    say green "Incredible! You guessed it on your first try!"
}
```

Try running it a few times to observe how the program responds to different inputs.

---

## Step 4: Allowing Multiple Guesses with Looping

Currently, the game exits after a single guess. To make the game playable, we want to loop until the player successfully guesses the number. We also want to count how many attempts the player took.

In Runvoid, you can create a loop using `while condition { ... }`. You can also break out of any loop immediately using the `stop` statement.

Let's modify `guessing_game.rv`:

```runvoid
say cyan "=== Welcome to the Runvoid Guessing Game! ==="
say "I have chosen a secret number between 1 and 100."

remember secret_number = random 1 to 100
remember attempts = 0

while true {
    remember guess = ask "Enter your guess (or 'quit'): "
    
    if guess == "quit" {
        say "Thanks for playing! The secret number was {secret_number}."
        stop
    }
    
    attempts = attempts + 1

    if guess < secret_number {
        say yellow "Too small! Try a higher number."
    } otherwise if guess > secret_number {
        say yellow "Too big! Try a lower number."
    } otherwise {
        say green "You win! You guessed the secret number {secret_number} in {attempts} attempts!"
        stop
    }
}
```

### How the Loop Operates:
1. `while true`: Creates an infinite loop that continues until an explicit `stop` is reached.
2. `if guess == "quit"`: Allows the player to gracefully forfeit at any time.
3. `attempts = attempts + 1`: Increments our counter on each valid guess.
4. `stop`: Terminates the loop when the guess matches `secret_number`.

---

## Step 5: Adding Audio and Visual Polish

Let’s make the game feel like a classic arcade experience! Runvoid includes built-in hardware audio and terminal styling:

- `beep frequency for duration_ms`: Triggers a native system tone (PC speaker frequency in Hertz, duration in milliseconds).
- Colored text: `say green`, `say yellow`, `say red`, `say cyan`.

Let's add sound effects to our game:

```runvoid
say cyan "=== Welcome to the Runvoid Guessing Game! ==="
say "I have chosen a secret number between 1 and 100."

remember secret_number = random 1 to 100
remember attempts = 0

while true {
    remember guess = ask "Enter your guess (or 'quit'): "
    
    if guess == "quit" {
        say "Goodbye!"
        stop
    }
    
    attempts = attempts + 1

    if guess < secret_number {
        beep 400 for 100
        say yellow "Too small! Try higher."
    } otherwise if guess > secret_number {
        beep 400 for 100
        say yellow "Too big! Try lower."
    } otherwise {
        ; Play an ascending victory arpeggio!
        beep 523 for 120   ; C5
        beep 659 for 120   ; E5
        beep 784 for 150   ; G5
        beep 1046 for 300  ; C6
        
        say green "=================================================="
        say green "CONGRATULATIONS! You won in {attempts} attempts!"
        say green "=================================================="
        stop
    }
}
```

---

## Step 6: Building Native Binaries for Linux & Windows

Now that your game is finished, compile it into a standalone distribution binary.

### Building for Linux:
```bash
$ runvoid build guessing_game.rv -o guessing_game
[runvoid] Compiling guessing_game.rv -> guessing_game (target: linux)...
[runvoid] Build completed successfully: guessing_game
```

You can now run `./guessing_game` on any 64-bit Linux distribution without needing Runvoid or Rust installed!

### Cross-Building for Windows:
```bash
$ runvoid build --target windows guessing_game.rv -o guessing_game.exe
[runvoid] Compiling guessing_game.rv -> guessing_game.exe (target: windows)...
[runvoid] Build completed successfully: guessing_game.exe
```

The resulting `guessing_game.exe` will run natively on Windows 10 and 11, utilizing the Windows `Beep` API from `kernel32.dll` for the arcade audio effects!

---

## Summary

In this tutorial, you built a complete, interactive, cross-platform terminal game with Runvoid. You learned:
- How to declare and update variables using `remember`.
- How to handle user input with `ask`.
- How to generate random numbers with `random A to B`.
- How to control program execution with `if`, `otherwise`, and `while`.
- How to enrich CLI applications with native audio (`beep`) and colors.
- How to compile standalone release binaries for both Linux and Windows.

In the next chapter, we will explore Runvoid’s common programming concepts in deeper detail!

