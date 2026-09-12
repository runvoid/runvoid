# 10.3 Bitwise Operations & Audio Synthesizer

Systems programmers, game designers, and network engineers frequently need to pack data into compact bitfields, manipulate hardware registers, and synthesize audio feedback.

Runvoid 1.3 provides native hardware bitwise operations and a programmable square-wave frequency sound generator.

---

## 1. Native Bitwise Operations

Runvoid supports both natural English phrases and standard programming symbols for bit manipulation:

| Operation | Conversational Syntax | Symbolic Syntax | Emitted x86_64 Instruction | Description |
| :--- | :--- | :--- | :--- | :--- |
| **Bitwise AND** | `a bit and b` | `a & b` | `and rbx, rax` | Bits set in both operands |
| **Bitwise OR** | `a bit or b` | `a \| b` | `or rbx, rax` | Bits set in either operand |
| **Bitwise XOR** | `a bit xor b` | `a ^ b` | `xor rbx, rax` | Bits set in exactly one operand |
| **Shift Left** | `a shift left b` | `a << b` | `shl rbx, cl` | Logical shift bits to the left |
| **Shift Right** | `a shift right b`| `a >> b` | `shr rbx, cl` | Logical shift bits to the right |
| **Bitwise NOT** | `~a` | `~a` | `not rax` | Invert all 64 bits (one's complement) |

---

## 2. Practical Systems Example: Bitmask Permissions

Bitwise operations allow packing multiple boolean flags into a single 64-bit integer:

```runvoid
// Define permission bitmasks:
remember PERM_READ    = 1 shift left 0  // 0b0001 (1)
remember PERM_WRITE   = 1 shift left 1  // 0b0010 (2)
remember PERM_EXEC    = 1 shift left 2  // 0b0100 (4)
remember PERM_ADMIN   = 1 shift left 3  // 0b1000 (8)

// Grant Read and Execute permissions using Bitwise OR:
remember user_perms = PERM_READ bit or PERM_EXEC

// Check if Write permission is set using Bitwise AND:
if (user_perms bit and PERM_WRITE) != 0 {
    say green "Write access granted."
} otherwise {
    say red "Permission Denied: Write access required!"
}

// Grant Admin access dynamically:
user_perms = user_perms bit or PERM_ADMIN
say "Updated bitmask: {user_perms}"
```

### Packing Values
You can pack two 32-bit integers (such as 2D coordinate `(x, y)`) into a single 64-bit register:

```runvoid
action pack_coordinates(x: Int, y: Int): Int {
    remember high_bits: Int = x shift left 32
    give high_bits bit or (y bit and 0xFFFFFFFF)
}

action unpack_x(packed: Int): Int {
    give packed shift right 32
}

action unpack_y(packed: Int): Int {
    give packed bit and 0xFFFFFFFF
}
```

---

## 3. The Programmable Sound Synthesizer (`play synth`)

While the basic `beep` statement triggers a simple terminal bell or fixed-frequency alert, `play synth` gives you access to a programmable tone generator:

```runvoid
play synth frequency_hz, duration_ms
```

- **Frequency:** Tone frequency specified in Hertz (e.g., 440 Hz for Concert A).
- **Duration:** Duration of tone specified in milliseconds.

### Playing an 8-Bit Arcade Victory Fanfare

```runvoid
say cyan "=== LEVEL CLEAR ==="

// Play ascending arpeggio (C5 -> E5 -> G5 -> C6):
play synth 523, 150  // C5
play synth 659, 150  // E5
play synth 784, 150  // G5
play synth 1046, 350 // C6

say green "Victory Fanfare complete!"
```

### Frequency Table Reference for Game Audio

| Musical Note | Frequency (Hz) | Usage / Tone |
| :--- | :--- | :--- |
| **C4 (Middle C)**| `261 Hz` | Neutral UI confirmation |
| **A4 (Concert A)**| `440 Hz` | Standard tuning pitch |
| **C5** | `523 Hz` | High confirmation tone |
| **E5** | `659 Hz` | Upbeat melody |
| **G5** | `784 Hz` | Victory arpeggio |
| **Low Warning** | `150 Hz` | Error buzz / damage sound |

Under the hood, the runtime uses hardware audio APIs:
- **On Windows:** Uses the Win32 `Beep(frequency, duration)` driver from `kernel32.dll`.
- **On Linux:** Interacts with the console audio subsystem or synthesizes PCM waveform bursts.

---

## 4. Systems Bit Hacks: Colors & Bit Counting

### 1. Packing 24-bit RGB Colors
In graphics programming, color values `(red, green, blue)` from 0 to 255 are packed into a single 32-bit integer:

```runvoid
action pack_rgb(r: Int, g: Int, b: Int): Int {
    remember r_shifted: Int = r shift left 16
    remember g_shifted: Int = g shift left 8
    give r_shifted bit or g_shifted bit or b
}

remember neon_cyan: Int = pack_rgb(0, 255, 255)
say "Packed 24-bit RGB Color: {neon_cyan}" // 0x00FFFF = 65535
```

### 2. Checking if a Number is a Power of Two
A classic binary trick: a positive integer `n` is a power of 2 if and only if `n & (n - 1) == 0`:

```runvoid
action is_power_of_two(n: Int): Int {
    if n <= 0 { give 0 }
    if (n bit and (n - 1)) == 0 {
        give 1
    }
    give 0
}

say "Is 64 power of 2? {is_power_of_two(64)}" // 1 (true)
say "Is 60 power of 2? {is_power_of_two(60)}" // 0 (false)
```

---

## 5. Hands-On Project: 8-Bit Chiptune Player ("Ode to Joy")

Let's synthesize a classic melodic sequence using `play synth`:

```runvoid
say cyan "=== SYNTHESIZING: ODE TO JOY ==="

remember NOTE_E4 = 329
remember NOTE_F4 = 349
remember NOTE_G4 = 392
remember NOTE_D4 = 294
remember NOTE_C4 = 261

// Measure 1: E E F G
play synth NOTE_E4, 200
play synth NOTE_E4, 200
play synth NOTE_F4, 200
play synth NOTE_G4, 200

// Measure 2: G F E D
play synth NOTE_G4, 200
play synth NOTE_F4, 200
play synth NOTE_E4, 200
play synth NOTE_D4, 200

// Measure 3: C C D E
play synth NOTE_C4, 200
play synth NOTE_C4, 200
play synth NOTE_D4, 200
play synth NOTE_E4, 300

say green "Playback complete!"
```


