# 6. Graphics & Windowing: 2D Canvas & GUI

Modern developers often need visual output—whether for 2D arcade games, visual data simulations, or graphical administrative dashboards.

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

## 2. Interactive Game Loops & Animation

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

## 3. Declarative Desktop Windows (GUI)

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

## 4. Under the Hood: Cross-Platform Windowing

The Runvoid runtime adapts its graphics pipeline to the target operating system:
- **On Linux:** Connects to the local X11 display server (`XOpenDisplay`, `XCreateSimpleWindow`, `XSelectInput`, `XPutImage`) with zero third-party framework overhead.
- **On Windows:** Registers a native Win32 window class (`RegisterClassExA`), invokes `CreateWindowExA`, and processes messages via `GetMessage` / `DispatchMessageA`, utilizing the fast Win32 GDI subsystem.

This design gives your Runvoid graphical applications instant startup times (under 2 milliseconds) and negligible memory consumption!

