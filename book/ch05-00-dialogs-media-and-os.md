# 5. Console Dialogs, Audio & System Utilities

Modern applications often require seamless interaction with the operating system—displaying native alert dialogs, reading sensitive passwords, playing sound alerts, inspecting files, running subprocesses, handling POSIX signals, or benchmarking execution times.

Runvoid provides batteries-included system primitives that automatically adapt to **Linux** and **Windows** without requiring platform-specific `#ifdef` boilerplate.

---

## 1. Native GUI Dialogs & Confirmations

Runvoid allows you to trigger native desktop dialogs directly from conversational code:

### Information Alerts: `alert`
Display an operating system modal dialog box with an "OK" confirmation button:

```runvoid
alert "Database backup completed successfully!"
```

- **On Windows:** Calls the Win32 `MessageBoxA(NULL, message, "Alert", MB_OK | MB_ICONINFORMATION)`.
- **On Linux:** Automatically dispatches to `zenity`, `kdialog`, or Python `tkinter`, gracefully falling back to a formatted terminal prompt if no graphical display server is detected.

### Binary Questions: `ask user`
Prompt the user for a Yes/No or OK/Cancel decision:

```runvoid
remember proceed = ask user "Do you want to overwrite the existing configuration?"

if proceed {
    say green "Proceeding with file overwrite..."
} otherwise {
    say yellow "Operation cancelled by user."
}
```

On Windows, this renders a native modal dialog with **Yes** and **No** buttons (`MB_YESNO`), returning `1` for Yes and `0` for No.

---

## 2. Advanced Terminal Interaction

### Password Masking: `ask hidden`
When prompting for sensitive credentials such as database passwords, API tokens, or encryption keys, use `ask hidden` to disable terminal character echo:

```runvoid
remember api_token = ask hidden "Enter your secret API key: "
say "Key captured safely ({api_token.length} characters)."
```

### Arrow-Key Menus: `choose`
Runvoid includes a built-in terminal menu widget. It presents an interactive list where users can navigate using their arrow keys and press Enter to select:

```runvoid
remember environment = choose "Select target deployment tier:", "Local Development", "Staging Cluster", "Production (US-East)"
say "Selected environment: {environment}"
```

---

## 3. Audio & Speech Synthesis

### System Beep & Frequency Tone: `beep`
Trigger system tones and PC speaker audio:

```runvoid
// Simple terminal chime:
beep

// Explicit frequency (Hz) and duration (milliseconds):
beep 440 for 200  // Concert A (440 Hz) for 200ms
beep 880 for 150  // Higher octave (880 Hz) for 150ms
```

- **On Windows:** Uses the hardware-level `Beep(frequency, duration)` Win32 API.
- **On Linux:** Emits a terminal bell escape code or interacts with the PC speaker device.

### Text-to-Speech: `speak`
Synthesize spoken English text through the default operating system speech engine:

```runvoid
speak "System update finished. All services are operational."
```

---

## 4. Cross-Platform File & Directory I/O

Runvoid makes filesystem manipulation conversational and concise:

### Reading and Writing Files
```runvoid
// Write text into a file (creates or overwrites):
write "Runvoid 1.3.0 Build Cache\nStatus: OK" into "build.log"

// Read the contents of a file into a string:
remember content = read "build.log"
say "File contents:"
say content
```

### Checking File Existence
```runvoid
if file "config.json" exists {
    say green "Configuration file found."
} otherwise {
    say yellow "Creating default configuration..."
    write "{}" into "config.json"
}
```

### Directory Management & File Operations
```runvoid
// Create directory tree:
create folder "dist/bundles"

// Copy files:
copy file "build.log" to "dist/bundles/build.log"

// Delete files:
delete file "build.log"
```

---

## 5. Subprocess Execution & Environment Inspection

Systems daemons must invoke external utilities and query environment configurations:

### 1. Reading Environment Variables
```runvoid
remember user_home = env "HOME"
remember current_path = env "PATH"
say "Current Home Directory: {user_home}"
```

### 2. Spawning Subprocesses
Runvoid allows executing external shell commands, capturing their exit code and standard output:
```runvoid
remember git_commit = exec "git rev-parse --short HEAD"
say "Active Git Revision: {git_commit}"
```

---

## 6. Graceful Signal Handling (`SIGINT` / `Ctrl+C`)

When writing persistent daemons or services, terminating abruptly with `Ctrl+C` can corrupt database files or leave orphaned temporary locks. Runvoid provides a signal interceptor:

```runvoid
remember is_terminating = false

on signal SIGINT {
    say yellow "\nReceived SIGINT interruption signal. Commencing graceful shutdown..."
    is_terminating = true
}

repeat 10 times as tick {
    if is_terminating {
        say green "Released system locks. Clean shutdown completed."
        stop
    }
    wait 500
}
```

---

## 7. Web Networking

Fetch remote data or download assets using built-in web primitives:

```runvoid
// Read raw text/JSON from an HTTP endpoint:
remember user_agent_info = read web "https://httpbin.org/user-agent"
say user_agent_info

// Download remote files directly to disk:
download "https://runvoid.org/favicon.ico" into "icon.ico"
```

---

## 8. Text Transformations & Performance Profiling

### In-Place String Transformations
Runvoid supports conversational text modifiers:

```runvoid
remember heading = "   the runvoid language   "

make heading uppercase  // "   THE RUNVOID LANGUAGE   "
make heading lowercase  // "   the runvoid language   "
make heading trim       // "the runvoid language"

remember announcement = replace "runvoid" with "Runvoid Native" in heading
say announcement
```

### High-Precision Code Profiling: `measure time`
Measure the exact elapsed time of any block of code with microsecond precision:

```runvoid
measure time {
    remember accumulator = 0
    repeat 10000000 as i {
        accumulator = accumulator + i
    }
    say "Sum completed: {accumulator}"
}
```

The runtime captures the starting and ending hardware timestamps (`clock_gettime` with `CLOCK_MONOTONIC` on Linux, `QueryPerformanceCounter` on Windows) and prints the elapsed milliseconds directly to the console:

```text
Sum completed: 49999995000000
[Execution Time: 8.42 ms]
```

---

## 9. OS API Architecture Comparison

Here is how Runvoid abstracts operating system capabilities across platforms:

| Feature | Linux x86_64 Implementation | Windows x86_64 Implementation |
| :--- | :--- | :--- |
| **`alert "..."`** | `zenity` / `kdialog` / ANSI fallback | Win32 `MessageBoxA` |
| **`ask hidden`** | POSIX `tcgetattr`/`tcsetattr` (`~ECHO`) | Win32 `GetConsoleMode`/`SetConsoleMode` |
| **`beep freq for ms`** | Terminal `\a` or `/dev/console` ioctl | Win32 `Beep(freq, ms)` (`kernel32.dll`) |
| **`measure time`** | POSIX `clock_gettime(CLOCK_MONOTONIC)` | Win32 `QueryPerformanceCounter` |
| **`read web`** | `libcurl` / `wget` / POSIX socket | Win32 `WinINet` / `WinHTTP` API |
| **`speak "..."`** | `spd-say` / `espeak` | SAPI 5 / `System.Speech` |
| **`env "KEY"`** | POSIX `getenv()` | Win32 `GetEnvironmentVariableA` |
| **`on signal`** | POSIX `sigaction()` | Win32 `SetConsoleCtrlHandler` |

---

## 10. Hands-On Project: Automated Server Health Auditor

Let's combine dialogs, colored output, filesystem I/O, and timing into an automated server health audit script:

```runvoid
say cyan "=================================================="
say cyan "       RUNVOID SERVER HEALTH AUDITOR v1.3         "
say cyan "=================================================="

remember report_file = "audit_report.txt"
remember start_audit = ask user "Initiate comprehensive system audit?"

if not start_audit {
    say yellow "Audit aborted by administrator."
    give 0
}

measure time {
    say "Verifying application directories..."
    create folder "var/log"
    create folder "var/cache"

    say "Testing filesystem write latency..."
    remember sample_data = "AUDIT_TEST_PAYLOAD_8192_BYTES_VERIFIED"
    write sample_data into "var/cache/probe.tmp"

    if file "var/cache/probe.tmp" exists {
        say green "[PASS] Filesystem read/write verified."
        delete file "var/cache/probe.tmp"
    } otherwise {
        say red "[FAIL] Filesystem write failed!"
        alert "Critical: Filesystem permissions check failed!"
    }

    say "Generating final audit log..."
    write "Server Audit Log\nStatus: HEALTHY\nNode: Antigravity-1" into report_file
}

say green "Audit complete. Report saved to: {report_file}"
beep 523 for 100
beep 1046 for 200
```
