# 6. Graphics & Windowing: 2D Canvas & GUI

Runvoid features built-in graphics windowing, enabling developers to build arcade games and native desktop panels without installing heavyweight third-party frameworks.

## Hardware 2D Screen Canvas

Open an accelerated 2D graphics window powered by the native X11 engine:

```runvoid
screen "Arcade 2D", 640, 480 {
    draw box at 0, 0, size 640, 480, color "black"
    draw circle at 320, 240, size 50, color "cyan"
    draw line from 0, 0 to 640, 480, color "red"
    draw text "Player 1 Ready", at 240, 50, color "yellow"
}
```

Primitive drawing commands include:
- `draw box at <x>, <y>, size <w>, <h>, color <name>`
- `draw circle at <x>, <y>, size <radius>, color <name>`
- `draw line from <x1>, <y1> to <x2>, <y2>, color <name>`
- `draw text <str>, at <x>, <y>, color <name>`

## Declarative Desktop Windows (GUI)

Create desktop utility windows with interactive widgets:

```runvoid
window "Preferences Panel", 450, 300 {
    label "Engine Configuration:"
    checkbox "Enable Sound Effects", 1
    checkbox "Fullscreen Display", 0

    button "Apply Settings" {
        say "Settings updated!"
    }
}
```
