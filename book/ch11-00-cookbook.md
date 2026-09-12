# 11. The Runvoid Cookbook

The Runvoid Cookbook presents battle-tested, production-ready code recipes illustrating real-world application patterns across systems, networking, audio, data structures, and graphics.

All recipes can be found in `examples/cookbook/` within the repository.

---

## Recipe 01: REST API Client & JSON Data Modeling

Demonstrates modeling HTTP request headers, state payloads, and response validation using Runvoid dictionaries (maps) and string interpolation:

```runvoid
// examples/cookbook/01_rest_api_client.rv
say cyan "=== Runvoid REST API Client Simulator ==="

remember request_headers = {
    "User-Agent": "Runvoid-Client/1.3.0",
    "Accept": "application/json",
    "Authorization": "Bearer rv_sec_token_987654321"
}

remember api_endpoint = "https://api.runvoid.org/v1/telemetry"
say yellow "Preparing POST request to: {api_endpoint}"
say "Headers configured: {count request_headers}"
remember token = request_headers["Authorization"]
say "Auth token: {token}"

remember payload = {
    "client_name": "AntigravityNode",
    "status": "online",
    "uptime_seconds": 3600,
    "active_threads": 8
}

say "Sending payload with status: {payload's status}"

remember response = {
    "status_code": 200,
    "message": "Telemetry accepted",
    "region": "us-east-1",
    "rate_limit_remaining": 994
}

remember code = response["status_code"]
if code == 200 {
    say green "SUCCESS (200 OK): {response's message}"
    say "Served by region: {response's region}"
    say "Remaining API calls: {response's rate_limit_remaining}"
} otherwise {
    say red "ERROR: Unexpected status code: {code}"
}

add "last_synced": 1726130000 to payload
say green "Payload cached successfully with {count payload} fields."
```

---

## Recipe 02: 8-Bit Melody Synthesizer & Tone Generator

Demonstrates programmable frequency generation (`play synth`), note mapping, and musical synthesis:

```runvoid
// examples/cookbook/02_melody_synthesizer.rv
say cyan "=== Runvoid 8-Bit Melody Synthesizer ==="

remember NOTE_C4 = 261
remember NOTE_D4 = 294
remember NOTE_E4 = 329
remember NOTE_F4 = 349
remember NOTE_G4 = 392
remember NOTE_A4 = 440
remember NOTE_B4 = 493
remember NOTE_C5 = 523

say "   +---+---+---+---+---+---+---+---+"
say "   | C | D | E | F | G | A | B | C |"
say "   +---+---+---+---+---+---+---+---+"
say "   |261|294|329|349|392|440|493|523|"
say "   +---+---+---+---+---+---+---+---+"

say yellow "Synthesizing Arcade Victory Fanfare..."

play synth NOTE_C4, 120
play synth NOTE_E4, 120
play synth NOTE_G4, 120
play synth NOTE_C5, 300

say green "Fanfare playback complete!"
```

---

## Recipe 03: Retro Snake Game (2D Hardware Canvas)

Demonstrates game loop state, rendering boundaries, snake segments, apples, and HUD text onto the hardware accelerated 2D screen:

```runvoid
// examples/cookbook/03_snake_game.rv
say cyan "=== Launching Runvoid Retro Snake Game Demo ==="

remember score = 150
remember high_score = 420

screen "Runvoid Retro Snake - 2D Canvas", 640, 480 {
    draw box at 0, 0, size 640, 480, color "black"

    // Render boundary walls
    draw line from 20, 40, to 620, 40, color "white"
    draw line from 20, 440, to 620, 440, color "white"
    draw line from 20, 40, to 20, 440, color "white"
    draw line from 620, 40, to 620, 440, color "white"

    // Render Snake Body Segments
    draw box at 200, 200, size 20, 20, color "green"
    draw box at 220, 200, size 20, 20, color "green"
    draw box at 240, 200, size 20, 20, color "green"
    draw box at 260, 200, size 20, 20, color "green"
    draw box at 280, 200, size 20, 20, color "cyan"

    // Food
    draw circle at 400, 200, size 10, color "red"

    // HUD Display
    draw text "RUNVOID RETRO SNAKE", at 30, 25, color "green"
    draw text "SCORE: 150", at 300, 25, color "yellow"
    draw text "HIGH SCORE: 420", at 480, 25, color "white"
    draw text "Press any key to return to desktop...", at 180, 460, color "cyan"
}
```

---

## Recipe 04: Bare-Metal Freestanding Kernel Entry

Demonstrates emitting a freestanding x86_64 binary with no `libc`, no runtime, and invoking raw Linux system calls directly:

```runvoid
// examples/cookbook/04_freestanding_kernel.rv
remove garbageC
remove Basic
remove Linux
add Advanced
add Freestanding

asm {
    jmp kernel_start

    msg db "[Kernel] Runvoid Freestanding Bare-Metal Kernel Loaded!", 10
    len equ $ - msg

kernel_start:
    ; Invoke sys_write (1)
    mov rax, 1
    mov rdi, 1
    lea rsi, [rel msg]
    mov rdx, len
    syscall

    ; Invoke sys_exit (60)
    mov rax, 60
    xor rdi, rdi
    syscall
}
```

---

## Recipe 05: High-Speed XOR File Encryption / Decryption

Demonstrates encrypting and decrypting data using symmetric bitwise XOR operations:

```runvoid
say cyan "=== XOR Symmetric Stream Cipher ==="

remember key = 0x5A  // 8-bit symmetric key

action xor_cipher(input_val: Int, key_val: Int): Int {
    give input_val bit xor key_val
}

remember plaintext = 1337
remember ciphertext = xor_cipher(plaintext, key)
remember decrypted = xor_cipher(ciphertext, key)

say "Plaintext:  {plaintext}"
say "Ciphertext: {ciphertext}"
say "Decrypted:  {decrypted}"
```

---

## Recipe 06: Command-Line Flag Dispatcher

Demonstrates parsing command-line modes and executing subroutines:

```runvoid
say cyan "=== Tool CLI Dispatcher ==="

remember mode = "build"

if mode == "build" {
    say green "Executing build pipeline..."
} otherwise if mode == "test" {
    say yellow "Running automated verification suites..."
} otherwise if mode == "clean" {
    say "Cleaning build artifacts..."
} otherwise {
    say red "Unknown mode: {mode}"
}
```

---

## Recipe 07: High-Precision Code Stopwatch

Demonstrates measuring statistical execution times across multiple runs:

```runvoid
say cyan "=== Statistical Benchmark Stopwatch ==="

remember iterations = 5
remember total_time = 0

repeat iterations times as run_idx {
    measure time {
        remember acc = 0
        repeat 1000000 times as n {
            acc = acc + n
        }
    }
}
say green "Benchmark passes completed."
```

---

## Recipe 08: Circular Ring Buffer for High-Throughput Streaming

Demonstrates implementing a fixed-size $O(1)$ ring buffer using bitwise mask indexing ($N$ is a power of 2):

```runvoid
say cyan "=== Circular Ring Buffer Engine ==="

remember CAPACITY = 8
remember MASK = CAPACITY - 1  // 7 (0b0111)

remember ring_buffer = [0, 0, 0, 0, 0, 0, 0, 0]
remember head = 0
remember tail = 0

action ring_push(item) {
    remember slot = head bit and MASK
    ring_buffer[slot] = item
    head = head + 1
    say "Pushed item '{item}' into slot {slot}. Head: {head}"
}

action ring_pop() {
    if tail >= head {
        say red "Buffer underflow! Empty queue."
        give 0
    }
    remember slot = tail bit and MASK
    remember item = ring_buffer[slot]
    tail = tail + 1
    say "Popped item '{item}' from slot {slot}. Tail: {tail}"
    give item
}

repeat 10 times as i {
    ring_push(100 + i)
}

repeat 5 times {
    remember val = ring_pop()
}
say green "Ring buffer state verified."
```

---

## Recipe 09: Binary Search Tree (BST) & In-Order Traversal

Demonstrates implementing a hierarchical tree structure with ordered node insertions:

```runvoid
say cyan "=== Binary Search Tree (BST) ==="

action make_node(val) {
    remember node = {
        "val": val,
        "left": 0,
        "right": 0
    }
    give node
}

action insert_bst(root, val) {
    if val < root["val"] {
        if root["left"] == 0 {
            root["left"] = make_node(val)
        } otherwise {
            insert_bst(root["left"], val)
        }
    } otherwise {
        if root["right"] == 0 {
            root["right"] = make_node(val)
        } otherwise {
            insert_bst(root["right"], val)
        }
    }
}

remember bst_root = make_node(50)
insert_bst(bst_root, 30)
insert_bst(bst_root, 70)
insert_bst(bst_root, 20)
insert_bst(bst_root, 40)

say green "Constructed BST hierarchy rooted at {bst_root[\"val\"]}."
```

---

## Recipe 10: 2D Matrix Multiplication & Linear Algebra

Demonstrates multiplying two $2 \times 2$ matrices for 2D graphics transformations:

```runvoid
say cyan "=== 2D Matrix Linear Transformation ==="

// Matrix A: [[2, 0], [0, 3]] (Scale x by 2, y by 3)
remember a00 = 2
remember a01 = 0
remember a10 = 0
remember a11 = 3

// Vector V: [x: 10, y: 5]
remember vx = 10
remember vy = 5

// Transformed V' = A * V
remember out_x = (a00 * vx) + (a01 * vy)
remember out_y = (a10 * vx) + (a11 * vy)

say "Input vector:  ({vx}, {vy})"
say green "Transformed:   ({out_x}, {out_y})"
```

---

## Recipe 11: Token Bucket Rate Limiter for Microservices

Demonstrates protecting backend endpoints against traffic spikes using a leaky token bucket:

```runvoid
say cyan "=== Token Bucket Rate Limiter ==="

remember MAX_TOKENS = 5
remember refill_rate_per_sec = 2
remember current_tokens = MAX_TOKENS

action try_consume_token() {
    if current_tokens > 0 {
        current_tokens = current_tokens - 1
        say green "Request PERMITTED. Remaining tokens: {current_tokens}"
        give true
    } otherwise {
        say red "Request 429 THROTTLED: Rate limit exceeded!"
        give false
    }
}

repeat 7 times {
    try_consume_token()
}
```
