# 1.1 Installation & System Requirements

Runvoid compiles high-level conversational code into native machine code for **Linux x86_64** (ELF64) and **Windows x86_64** (PE32+). To use Runvoid, you need the compiler itself along with the underlying assembler (`nasm`) and linker (`gcc`).

This section guides you through installing Runvoid on Linux and Windows, configuring cross-compilation toolchains, and verifying your environment.

---

## Quick Start: Native Installers

Runvoid provides native, one-step installer scripts for both major operating systems.

### Installing on Linux

You can install Runvoid automatically using our shell installer:

```bash
git clone https://github.com/runvoid/runvoid.git
cd runvoid
chmod +x install.sh
./install.sh
```

The installer will:
1. Detect your Linux distribution package manager (`pacman`, `apt`, `dnf`, `zypper`, `apk`).
2. Offer to install missing prerequisites (`nasm`, `gcc`, `rust/cargo`).
3. Compile the release binary using `cargo build --release`.
4. Install `runvoid` to `/usr/local/bin` (or `~/.local/bin` if root access is not available).
5. Verify that `runvoid` is accessible in your current shell `PATH`.

### Installing on Windows

On Windows 10 or 11 (64-bit), open **PowerShell** (or Windows Terminal) and run:

```powershell
git clone https://github.com/runvoid/runvoid.git
cd runvoid
powershell -ExecutionPolicy Bypass -File .\install.ps1
```

Alternatively, you can double-click **`install.bat`** from Windows Explorer to launch the automated installer in a classic Command Prompt window.

The Windows installer will:
1. Check for `cargo`, `nasm`, and `gcc` (MinGW-w64).
2. Build the optimized native `runvoid.exe` binary.
3. Install the executable to `%LOCALAPPDATA%\Programs\Runvoid\bin`.
4. Add the installation directory to your User `PATH` environment variable permanently.
5. Verify the installation.

---

## Manual Installation & Prerequisites

If you prefer full control over your environment, you can install the individual prerequisites manually.

### Linux Prerequisites

Runvoid requires:
* **Architecture:** x86_64 (AMD64)
* **Rust Toolchain:** `rustc` and `cargo` (>= 1.80) — install via [rustup.rs](https://rustup.rs/) or your distribution.
* **Assembler:** `nasm` (>= 2.15)
* **Linker / C Compiler:** `gcc` or `clang`
* **Optional X11 Library:** `libX11` (required only for 2D Screen canvas / GUI applications)

#### Arch Linux / Manjaro:
```bash
sudo pacman -S rust cargo nasm gcc libx11
```

#### Ubuntu / Debian / Linux Mint / Pop!_OS:
```bash
sudo apt update && sudo apt install -y rustc cargo nasm gcc libx11-dev
```

#### Fedora / RHEL / Rocky Linux:
```bash
sudo dnf install -y rust cargo nasm gcc libX11-devel
```

#### openSUSE:
```bash
sudo zypper install rust cargo nasm gcc libX11-devel
```

#### Alpine Linux:
```bash
sudo apk add rust cargo nasm gcc musl-dev libx11-dev
```

### Windows Prerequisites

On Windows, install the following tools:
1. **Rust:** Install via [rustup.rs](https://rustup.rs) (select `x86_64-pc-windows-msvc` or `x86_64-pc-windows-gnu`).
2. **NASM:** Install via Windows Package Manager (`winget`):
   ```powershell
   winget install NASM.NASM
   ```
   Or via Chocolatey:
   ```powershell
   choco install nasm
   ```
3. **MinGW-w64 GCC:**
   ```powershell
   winget install MSYS2.MSYS2
   ```
   Or install a standalone MinGW-w64 distribution (such as WinLibs) and ensure `gcc.exe` and `nasm.exe` are in your system `PATH`.

---

## Cross-Compilation Setup (Linux to Windows)

One of Runvoid's flagship features is seamless **cross-compilation**: you can compile native Windows `.exe` binaries directly from your Linux workstation!

To enable Windows cross-compilation on Linux:

### Arch Linux:
```bash
sudo pacman -S mingw-w64-gcc wine
```

### Ubuntu / Debian:
```bash
sudo apt install -y gcc-mingw-w64 wine
```

### Fedora:
```bash
sudo dnf install -y mingw64-gcc wine
```

Once installed, simply pass `--target windows` to any `build` or `run` command:

```bash
# Compile and run a Windows .exe immediately on Linux using Wine
runvoid run --target windows app.rv

# Build a standalone Windows executable
runvoid build --target windows app.rv -o my_tool.exe
```

---

## Verifying the Installation

Verify that Runvoid is properly installed and discoverable in your terminal:

```bash
runvoid --version
```

Output:
```text
runvoid 0.3.0
```

Print the interactive quick-reference cheat sheet:

```bash
runvoid cheat
```

If you see the full syntax cheatsheet with color-coded examples, your Runvoid environment is fully operational!

---

## Updating and Uninstalling

### Updating Runvoid

To update to the latest release, pull the newest changes from the repository and re-run the installer:

```bash
git pull origin main
./install.sh
```

### Uninstalling Runvoid

On Linux:
```bash
./install.sh --uninstall
```

On Windows:
```powershell
powershell -ExecutionPolicy Bypass -File .\install.ps1 -Uninstall
```

---

## Troubleshooting

### `nasm: command not found`
The Runvoid compiler relies on the Netwide Assembler (`nasm`) to translate generated assembly into object files. If you encounter this error, ensure `nasm` is installed and located in a directory listed in your `PATH` environment variable. On Windows, the default NASM installer places the executable in `C:\Program Files\NASM` or `C:\Users\<User>\AppData\Local\bin\NASM`.

### Linker Error: `cannot find -lX11`
If you compile an application that utilizes the 2D Screen canvas (`open screen`) on Linux, the linker requires the X11 development headers. Install `libx11-dev` (Debian/Ubuntu) or `libX11-devel` (Fedora). If you are building headless CLI utilities or using Pro Systems mode, X11 is not needed.

### Windows PowerShell: Execution Policy Error
If PowerShell prevents running `install.ps1` with an execution policy error, run:
```powershell
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
.\install.ps1
```

