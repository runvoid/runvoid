# 10.4 Test Runner, Assertions & Hot-Reloading Watch

Writing reliable software demands rapid iteration cycles and automated regression testing. In many programming ecosystems, setting up testing requires installing third-party frameworks, configuring complex runners, and writing brittle build scripts.

Runvoid 1.3 ships with **native unit testing syntax**, a **built-in test runner CLI**, an **interactive REPL**, and a **hot-reloading file watcher** directly out of the box.

---

## 1. Writing Unit Tests: `test` and `verify that`

Runvoid introduces dedicated syntax for automated tests. You define test cases using `test "description" { ... }` blocks and check conditions with `verify that`:

```runvoid
// tests/math_test.rv

action multiply(a, b) {
    give a * b
}

action divide(a, b) {
    if b == 0 { give 0 }
    give a / b
}

test "integer multiplication" {
    verify that multiply(6, 7) is 42
    verify that multiply(-3, 5) is -15
    verify that multiply(0, 100) is 0
}

test "safe division" {
    verify that divide(100, 4) is 25
    verify that divide(50, 0) is 0
}
```

### Testing Collections and Dictionaries
You can verify complex state structures:

```runvoid
test "dictionary mutations" {
    remember player = name: "Hero", hp: 100
    add "gold": 50 to player

    verify that player's name is "Hero"
    verify that player's gold is 50
    verify that player has "hp"
    verify that {count player} is 3
}
```

---

## 2. Running Test Suites with `runvoid test`

To execute your test suite, invoke the `test` subcommand from the root of your project:

```bash
$ runvoid test
```

### Automatic Test Discovery
The test runner scans:
1. Any file matching the pattern `*_test.rv` in your project tree.
2. All `.rv` files located inside the `tests/` directory.

### Test Output and CI Exit Codes
When tests run, the runner outputs clean colorized results:

```text
[runvoid test] Discovering test files in tests/ ...
[PASS] tests/math_test.rv (5 assertions passed)
[PASS] tests/dictionary_test.rv (4 assertions passed)
[PASS] tests/collections_test.rv (8 assertions passed)
------------------------------------------------------
Result: 3 test suites, 17 assertions passed, 0 failures (14ms)
```

If an assertion fails:
- The runner prints the exact file path and line number of the failure.
- It displays the failed condition along with the expected versus actual values.
- The process exits with code `1`, making it immediately compatible with GitHub Actions and automated CI/CD pipelines.

---

## 3. Mocking & Dependency Injection in Tests

Testing networked or filesystem-heavy actions requires decoupling side-effects from pure business logic:

```runvoid
// Pass a mock storage dictionary instead of real disk I/O:
action save_user_profile(user_dict, store) {
    add user_dict["id"]: user_dict to store
    give true
}

test "user profile persistence mock" {
    remember test_store = {}
    remember test_user = {"id": 101, "name": "Elena"}
    
    save_user_profile(test_user, test_store)
    
    verify that test_store has 101
    verify that test_store[101]["name"] is "Elena"
}
```

---

## 4. The Interactive REPL (`runvoid repl`)

The Read-Eval-Print Loop lets you prototype ideas, experiment with expressions, and test library functions without creating temporary files:

```bash
$ runvoid repl
Runvoid 1 Interactive REPL (v1.3)
Type :help for help, :clear to reset, :exit to quit

runvoid> remember user = name: "Aria", score: 950
runvoid> user's name
"Aria"
runvoid> 1 shift left 4
16
runvoid> [1, 2, 3, 4].length
4
runvoid> :exit
Goodbye!
```

### REPL Meta-Commands:
- `:help`: Displays interactive commands.
- `:clear`: Resets active variables and functions in the current session.
- `:exit`: Gracefully terminates the REPL.

---

## 5. Hot-Reloading Watcher (`runvoid watch`)

During active feature development or game prototyping, restarting the compiler on every small edit breaks your flow state. The `watch` subcommand provides frictionless continuous execution:

```bash
$ runvoid watch src/game.rv
```

### How Watch Mode Operates:
1. Monitors `src/game.rv` and imported modules for filesystem change events.
2. Incorporates intelligent debouncing (150ms) to ensure saving multiple buffers doesn't trigger rapid recompile thrashing.
3. Automatically clears the terminal window, recompiles the source code into native machine code, and launches the updated binary immediately.

You can combine `watch` with cross-compilation:
```bash
$ runvoid watch --target windows src/game.rv
```
Whenever you edit code on Linux, Runvoid recompiles the Windows PE executable and launches it inside Wine instantly!

---

## 6. Test-Driven Development (TDD) Workflow

In Test-Driven Development, you write the verification check *before* writing the implementation:

### Step 1: Write the Failing Test (`tests/string_utils_test.rv`)
```runvoid
action reverse_word(w) {
    // Intentionally empty stub
    give ""
}

test "reversing standard words" {
    verify that reverse_word("void") is "diov"
}
```
Run `runvoid test`:
```text
[FAIL] tests/string_utils_test.rv
  Assertion failed at line 8: expected "diov", but received ""
Result: 1 test suite, 0 passed, 1 failure
```

### Step 2: Write the Passing Implementation
```runvoid
action reverse_word(w) {
    remember len = w.length
    remember result = ""
    remember i = len - 1
    while i >= 0 {
        result = result + w[i]
        i = i - 1
    }
    give result
}
```
Run `runvoid test` again:
```text
[PASS] tests/string_utils_test.rv (1 assertion passed)
Result: 1 test suite, 1 passed, 0 failures (10ms)
```

Now you have automated regression protection for every future refactor!

### Summary of Testing Principles
By verifying behavior before writing code, checking boundaries, and running automated test suites on every pull request, your software remains bulletproof across platforms and versions.
