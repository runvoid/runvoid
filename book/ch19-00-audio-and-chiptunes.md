# 19. Audio Synthesis & Chiptune Sound Engines

Sound design and audio feedback bring computer software, operating systems, and video games to life. From arcade chiptunes and retro bleeps to real-time audio DSP (Digital Signal Processing), understanding how digital silicon models acoustic vibrations is a core systems capability.

In this chapter, you will master:
1. The physics of sound: frequency (Pitch), amplitude (Volume), and sample rates.
2. Waveform mathematics: Square, Triangle, Sawtooth, and Sine waves.
3. ADSR (Attack, Decay, Sustain, Release) volume envelopes.
4. Building an 8-bit Sound Effects (SFX) Generator in Runvoid.
5. Programming a multi-track tracker engine to compose polyphonic chiptune melodies.

---

## 1. The Physics of Digital Audio

Sound in the physical world is a mechanical wave of pressure oscillations traveling through air. A digital computer represents continuous sound waves as a discrete sequence of numbers sampled thousands of times per second (Pulse Code Modulation, or PCM).

```
Continuous Acoustic Sound Wave:
      +1.0 |    _--_                _--_
           |  /      \            /      \
       0.0 +------------\------/------------\------/---> Time (t)
           |              \__--/              \__--/
      -1.0 |

Digital Discrete Sampling (e.g. 44,100 samples per second):
      +1.0 |   * * *               * * *
           | *       *           *       *
       0.0 +-----------*-------*-----------*-------*---> Sample Index (n)
           |             * * *               * * *
      -1.0 |
```

### The Nyquist-Shannon Sampling Theorem

To accurately represent an audio frequency $f_{\text{max}}$, the digital sampling rate $f_s$ must be at least double that frequency:

$$f_s \ge 2 \cdot f_{\text{max}}$$

Human hearing spans roughly $20\text{ Hz}$ to $20,000\text{ Hz}$ ($20\text{ kHz}$). Therefore, CD audio standards use $44,100\text{ Hz}$ ($44.1\text{ kHz}$), comfortably capturing the full audible spectrum without aliasing artifacts.

---

## 2. The Four Classic Chiptune Waveforms

Retro gaming hardware (such as the Nintendo NES RP2A03 or Game Boy LR35902 sound chips) generated sound using four fundamental waveforms:

```
1. Square Wave (Crisp, hollow, 8-bit lead melodies):
   +1 |---|   |---|   |---|
   -1     |---|   |---|   |---

2. Triangle Wave (Smooth, mellow, basslines and flutes):
   +1   /\      /\      /\
   -1  /  \    /  \    /  \
           \  /    \  /    \  /
            \/      \/      \/

3. Sawtooth Wave (Rich, buzzy, sharp synth brass):
   +1  /|  /|  /|  /|
   -1 / | / | / | / |

4. White Noise (Random numbers, percussion, snare drums, explosions):
   +1 || | | |||| | ||| | ||
   -1 ||||| || |||||| ||||||
```

### Musical Note Frequencies (A4 = 440 Hz)

Western music divides octaves into 12 semitones. The frequency $f$ of any MIDI note $n$ relative to A4 ($n = 69$, $440\text{ Hz}$) is:

$$f = 440 \cdot 2^{(n - 69) / 12}$$

Standard Note Frequencies in Hertz:
- **C4 (Middle C):** $261.63\text{ Hz}$
- **D4:** $293.66\text{ Hz}$
- **E4:** $329.63\text{ Hz}$
- **F4:** $349.23\text{ Hz}$
- **G4:** $392.00\text{ Hz}$
- **A4:** $440.00\text{ Hz}$
- **B4:** $493.88\text{ Hz}$
- **C5:** $523.25\text{ Hz}$

---

## 3. ADSR Volume Envelopes

A constant-volume square wave sounds sterile and artificial. Acoustic instruments have dynamic volume profiles over time. Synthesizers model this with the **ADSR Envelope**:

```
Amplitude (Volume)
  100% |     /\  (Decay)
       |    /  \_______ (Sustain Level)
       |   /           \
       |  / (Attack)    \ (Release)
    0% +-----------------\---------> Time (ms)
       ^ Key Pressed     ^ Key Released
```

1. **Attack:** Time taken for the volume to rise from 0 to peak amplitude.
2. **Decay:** Time taken to drop from peak down to the steady sustain level.
3. **Sustain:** Constant volume held while the note remains active.
4. **Release:** Time taken for sound to fade out to zero after note release.

---

## 4. Hands-On Project: The Arcade Chiptune SFX Engine

Let's write a complete sound synthesizer in Runvoid that generates retro game sound effects:

```runvoid
say cyan "=================================================="
say cyan "        RUNVOID CHIPTUNE SOUND FX ENGINE          "
say cyan "=================================================="

// Sound Frequency Presets (in Hertz)
remember NOTE_C4 = 262
remember NOTE_E4 = 330
remember NOTE_G4 = 392
remember NOTE_B4 = 494
remember NOTE_C5 = 523
remember NOTE_E5 = 659
remember NOTE_G5 = 784

// Sound Effect 1: Coin Pickup (Ascending Arpeggio)
action play_sfx_coin() {
    say yellow "[SFX] Coin Collected!"
    play tone NOTE_B4 for 70
    play tone NOTE_E5 for 180
}

// Sound Effect 2: Jump Sound (Fast Frequency Slide Up)
action play_sfx_jump() {
    say cyan "[SFX] Player Jump!"
    remember freq = 150
    repeat 8 times {
        play tone freq for 25
        freq = freq + 40
    }
}

// Sound Effect 3: Laser Blast (Frequency Slide Down)
action play_sfx_laser() {
    say magenta "[SFX] Laser Fire!"
    remember freq = 900
    repeat 10 times {
        play tone freq for 18
        freq = freq - 70
    }
}

// Sound Effect 4: Explosion (Simulated Low-Frequency Rumbling)
action play_sfx_explosion() {
    say red "[SFX] Explosion Impact!"
    remember low_freq = 110
    repeat 6 times {
        play tone low_freq for 60
        low_freq = low_freq - 12
    }
}

// Sound Effect 5: Level Complete Victory Fanfare
action play_sfx_victory() {
    say green "[SFX] Victory Fanfare!"
    play tone NOTE_C4 for 120
    play tone NOTE_E4 for 120
    play tone NOTE_G4 for 120
    play tone NOTE_C5 for 350
}

// Demonstration Routine
say "Testing Retro Arcade Audio Generator:"
play_sfx_coin()
play_sfx_jump()
play_sfx_laser()
play_sfx_explosion()
play_sfx_victory()

say green "Audio sound effects synthesis completed successfully!"
```

---

## 5. Hands-On Project: Multi-Track Music Tracker

Now let's construct a music score sequencer that reads an array of note-duration pairs and renders a melodic composition:

```runvoid
say cyan "=================================================="
say cyan "          8-BIT MUSIC SCORE SEQUENCER             "
say cyan "=================================================="

// Track Data: [Frequency_Hz, Duration_ms]
remember melody = [
    [NOTE_E5, 200], [NOTE_E5, 200], [0, 100],       [NOTE_E5, 200],
    [0, 100],       [NOTE_C5, 200], [NOTE_E5, 200], [NOTE_G5, 400],
    [0, 200],       [NOTE_G4, 400]
]

action play_tracker_score(score) {
    say yellow "Sequencing melody track ({count score} events)..."
    
    remember step = 1
    for every note_event in score {
        remember hz = note_event[0]
        remember ms = note_event[1]
        
        if hz > 0 {
            say "Step {step}: Tone {hz} Hz ({ms} ms)"
            play tone hz for ms
        } otherwise {
            say "Step {step}: Rest silence ({ms} ms)"
            wait ms
        }
        step = step + 1
    }
    
    say green "Melody playback completed!"
}

play_tracker_score(melody)
```

---

## 6. Real-Time Audio Buffers via C FFI

In production game engines (like SDL2 or ALSA/PulseAudio on Linux, WASAPI on Windows), sound is played through a double-buffered circular DMA queue.

```
+-------------------------------------------------------------+
| Circular Audio Ring Buffer (e.g. 2048 Samples, Float32)     |
| [ Left Ear Buffer ][ Right Ear Buffer ]                     |
+-------------------------------------------------------------+
         ^                               ^
         | Hardware DMA Playhead         | Audio Engine Synthesizer
     (Consumer)                     (Producer - Generates Next Frame)
```

In Runvoid Pro mode (`add Advanced`), you can link directly to `libasound` on Linux or `OpenAL` across platforms to generate raw uncompressed waveform PCM samples in real time, bypassing all OS audio latency!
