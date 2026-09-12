# 1.1 Installation & System Requirements

Runvoid compiles high-level conversational code into native machine code for **Linux x86_64** (ELF64) and **Windows x86_64** (PE32+). To use Runvoid, you need the compiler itself along with the underlying assembler (`nasm`) and linker (`gcc`).

This section guides you through installing Runvoid on Linux and Windows, configuring cross-compilation toolchains, and verifying your environment.

---

## Quick Start: Graphical & Script Installers

Runvoid provides both standalone **GUI setup wizards** (ideal for end-users downloading official releases) and **automated terminal scripts** (ideal for CI/CD and developer workstations).

### 1. Windows Graphical Setup Wizard (`runvoid-setup.exe`)

For Windows 10/11 x86_64, official releases include `runvoid-setup.exe`:
1. Download `runvoid-setup.exe` from the latest [GitHub Release](https://github.com/runvoid/runvoid/releases).
2. Double-click `runvoid-setup.exe` to launch the native Win32 setup wizard.
3. Choose your destination directory (defaults to `%LOCALAPPDATA%\Programs\Runvoid\bin`).
4. Keep the **"Add Runvoid to User PATH"** option checked.
5. Click **Install**. The wizard extracts the native compiler and standard C runtime components, configures your system environment variables, and verifies the installation.

### 2. Linux Graphical Installer App (`runvoid-installer-linux`)

For 64-bit Linux distributions, official releases provide a self-contained GUI installer:
1. Download `runvoid-installer-linux` from [GitHub Releases](https://github.com/runvoid/runvoid/releases).
2. Make the installer executable:
   ```bash
   chmod +x runvoid-installer-linux
   ```
3. Run the installer:
   ```bash
   ./runvoid-installer-linux
   ```
4. A native setup window will appear, allowing you to configure the destination (`/usr/local/bin` or `~/.local/bin`), update your shell PATH (`~/.bashrc`, `~/.profile`), and install with a single click!
   *(Note: If running on a headless server without a graphical display, `runvoid-installer-linux --cli` automatically runs in terminal mode).*

### 3. Automated Terminal Scripts

If you prefer building from source or running headless shell scripts:

#### Linux (`install.sh`)
```bash
git clone https://github.com/runvoid/runvoid.git
cd runvoid
chmod +x install.sh
./install.sh
```

#### Windows PowerShell (`install.ps1`) & CMD (`install.bat`)
```powershell
powershell -ExecutionPolicy Bypass -File .\install.ps1
```
Or double-click `install.bat` from Windows Explorer.

---

## Verifying Release Integrity with Checksums

Every official release includes a cryptographic `SHA256SUMS.txt` file containing SHA-256 hashes of all release packages and installer executables.

To verify the integrity and authenticity of your downloaded files on Linux:

```bash
sha256sum -c SHA256SUMS.txt
```

On Windows (PowerShell):
```powershell
Get-FileHash .\runvoid-setup.exe -Algorithm SHA256
```
Compare the output against the hash listed in `SHA256SUMS.txt`.


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
runvoid 1.3.0
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

---

## Alternative Installation: Docker & DevContainers

If you prefer zero local configuration or want to run Runvoid in an isolated sandbox, Runvoid provides official Docker images and VS Code DevContainer configurations.

### 1. Running via Docker

The official Runvoid Docker image bundles the `runvoid` compiler, `nasm`, `gcc`, and the full runtime in a compact multi-stage container:

```bash
# Pull and run the interactive cheat sheet:
docker run --rm -it runvoid/runvoid:latest cheat

# Mount your current directory and compile an application:
docker run --rm -v $(pwd):/workspace -w /workspace runvoid/runvoid:latest run app.rv
```

### 2. Instant Cloud Development with VS Code DevContainers

Runvoid includes a `.devcontainer` configuration supporting **GitHub Codespaces** and the **VS Code Dev Containers extension**:

```
+--------------------------------------------------------------------------+
| VS Code Desktop / Browser (GitHub Codespaces)                            |
| +----------------------------------------------------------------------+ |
| | DevContainer: Debian Trixie + Rust + NASM + GCC + MinGW + Wine + X11 | |
| | Extensions: Rust Analyzer, CodeLLDB, Runvoid TextMate Syntax         | |
| +----------------------------------------------------------------------+ |
+--------------------------------------------------------------------------+
```

1. Open the repository in VS Code:
   ```bash
   code .
   ```
2. When prompted with **"Folder contains a Dev Container configuration file. Reopen in Container?"**, click **Reopen in Container**.
3. All compilers, linkers, cross-compilers, and editor extensions are configured automatically inside the container within seconds!

---

## Toolchain Resolution & Compilation Flow

When `runvoid` compiles a source file, it dynamically probes and coordinates the system toolchain:

```
                  +---------------------------+
                  | Runvoid Compiler Engine   |
                  +---------------------------+
                                |
                                v
               Detects Target: Linux or Windows?
                                |
             +------------------+------------------+
             |                                     |
             v (Linux ELF64)                       v (Windows PE32+)
    +-----------------+                   +-----------------+
    | Probe nasm      |                   | Probe nasm      |
    | (nasm -f elf64) |                   | (nasm -f win64) |
    +-----------------+                   +-----------------+
             |                                     |
             v                                     v
    +-----------------+                   +-----------------+
    | Probe gcc       |                   | Probe MinGW GCC |
    | (gcc -no-pie)   |                   | (x86_64-w64-    |
    +-----------------+                   |  mingw32-gcc)   |
             |                            +-----------------+
             v                                     |
    Native Linux Binary                            v
                                          Windows .exe Binary
```

If any prerequisite is missing from your system `PATH`, Runvoid reports the exact missing binary along with instructions on how to install it.


