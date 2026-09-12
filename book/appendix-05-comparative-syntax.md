# Appendix E: Rosetta Stone Comparative Syntax Guide

For engineers transitioning to Runvoid from other mainstream programming languages—or for Runvoid programmers learning systems design—this Rosetta Stone maps identical idiomatic tasks across **Runvoid**, **C**, **Rust**, **Go**, **Python**, and **Lua**.

---

## 1. Hello World & Terminal Output

### Runvoid:
```runvoid
say green "Hello, World from Runvoid!"
```

### C (C11):
```c
#include <stdio.h>

int main(void) {
    printf("\033[32mHello, World from C!\033[0m\n");
    return 0;
}
```

### Rust (2021 Edition):
```rust
fn main() {
    println!("\x1b[32mHello, World from Rust!\x1b[0m");
}
```

### Go:
```go
package main

import "fmt"

func main() {
    fmt.Println("\033[32mHello, World from Go!\033[0m")
}
```

### Python:
```python
print("\033[32mHello, World from Python!\033[0m")
```

### Lua:
```lua
print("\033[32mHello, World from Lua!\033[0m")
```

---

## 2. Variables & Mutability

| Feature | Runvoid | Rust | Go | C | Python |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Declaration** | `remember x = 10` | `let mut x = 10;` | `var x int = 10` | `int x = 10;` | `x = 10` |
| **Constant** | `remember const C = 42` | `const C: i32 = 42;` | `const C = 42` | `const int C = 42;`| `C = 42` (convention) |
| **Strict Type** | `remember x: Int = 10` | `let x: i64 = 10;` | `var x int64 = 10` | `int64_t x = 10;` | `x: int = 10` |

---

## 3. Functions & Actions

### Runvoid:
```runvoid
action calculate_tax(subtotal: Float, rate: Float) -> Float {
    remember total = subtotal * (1.0 + rate)
    give total
}
```

### Rust:
```rust
fn calculate_tax(subtotal: f64, rate: f64) -> f64 {
    subtotal * (1.0 + rate)
}
```

### Go:
```go
func calculateTax(subtotal float64, rate float64) float64 {
    return subtotal * (1.0 + rate)
}
```

### C:
```c
double calculate_tax(double subtotal, double rate) {
    return subtotal * (1.0 + rate);
}
```

### Python:
```python
def calculate_tax(subtotal: float, rate: float) -> float:
    return subtotal * (1.0 + rate)
```

---

## 4. Collections: Arrays & Dynamic Lists

### Runvoid:
```runvoid
remember numbers = [10, 20, 30]
add 40 to numbers
remember length = count numbers
remember first = numbers[0]
```

### Rust:
```rust
let mut numbers = vec![10, 20, 30];
numbers.push(40);
let length = numbers.len();
let first = numbers[0];
```

### Go:
```go
numbers := []int{10, 20, 30}
numbers = append(numbers, 40)
length := len(numbers)
first := numbers[0]
```

### Python:
```python
numbers = [10, 20, 30]
numbers.append(40)
length = len(numbers)
first = numbers[0]
```

---

## 5. Dictionaries & Key-Value Hash Maps

### Runvoid:
```runvoid
remember config = {
    "host": "127.0.0.1",
    "port": 8080
}
add "timeout": 30 to config

if config has "host" {
    say "Host is: {config[\"host\"]}"
}
```

### Rust:
```rust
use std::collections::HashMap;

let mut config = HashMap::new();
config.insert("host", "127.0.0.1");
config.insert("port", "8080");

if let Some(host) = config.get("host") {
    println!("Host is: {}", host);
}
```

### Go:
```go
config := map[string]string{
    "host": "127.0.0.1",
    "port": "8080",
}
config["timeout"] = "30"

if host, ok := config["host"]; ok {
    fmt.Printf("Host is: %s\n", host)
}
```

### Python:
```python
config = {
    "host": "127.0.0.1",
    "port": 8080
}
config["timeout"] = 30

if "host" in config:
    print(f"Host is: {config['host']}")
```

---

## 6. Systems Programming & Manual Memory (Pointers)

### Runvoid (Pro Mode):
```runvoid
remove garbageC
remove Basic
add Advanced
use mem

remember buffer: Ptr = alloc(1024)
// Use raw memory...
free(buffer)
```

### C:
```c
#include <stdlib.h>

void* buffer = malloc(1024);
// Use raw memory...
free(buffer);
```

### Rust:
```rust
use std::alloc::{alloc, dealloc, Layout};

unsafe {
    let layout = Layout::from_size_align(1024, 8).unwrap();
    let ptr = alloc(layout);
    // Use raw memory...
    dealloc(ptr, layout);
}
```

---

## 7. Pipeline Operator (`|>`)

### Runvoid:
```runvoid
remember result = 5 |> double |> square |> increment
```

### Elixir:
```elixir
result = 5 |> double() |> square() |> increment()
```

### F#:
```fsharp
let result = 5 |> double |> square |> increment
```

### Python (Traditional nested calls):
```python
result = increment(square(double(5)))
```

Runvoid combines the readability of human-oriented keywords with the uncompromised execution speed of compiled native machine code!
