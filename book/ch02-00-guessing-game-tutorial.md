# 2. Programming a Guessing Game

Let’s jump into Runvoid by working through a hands-on project together! This chapter introduces you to a few common Runvoid concepts by showing you how to use them in a real program. You’ll learn about `remember`, `ask`, `say`, string interpolation `{...}`, `random ... to ...`, and control flow with `while`, `if`, and `stop`.

## What We're Building

We will implement a classic beginner programming problem: a guessing game. Here’s how it works: the program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. Upon a guess being entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will print a congratulatory message and exit.

## Setting Up the Game

Create a new file named `guessing_game.rv`:

```runvoid
say "=== Guess the Number! ==="

remember secret_number = random 1 to 100
remember attempts = 0
remember guessed = false

while not guessed {
    remember guess = ask "Please input your guess: "
    attempts = attempts + 1

    if guess < secret_number {
        say yellow "Too small! Try higher."
    } otherwise if guess > secret_number {
        say yellow "Too big! Try lower."
    } otherwise {
        say green "You win! You guessed the number in {attempts} attempts!"
        guessed = true
    }
}
```

## Running the Guessing Game

Run your game from the terminal:

```bash
$ runvoid run guessing_game.rv
=== Guess the Number! ===
Please input your guess: 50
Too small! Try higher.
Please input your guess: 75
Too big! Try lower.
Please input your guess: 63
You win! You guessed the number in 3 attempts!
```

## How It Works

1. **`remember secret_number = random 1 to 100`**: Generates a random 64-bit integer between 1 and 100 using the Linux kernel's high-entropy random source.
2. **`remember guess = ask "..."`**: Prompts the user and reads their input from standard input.
3. **`while not guessed`**: Runs the loop until `guessed` becomes `true`.
4. **`if / otherwise if / otherwise`**: Compares the player's guess to the secret number and outputs colored terminal feedback (`say yellow`, `say green`).
5. **`{attempts}`**: String interpolation dynamically inserts the current attempt counter directly into the victory message.
