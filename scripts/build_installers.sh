#!/usr/bin/env bash
# Build script for Runvoid GUI Installers (Linux & Windows)
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "=== 1. Locating or Building Linux Release Binary ==="
if [ -f target/x86_64-unknown-linux-gnu/release/runvoid ]; then
    LINUX_BIN="target/x86_64-unknown-linux-gnu/release/runvoid"
elif [ -f target/release/runvoid ]; then
    LINUX_BIN="target/release/runvoid"
else
    cargo build --release
    LINUX_BIN="target/release/runvoid"
fi

echo "=== 2. Locating or Building Windows Release Binary ==="
WIN_BIN="target/x86_64-pc-windows-gnu/release/runvoid.exe"
if [ ! -f "$WIN_BIN" ]; then
    cargo build --release --target x86_64-pc-windows-gnu
fi

echo "=== 3. Generating Embedded Payloads ==="
mkdir -p installer
python3 scripts/generate_payloads.py runtime/gui.c payload_gui installer/payload_gui.h
python3 scripts/generate_payloads.py "$LINUX_BIN" payload_runvoid_linux installer/payload_runvoid_linux.h
python3 scripts/generate_payloads.py "$WIN_BIN" payload_runvoid_win installer/payload_runvoid_win.h

echo "=== 4. Compiling Linux GUI Installer ==="
gcc -O2 installer/installer_linux.c -o runvoid-installer-linux -lX11
chmod +x runvoid-installer-linux

echo "=== 5. Compiling Windows GUI Installer ==="
x86_64-w64-mingw32-gcc -O2 -mwindows installer/installer_win.c -o runvoid-setup.exe -lcomctl32 -lshlwapi -lole32

echo "=== SUCCESS: Installers built successfully ==="
ls -lh runvoid-installer-linux runvoid-setup.exe
