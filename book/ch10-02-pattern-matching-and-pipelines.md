# 10.2 Pattern Matching & Pipelines

Modern systems and application programming heavily involve transforming structured data and dispatching actions based on dynamic values.

Runvoid 1.3 introduces two expressive constructs:
1. **Pattern Matching (`match`):** High-speed multi-branch value matching that replaces cumbersome `if`/`otherwise` ladders.
2. **The Function Pipeline Operator (`|>`):** Left-to-right data transformation chains that eliminate unreadable nested parentheses.

---

## 1. Pattern Matching with `match`

In many programming languages, testing a single variable against multiple potential values produces unwieldy nested conditional statements. In Runvoid, `match` tests a subject expression against patterns clearly and cleanly:

```runvoid
remember http_status = 404

match http_status {
    when 200 -> say green "200 OK: Request succeeded."
    when 201 -> say green "201 Created: Resource provisioned."
    when 400 -> say yellow "400 Bad Request: Malformed syntax."
    when 401 -> say red "401 Unauthorized: Authentication required."
    when 403 -> say red "403 Forbidden: Access denied."
    when 404 -> say red "404 Not Found: Endpoint does not exist."
    when 500 -> say red "500 Internal Server Error."
    otherwise -> say yellow "Unhandled HTTP response code: {http_status}"
}
```

### Matching String Values
Pattern matching works seamlessly with string literals:

```runvoid
remember user_action = ask "Choose action (start / restart / stop): "

match user_action {
    when "start" -> {
        say green "Booting service cluster..."
        beep 800 for 100
    }
    when "restart" -> {
        say yellow "Restarting daemons..."
        beep 600 for 100
    }
    when "stop" -> {
        say red "Terminating all processes."
        beep 400 for 200
    }
    otherwise -> {
        say "Invalid command: '{user_action}'. Type 'help' for options."
    }
}
```

### Multi-Line Branch Bodies
When a match arm requires executing more than one statement, wrap the branch body in curly braces `{ ... }`, as demonstrated above.

---

## 2. Under the Hood: How `match` Compiles

In naive compilers, `match` statements are simply translated into a chain of linear comparisons (`cmp rax, val; jne ...`).

The Runvoid code generator employs compiler optimizations for `match`:
1. **Dense Integer Jump Tables:** When matching against contiguous or dense integer ranges (e.g., numbers 0 through 10), the compiler constructs an indirect **jump table** in the read-only data section (`section .rodata`). Dispatch executes in **O(1) constant time** with a single indirect jump:
   ```nasm
   cmp rax, 10
   ja .L_fallback
   jmp [rel .L_jump_table + rax * 8]
   ```
2. **Sparse Binary Comparison Trees:** For scattered values, the compiler generates a balanced binary search comparison tree rather than a linear scan, reducing comparison operations from $O(N)$ to $O(\log N)$.

---

## 3. The Function Pipeline Operator (`|>`)

In traditional code, composing several functions requires wrapping them inside nested parentheses:

```runvoid
// Traditional nested function calls:
remember final_output = save_to_disk(validate_token(sanitize_input(raw_user_input)))
```

Notice the cognitive dissonance: to understand what happens first, your eyes must start in the deepest, innermost parenthesis (`raw_user_input`), then read backwards from right to left (`sanitize_input`), then outwards again (`validate_token`), and finally to the outermost function (`save_to_disk`).

The **pipeline operator (`|>`)** reverses this clutter, routing data from left to right in chronological order:

```runvoid
remember final_output = raw_user_input |> sanitize_input |> validate_token |> save_to_disk
```

### Practical Pipeline Example

```runvoid
action clean_string(s) {
    give replace "\r" with "" in s
}

action wrap_xml(s) {
    give "<payload>" + s + "</payload>"
}

action print_banner(msg) {
    say green "=== PROCESSED DATA ==="
    say msg
}

remember raw_data = "System Payload Ready"

// Stream the data through our transformation pipeline:
raw_data |> clean_string |> wrap_xml |> print_banner
```

Output:
```text
=== PROCESSED DATA ===
<payload>System Payload Ready</payload>
```

By chaining actions with `|>`, your business logic mirrors a clear assembly line, making complex transformations trivial to inspect, refactor, and test.

---

## 4. Hands-On Project: Interactive CLI Command Dispatcher

Let's combine `match` and pipelines to build an interactive command processor:

```runvoid
say cyan "=== COMMAND REPL ENGINE ==="

action trim_command(cmd) {
    make cmd trim
    give cmd
}

action dispatch_command(cmd) {
    match cmd {
        when "status" -> {
            say green "[STATUS] All 12 worker nodes operational. Uptime: 99.98%."
            beep 880 for 50
        }
        when "sync" -> {
            say yellow "[SYNC] Flushed 1,420 cached records to NVMe storage."
            beep 660 for 50
        }
        when "reboot" -> {
            say red "[WARN] Simulating cluster reboot in 5 seconds..."
            beep 330 for 150
        }
        when "help" -> {
            say "Available commands: status, sync, reboot, exit"
        }
        when "exit" -> {
            say "Exiting REPL session."
            give "EXIT"
        }
        otherwise -> {
            say red "Unknown command '{cmd}'. Type 'help' for options."
        }
    }
    give "CONTINUE"
}

remember test_input = "   status   "
test_input |> trim_command |> dispatch_command
```


