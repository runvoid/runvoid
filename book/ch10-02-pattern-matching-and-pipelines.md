# 10.2 Pattern Matching & Pipelines

Runvoid 0.3.0 introduces functional constructs that make data transformation and branch dispatch clean, readable, and lightning-fast.

## Pattern Matching (`match`)

Instead of cascading `if` / `otherwise` chains, use `match` blocks to test values against constant patterns:

```runvoid
remember status_code = 404

match status_code {
    when 200 -> say green "OK - Request successful"
    when 400 -> say yellow "Bad Request"
    when 404 -> say red "Not Found"
    when 500 -> say red "Internal Server Error"
    otherwise -> say "Unhandled status code: {status_code}"
}
```

Pattern matching also supports strings and booleans:

```runvoid
remember command = "start"

match command {
    when "start" -> say "System starting..."
    when "stop"  -> say "System stopping..."
    otherwise    -> say "Unknown command"
}
```

## Function Pipeline Operator (`|>`)

The pipeline operator forwards the result of the left-hand expression as the first argument to the right-hand function call:

```runvoid
action double(x: Int): Int {
    give x * 2
}

action add_five(x: Int): Int {
    give x + 5
}

// Without pipeline:
remember res1 = add_five(double(10))

// With pipeline:
remember res2 = 10 |> double |> add_five
say "Result: {res2}" # Prints 25
```

Pipelines eliminate deeply nested parentheses and allow code to be read in the exact chronological order of execution.
