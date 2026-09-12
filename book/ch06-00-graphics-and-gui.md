# 6. Graphics & Windowing: 2D Canvas & GUI

Modern developers often need visual output—whether for 2D arcade games, visual data simulations, computer-aided design (CAD), or graphical administrative dashboards.

Runvoid includes built-in 2D canvas rendering and declarative desktop windowing directly in its standard runtime. You do not need to configure massive third-party graphical toolkits like Qt, GTK, or SDL; your graphical applications compile into single, lightweight native executables.

---

## 1. The 2D Screen Canvas

The `screen` block opens an accelerated 2D graphics viewport:

```runvoid
screen "Arcade 2D", 640, 480 {
    draw box at 0, 0, size 640, 480, color "black"
    draw circle at 320, 240, size 50, color "cyan"
    draw line from 0, 0 to 640, 480, color "red"
    draw text "Runvoid Engine 2D", at 220, 50, color "yellow"
}
```

### Canvas Primitive Reference
The 2D canvas provides five fundamental drawing primitives:

| Primitive | Syntax | Description |
| :--- | :--- | :--- |
| **Box / Rectangle** | `draw box at x, y, size w, h, color "..."` | Filled rectangle at `(x, y)` with width `w` and height `h` |
| **Circle** | `draw circle at x, y, size radius, color "..."` | Filled circle centered at `(x, y)` with radius `radius` |
| **Line** | `draw line from x1, y1 to x2, y2, color "..."` | Anti-aliased line between two coordinate points |
| **Pixel** | `draw pixel at x, y, color "..."` | Sets a single individual pixel in the framebuffer |
| **Text** | `draw text "...", at x, y, color "..."` | Renders a text string at specified coordinates |

### Supported Color Formats
Colors can be specified as named CSS-style strings (`"red"`, `"blue"`, `"green"`, `"cyan"`, `"magenta"`, `"yellow"`, `"white"`, `"black"`) or as 24-bit hex RGB values (`"#FF8800"`, `"#1E1E2E"`).

---

## 2. Low-Level Rasterization: Bresenham's Line Algorithm

How does a graphics card or software rasterizer draw a diagonal line across a grid of discrete pixels? Rather than expensive floating-point trigonometric calculations, graphics engines use **Bresenham's Integer Line Algorithm**:

```
Grid of discrete display pixels:
(x0, y0)
  [*] [ ] [ ] [ ]
  [ ] [*] [*] [ ]  <-- Increments y when integer error accumulator >= dx
  [ ] [ ] [ ] [*] (x1, y1)
```

In Runvoid, Bresenham's algorithm is implemented purely with fast integer addition, subtraction, and bit shifts:

```runvoid
action rasterize_line(x0, y0, x1, y1, color_name) {
    remember dx = abs(x1 - x0)
    remember dy = 0 - abs(y1 - y0)
    remember sx = 1
    if x0 > x1 { sx = -1 }
    remember sy = 1
    if y0 > y1 { sy = -1 }
    remember err = dx + dy

    while 1 == 1 {
        draw pixel at x0, y0, color color_name
        if x0 == x1 and y0 == y1 {
            give 0
        }
        remember e2 = 2 * err
        if e2 >= dy {
            err = err + dy
            x0 = x0 + sx
        }
        if e2 <= dx {
            err = err + dx
            y0 = y0 + sy
        }
    }
}
```

This algorithm executes in zero allocations with $O(N)$ pixel writes, making it extraordinarily fast!

---

## 3. Interactive Game Loops & Animation

To build dynamic simulations or games, you can animate objects by updating coordinates inside a loop:

```runvoid
screen "Bouncing Orb", 800, 600 {
    remember x = 100
    remember y = 100
    remember dx = 4
    remember dy = 3
    remember radius = 20

    repeat 500 times {
        // Clear background:
        draw box at 0, 0, size 800, 600, color "black"

        // Update physics:
        x = x + dx
        y = y + dy

        // Bounce on boundary collisions:
        if x <= radius or x >= 800 - radius {
            dx = 0 - dx
            beep 600 for 30
        }
        if y <= radius or y >= 600 - radius {
            dy = 0 - dy
            beep 750 for 30
        }

        // Draw orb:
        draw circle at x, y, size radius, color "cyan"
    }
}
```

The runtime automatically synchronizes frames, handles double buffering to eliminate screen tearing, and routes window close events gracefully.

---

## 4. Declarative Desktop Windows (GUI)

For utility panels, configuration screens, and administrative dashboards, Runvoid provides the `window` declarative block:

```runvoid
window "System Diagnostics", 480, 320 {
    label "Runvoid Native Host Diagnostics"
    
    checkbox "Enable Telemetry Logging", 1
    checkbox "Hardware GPU Acceleration", 1
    checkbox "Audio Beep Notifications", 0

    button "Run Diagnostics" {
        say green "Running hardware audit..."
        alert "System Audit Complete: 0 warnings, 0 errors."
    }

    button "Close" {
        stop
    }
}
```

### Supported GUI Components:
- **`label "text"`**: Renders a read-only text header or status label.
- **`checkbox "label", default_checked`**: Renders a toggle checkbox (`1` = checked, `0` = unchecked).
- **`button "label" { ... }`**: Renders a clickable native button with an attached event handler block.

---

## 5. Under the Hood: Cross-Platform Windowing

The Runvoid runtime adapts its graphics pipeline to the target operating system:
- **On Linux:** Connects to the local X11 display server (`XOpenDisplay`, `XCreateSimpleWindow`, `XSelectInput`, `XPutImage`) with zero third-party framework overhead.
- **On Windows:** Registers a native Win32 window class (`RegisterClassExA`), invokes `CreateWindowExA`, and processes messages via `GetMessage` / `DispatchMessageA`, utilizing the fast Win32 GDI subsystem.

This design gives your Runvoid graphical applications instant startup times (under 2 milliseconds) and negligible memory consumption!

```
                    Double Buffering Frame Pipeline:
   +---------------------------------------+
   | Offscreen Backbuffer (in RAM)         |  <--- 1. Draw box, circle, line
   +---------------------------------------+
                      |
                      | 2. Swap Buffers (XPutImage / BitBlt)
                      v
   +---------------------------------------+
   | Onscreen Frontbuffer (Visible Display)|  <--- 3. Smooth, Tear-Free 60 FPS
   +---------------------------------------+
```

---

## 6. Hands-On Project: Mini-Paint Graphics Studio

Let's build a mini-paint application that renders geometric shapes, coordinate rulers, and palette colors:

```runvoid
say cyan "=== Launching Runvoid Mini-Paint Studio ==="

screen "Runvoid Mini-Paint Studio", 640, 480 {
    // 1. Dark background
    draw box at 0, 0, size 640, 480, color "black"

    // 2. Palette toolbar at the top
    draw box at 0, 0, size 640, 50, color "white"
    draw text "MINI-PAINT 2D STUDIO", at 20, 20, color "black"

    // Palette swatches
    draw box at 240, 10, size 30, 30, color "red"
    draw box at 280, 10, size 30, 30, color "green"
    draw box at 320, 10, size 30, 30, color "blue"
    draw box at 360, 10, size 30, 30, color "yellow"
    draw box at 400, 10, size 30, 30, color "cyan"
    draw box at 440, 10, size 30, 30, color "magenta"

    // 3. Canvas Artwork Area
    draw box at 50, 80, size 540, 360, color "black"
    draw line from 50, 80 to 590, 80, color "white"
    draw line from 50, 440 to 590, 440, color "white"
    draw line from 50, 80 to 50, 440, color "white"
    draw line from 590, 80 to 590, 440, color "white"

    // 4. Draw sample geometric vector graphics
    draw circle at 320, 260, size 80, color "cyan"
    draw circle at 320, 260, size 50, color "blue"
    draw circle at 320, 260, size 20, color "white"

    draw box at 100, 120, size 80, 80, color "yellow"
    draw box at 460, 120, size 80, 80, color "magenta"

    // 5. Status bar footer
    draw text "Canvas Resolution: 640x480 | Color Mode: 24-bit TrueColor", at 120, 460, color "cyan"
}

say green "Mini-Paint session closed cleanly."
```
