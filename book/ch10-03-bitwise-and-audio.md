# 10.3 Bitwise Operations & Audio Synthesizer

For systems programmers, embedded developers, and game designers, Runvoid 0.3.0 brings native bitwise instructions and frequency audio synthesis.

## Bitwise Operators

Runvoid supports both natural English keywords and standard symbols for bit manipulation:

| Operation | English Keyword | Symbol | Native x86_64 Instruction |
|---|---|---|---|
| Bitwise AND | `a bit and b` | `a & b` | `and rbx, rax` |
| Bitwise OR | `a bit or b` | `a \| b` | `or rbx, rax` |
| Bitwise XOR | `a bit xor b` | `a ^ b` | `xor rbx, rax` |
| Shift Left | `a shift left b` | `a << b` | `shl rbx, cl` |
| Shift Right | `a shift right b` | `a >> b` | `shr rbx, cl` |
| Bitwise NOT | `~a` | `~a` | `not rax` |

### Example

```runvoid
remember flags = 0
remember READ_FLAG = 1 shift left 0    # 1
remember WRITE_FLAG = 1 shift left 1   # 2
remember EXEC_FLAG = 1 shift left 2    # 4

// Enable read and write
remember my_perms = READ_FLAG bit or WRITE_FLAG

// Check if write is permitted
if (my_perms bit and WRITE_FLAG) != 0 {
    say green "Write permission enabled!"
}
```

The optimizer performs constant-folding at compile-time on constant bitwise expressions with zero runtime overhead.

## Sound Synthesizer (`play synth`)

Runvoid 0.3.0 includes a built-in square-wave synthesizer that emits tones directly via system audio:

```runvoid
// Play an A4 tone (440 Hz) for 300 milliseconds
play synth 440, 300

// Simple melody
play synth 523, 150 # C5
play synth 587, 150 # D5
play synth 659, 300 # E5
```
