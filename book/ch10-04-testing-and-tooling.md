# 10.4 Test Runner, Assertions & Hot-Reloading Watch

Developing robust software requires automated verification and frictionless iteration cycles. Runvoid 0.3.0 provides native unit testing syntax and CLI tooling out of the box.

## Writing Tests

Use `test "name" { ... }` blocks to define test suites, and `verify that <condition>` to assert expectations:

```runvoid
test "player initialization" {
    remember player = name: "Hero", hp: 100, gold: 50
    verify that player["hp"] is 100
    verify that player's gold is 50
    verify that player has "name"
}

test "inventory capacity" {
    remember backpack = "sword", "potion"
    add "shield" to backpack
    verify that {count backpack} is 3
}
```

If any `verify` assertion fails, Runvoid aborts execution with the exact line number, actual value, and expected value in bright red terminal text.

## Running Tests via CLI

Run all tests in `tests/` or current directory with a single command:

```bash
$ runvoid test
Running 3 test file(s)...

▶️ Running test file: "tests/01_maps_test.rv"
🧪 test map creation and indexing ... ok
🧪 test map mutation ... ok
✅ Passed: "tests/01_maps_test.rv"

==========================================
Test suite passed! (3 passed, 0 failed)
```

## Interactive REPL (`runvoid repl`)

Experiment with expressions and statements interactively without creating files:

```bash
$ runvoid repl
╔══════════════════════════════════════════════════════════════╗
║           Runvoid Interactive REPL v0.3.0                   ║
║   Type :help for help, :clear to reset, :exit to quit        ║
╚══════════════════════════════════════════════════════════════╝
runvoid> remember hero = name: "Alex", hp: 100
runvoid> hero's hp
100
runvoid> hero has "name"
true
runvoid> 12 bit and 10
8
runvoid> :exit
Goodbye!
```

## Hot-Reloading Watcher (`runvoid watch`)

Automatically recompile and re-execute your script whenever you save changes:

```bash
$ runvoid watch game.rv
👀 Watching "game.rv" for changes... (Press Ctrl+C to stop)
```
Whenever you edit and save `game.rv`, Runvoid recompiles in milliseconds and runs the updated executable immediately.
