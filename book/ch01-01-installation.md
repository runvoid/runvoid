# 1.1 Installation & System Requirements

Runvoid compiles code down to native Linux x86_64 ELF binaries. To run the compiler, ensure you have the standard build toolchain installed on your Linux system.

## System Prerequisites

Runvoid requires:
- A Linux operating system on x86_64 architecture
- **Rust Toolchain:** `rustc` and `cargo` (>= 1.80)
- **Assembler:** `nasm` (>= 2.15)
- **Linker:** `gcc` or GNU `ld`
- **X11 Libraries:** `libX11` (only needed if building GUI / Screen canvas applications)

### Installing Prerequisites by Distribution:

#### Arch Linux:
```bash
sudo pacman -S rust cargo nasm gcc libx11
```

#### Ubuntu / Debian / Linux Mint:
```bash
sudo apt update && sudo apt install -y rustc cargo nasm gcc libx11-dev
```

#### Fedora:
```bash
sudo dnf install -y rust cargo nasm gcc libX11-devel
```

## Installing Runvoid

Clone the repository and build the release binary:

```bash
git clone https://github.com/runvoid/runvoid.git
cd runvoid
cargo build --release
```

Once compilation completes, copy the executable to your path:

```bash
sudo cp target/release/runvoid /usr/local/bin/
```

Verify your installation:

```bash
runvoid --version
```

You should see output similar to:
```text
runvoid 0.2.0
```
