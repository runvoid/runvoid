# 11. The Runvoid Cookbook

The Runvoid Cookbook presents battle-tested, production-ready code recipes illustrating real-world application patterns across systems, networking, audio, and graphics.

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
