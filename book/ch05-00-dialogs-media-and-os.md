# 5. Console Dialogs, Audio & System Utilities

Runvoid includes batteries-included primitives for interacting with the user, playing sounds, querying the web, and manipulating files.

## Interactive Menus: `choose`

Present an interactive arrow-key selection menu directly inside the terminal:

```runvoid
remember weapon = choose "Select your primary weapon:", "Iron Sword", "Elven Bow", "Fire Staff"
say "Equipped weapon: {weapon}"
```

## Hidden Passwords: `ask hidden`

Read sensitive credentials without echoing keystrokes to the terminal:

```runvoid
remember token = ask hidden "Enter access key: "
```

## Dialog Boxes & Confirmations

```runvoid
remember proceed = ask user "Would you like to overwrite existing files?"
if proceed {
    say "Proceeding with write..."
}

alert "Operation completed successfully!"
```

## Terminal Bell & Text-to-Speech

```runvoid
beep                      # Terminal bell chime
speak "Mission completed" # System text-to-speech
```

## File I/O & Filesystem Operations

```runvoid
# Direct file reading and writing:
write "Runvoid v0.2.0 data" into "log.txt"
remember content = read "log.txt"
say content

# Directories and files:
create folder "backups"
copy file "log.txt" to "backups/log.txt"
delete file "log.txt"

if file "backups/log.txt" exists {
    say "Backup exists on disk."
}
```

## Web Networking

```runvoid
download "https://runvoid.org/logo.png" into "logo.png"
remember api_response = read web "https://api.github.com"
```

## Text Transforms & Performance Timers

```runvoid
remember title = "   The Great Kingdom   "
make title uppercase    # "   THE GREAT KINGDOM   "
make title lowercase    # "   the great kingdom   "
make title trim         # "The Great Kingdom"

remember replaced = replace "Kingdom" with "Empire" in title

# Benchmarking in milliseconds:
measure time {
    remember count = 0
    repeat 1000000 as i {
        count = count + 1
    }
}
```
