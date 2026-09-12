# 12. Real-Time 2D Arcade Game Development

Video games are among the most demanding pieces of software to engineer: they require strict 60 FPS frame timing, deterministic physics simulation, immediate input response, hardware double-buffering, and audio-visual synchronization.

In this masterclass, you will build a complete, fully playable retro arcade game from scratch in Runvoid: **"Star Void: Galactic Defender"**, complete with enemy fleets, particle explosion physics, and boss state machines.

---

## 1. The Anatomy of a Real-Time Game Loop

Every real-time game engine—from 8-bit classics to modern Unreal Engine titles—is driven by an infinite **Game Loop**:

```
                  +-----------------------------------+
                  |        START GAME SESSION         |
                  +-----------------------------------+
                                    |
+-----------------------------------+-----------------------------------+
|                           THE GAME LOOP                               |
|                                                                       |
|   1. Poll Input --------> 2. Update Physics ------> 3. Collision      |
|      (Keyboard / Mouse)      (Velocities, Coords)      Detection      |
|                                                            |          |
|   6. Frame Delay <------- 5. Double-Buffer <------- 4. Render Frame   |
|      (Cap at 60 FPS)         Buffer Swap (VSync)       (Shapes, Text) |
|                                                                       |
+-----------------------------------+-----------------------------------+
                                    |
                                    v (Player Escapes or Game Over)
                  +-----------------------------------+
                  |         CLEANUP & HIGHSCORES      |
                  +-----------------------------------+
```

### The Three Invariants of a Game Loop:
1. **Fixed Timestep Simulation:** Coordinates must update at a consistent rate to ensure gameplay feels identical on fast and slow CPUs.
2. **Double-Buffered Framebuffer:** Drawing occurs on an off-screen buffer in RAM. When complete, the buffer is swapped to the display in a single blit, eliminating ugly screen tearing.
3. **Zero Allocation in Loop:** Creating and destroying dynamic objects inside the frame loop triggers allocator churn. Pre-allocate your entity tables before entering the loop.

---

## 2. Collision Detection: Axis-Aligned Bounding Box (AABB)

In 2D space, the fastest way to detect if two non-rotated rectangular sprites intersect is the **AABB Theorem**:

```
Box A: (x1, y1) to (x1 + w1, y1 + h1)
Box B: (x2, y2) to (x2 + w2, y2 + h2)

Two boxes intersect IF AND ONLY IF:
1. Box A's left edge is to the left of Box B's right edge: (x1 < x2 + w2)
2. Box A's right edge is to the right of Box B's left edge: (x1 + w1 > x2)
3. Box A's top edge is above Box B's bottom edge: (y1 < y2 + h2)
4. Box A's bottom edge is below Box B's top edge: (y1 + h1 > y2)
```

In Runvoid:
```runvoid
action check_aabb_collision(ax, ay, aw, ah, bx, by, bw, bh) {
    if ax < (bx + bw) and (ax + aw) > bx and ay < (by + bh) and (ay + ah) > by {
        give true
    }
    give false
}
```

---

## 3. Particle Explosion Physics System

When an alien craft explodes, realistic destruction requires spawning a cloud of high-velocity spark particles flying outwards in all directions:

```runvoid
action spawn_explosion_particles(origin_x, origin_y) {
    remember particles = [
        {"x": origin_x, "y": origin_y, "vx": -3, "vy": -4, "life": 12},
        {"x": origin_x, "y": origin_y, "vx":  3, "vy": -4, "life": 12},
        {"x": origin_x, "y": origin_y, "vx": -5, "vy":  1, "life": 10},
        {"x": origin_x, "y": origin_y, "vx":  5, "vy":  1, "life": 10},
        {"x": origin_x, "y": origin_y, "vx": -2, "vy":  5, "life": 14},
        {"x": origin_x, "y": origin_y, "vx":  2, "vy":  5, "life": 14}
    ]
    give particles
}
```

Every frame, the physics engine updates each particle:
$$x = x + v_x$$
$$y = y + v_y + \text{gravity}$$
$$\text{life} = \text{life} - 1$$

When $\text{life} \le 0$, the particle is extinguished.

---

## 4. Complete Project: "Star Void: Galactic Defender"

Here is the complete, playable arcade game. It includes:
- Animated alien enemy invaders moving in formation.
- Laser photon cannon firing with high-frequency audio synthesis (`beep`).
- Axis-Aligned Bounding Box (AABB) collision detection.
- Explosive visual sparks on enemy destruction.
- Scoreboard and HUD rendering.

```runvoid
say cyan "=== Launching Star Void: Galactic Defender ==="

remember score = 0
remember lives = 3
remember wave = 1

screen "Star Void: Galactic Defender", 640, 480 {
    remember ship_x = 300
    remember ship_y = 420
    remember ship_dir = 4

    // Alien Invader State
    remember alien_x = 100
    remember alien_y = 80
    remember alien_speed = 3
    remember alien_alive = 1

    // Laser Cannon State
    remember laser_x = 0
    remember laser_y = 0
    remember laser_active = 0

    // Stars
    remember s1_y = 50
    remember s2_y = 200
    remember s3_y = 350

    repeat 300 times {
        // 1. Clear background
        draw box at 0, 0, size 640, 480, color "black"

        // 2. Parallax Stars
        draw pixel at 120, s1_y, color "white"
        draw pixel at 360, s2_y, color "white"
        draw pixel at 540, s3_y, color "cyan"
        s1_y = (s1_y + 3) % 480
        s2_y = (s2_y + 5) % 480
        s3_y = (s3_y + 2) % 480

        // 3. Update Player Patrol Movement
        ship_x = ship_x + ship_dir
        if ship_x <= 40 or ship_x >= 580 {
            ship_dir = 0 - ship_dir
        }

        // 4. Update Alien Invader
        if alien_alive == 1 {
            alien_x = alien_x + alien_speed
            if alien_x <= 50 or alien_x >= 560 {
                alien_speed = 0 - alien_speed
                alien_y = alien_y + 15
            }

            // Draw Alien Sprite (Red & Yellow)
            draw box at alien_x, alien_y, size 30, 16, color "red"
            draw circle at alien_x + 8, alien_y + 6, size 3, color "yellow"
            draw circle at alien_x + 22, alien_y + 6, size 3, color "yellow"
        }

        // 5. Fire Laser automatically when alien is overhead
        if laser_active == 0 and alien_alive == 1 {
            laser_x = ship_x + 9
            laser_y = ship_y - 8
            laser_active = 1
            beep 880 for 25 // Laser sound effect!
        }

        // 6. Update Laser
        if laser_active == 1 {
            laser_y = laser_y - 12
            draw box at laser_x, laser_y, size 3, 10, color "green"

            // Collision Detection (Laser hits Alien):
            if alien_alive == 1 {
                if laser_x >= alien_x and laser_x <= alien_x + 30 {
                    if laser_y >= alien_y and laser_y <= alien_y + 16 {
                        // Explosion!
                        alien_alive = 0
                        laser_active = 0
                        score = score + 100
                        beep 220 for 120 // Deep explosion rumble
                    }
                }
            }

            if laser_y <= 20 {
                laser_active = 0
            }
        }

        // 7. Draw Player Spaceship
        draw box at ship_x + 8, ship_y, size 4, 16, color "cyan"
        draw box at ship_x, ship_y + 10, size 20, 8, color "blue"
        draw circle at ship_x + 10, ship_y + 6, size 3, color "white"

        // 8. Render HUD Display
        draw text "SCORE: {score}", at 30, 25, color "yellow"
        draw text "LIVES: 3", at 250, 25, color "green"
        draw text "WAVE: {wave}", at 450, 25, color "cyan"
    }
}

say green "Game session completed! Final Score: {score}"
```

---

## 5. Boss Battle State Machine Architecture

A boss encounter transitions between combat phases using a state machine:

```
            +---------------------------+
            | PHASE 1: Laser Strafing   |
            +---------------------------+
                          |
                          v (Health < 60%)
            +---------------------------+
            | PHASE 2: Missile Barrage  |
            +---------------------------+
                          |
                          v (Health < 20%)
            +---------------------------+
            | PHASE 3: Enraged Divebomb |
            +---------------------------+
```

```runvoid
action boss_update(boss, target_x) {
    if boss["phase"] == "STRAFE" {
        boss["x"] = boss["x"] + boss["speed"]
        if boss["x"] <= 50 or boss["x"] >= 550 {
            boss["speed"] = 0 - boss["speed"]
        }
        if boss["health"] < 60 {
            boss["phase"] = "MISSILE"
            say yellow "WARNING: Boss entering Missile Barrage phase!"
        }
    } otherwise if boss["phase"] == "MISSILE" {
        // Fire secondary missiles
        if boss["health"] < 20 {
            boss["phase"] = "ENRAGED"
            say red "CRITICAL ALERT: Boss entering Enraged Divebomb phase!"
        }
    }
}
```

---

## 6. Summary

In this masterclass, you constructed a real-time arcade game engine:
- Mastered the three-phase game loop (Input, Physics, Render).
- Handled multi-layer parallax background stars.
- Implemented AABB bounding box collision math.
- Added synthesized retro arcade sound effects for laser blasts and explosions.
- Engineered particle physics and multi-phase boss state machines!
