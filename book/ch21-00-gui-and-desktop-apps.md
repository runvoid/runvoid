# 21. Native Desktop GUI & Window Application Architecture

Modern graphical user interfaces (GUIs) power desktop utilities, creative applications, developer tools, and operating system shells. Unlike stateless terminal programs that run sequentially from top to bottom and exit, GUI applications operate as **long-running stateful event loops**.

In this chapter, you will master GUI engineering in Runvoid:
1. The Windowing and Event-Driven Architecture (Win32, Wayland/X11, Cocoa).
2. The Anatomy of an Event Loop: Mouse, Keyboard, and Redraw signals.
3. State Management: Single Source of Truth and Declarative Canvas Rendering.
4. Building Custom UI Widgets: Buttons, Sliders, Progress Bars, and Textboxes.
5. Hands-On Project: A Complete Desktop System Monitor & Control Dashboard.

---

## 1. The Architecture of Graphical Desktop Applications

Every native desktop window is a managed OS surface connected to the display server:

```
+-------------------------------------------------------------------------+
| Display Server / Window Manager (Wayland, X11, Windows DWM)             |
+-------------------------------------------------------------------------+
       |                                                 ^
       | OS Input Events (Key, Mouse, Resize)            | Framebuffer Blit
       v                                                 |
+-------------------------------------------------------------------------+
| Runvoid Event Loop (poll_event)                                         |
+-------------------------------------------------------------------------+
       |                                                 |
       v                                                 v
+-------------------------------------------------------------------------+
| State Update (Mutate Data)  ---> Render Canvas (Draw Rect, Text, Sprites)
+-------------------------------------------------------------------------+
```

### The Event Loop Cycle

A standard 60 FPS (Frames Per Second) GUI application has approximately **$16.6\text{ milliseconds}$** per frame to complete:
1. **Poll OS Event Queue:** Collect mouse clicks, key presses, and window close events.
2. **Update Application State:** Mutate active models (e.g. increase click counter, toggle checkboxes).
3. **Layout & Compute Bounds:** Calculate pixel coordinates for all on-screen elements.
4. **Draw Framebuffer:** Issue 2D canvas drawing calls.
5. **Swap Buffers:** Flip the back buffer to the physical monitor to prevent visual tearing.

---

## 2. Immediate Mode vs. Retained Mode GUI

There are two dominant GUI design paradigms in modern software:

### 1. Retained Mode (DOM, Qt, WPF, GTK):
The programmer constructs an object graph of UI widgets (`Button`, `Window`, `Panel`). The framework maintains this tree in memory and handles rendering automatically.
- *Pros:* Easy layout management.
- *Cons:* Heavy memory allocations, complex synchronization between business state and UI state.

### 2. Immediate Mode (Dear ImGui, Runvoid 2D Canvas):
No widget objects are retained between frames. The UI is rebuilt every single frame directly from pure application state:
```runvoid
if button("Save File", x: 100, y: 50) {
    save_document()
}
```
- *Pros:* Zero synchronization bugs; minimal memory allocation; total visual control.
- *Cons:* CPU must redraw elements each frame (mitigated by GPU acceleration).

---

## 3. Hands-On Project: Desktop System Monitor Dashboard

Let's build a multi-widget desktop application in Runvoid that displays real-time hardware telemetry and provides interactive control buttons:

```runvoid
say cyan "=================================================="
say cyan "       RUNVOID DESKTOP CONTROL DASHBOARD          "
say cyan "=================================================="

// 1. Initialize Window Canvas
remember SCREEN_WIDTH = 640
remember SCREEN_HEIGHT = 480

say green "Spawning Native OS Window: 640x480..."
open window "System Control Panel" size 640 by 480

// 2. Application State Model
remember app_state = {
    "is_running": true,
    "cpu_usage_pct": 28,
    "ram_used_mb": 412,
    "ram_total_mb": 1024,
    "fan_boost_enabled": false,
    "selected_tab": "OVERVIEW"
}

// 3. UI Component: Progress Bar Widget
action draw_progress_bar(x, y, width, height, percent, label) {
    // Background bar
    draw rect at x, y size width by height color "darkgray"
    
    // Filled progress
    remember fill_w = (width * percent) / 100
    remember bar_color = "green"
    if percent > 80 {
        bar_color = "red"
    } otherwise if percent > 50 {
        bar_color = "yellow"
    }
    
    draw rect at x, y size fill_w by height color bar_color
    draw text "{label}: {percent}%" at x, y - 18 color "white"
}

// 4. UI Component: Interactive Push Button
action draw_button(x, y, width, height, label, is_active) {
    remember btn_color = "gray"
    if is_active {
        btn_color = "cyan"
    }
    draw rect at x, y size width by height color btn_color
    draw text label at x + 15, y + 10 color "black"
}

// 5. Main Application Event & Render Loop
say yellow "Entering primary 60 FPS desktop event loop..."

repeat 5 times { // Simulated 5 consecutive frames
    // Clear Framebuffer
    clear canvas "black"
    
    // Header Banner
    draw rect at 0, 0 size 640 by 50 color "blue"
    draw text "RUNVOID HARDWARE TELEMETRY MONITOR" at 20, 15 color "white"
    
    // Telemetry Gauges
    draw_progress_bar(50, 100, 300, 24, app_state["cpu_usage_pct"], "CPU Core Utilization")
    
    remember ram_pct = (app_state["ram_used_mb"] * 100) / app_state["ram_total_mb"]
    draw_progress_bar(50, 160, 300, 24, ram_pct, "RAM Occupancy ({app_state[\"ram_used_mb\"]} MB)")
    
    // Interactive Controls
    draw_button(50, 230, 180, 40, "TOGGLE FAN BOOST", app_state["fan_boost_enabled"])
    draw_button(250, 230, 180, 40, "TERMINATE PROCESS", false)
    
    // Render Frame to Display Surface
    render canvas
    
    // Simulate real-time sensor fluctuation
    app_state["cpu_usage_pct"] = app_state["cpu_usage_pct"] + 7
    wait 30
}

say green "Dashboard event loop shut down cleanly."
```

---

## 4. Cross-Platform Windowing Backends

Runvoid generates native GUI binaries across platforms:
- **Linux (Wayland & X11):** Interacts with `libwayland-client` and XCB/Xlib. Window rendering utilizes OpenGL/EGL hardware surfaces.
- **Windows (x86_64):** Utilizes `user32.dll` and Direct2D/DirectX hardware acceleration with native DPI scaling.
- **macOS (Cocoa):** Direct bridging into `NSWindow` and Metal 2D canvas surfaces.

With Runvoid's unified graphics and audio abstractions, your desktop applications run natively everywhere with zero code alterations!
