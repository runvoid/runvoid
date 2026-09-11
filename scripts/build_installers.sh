#!/usr/bin/env bash
# Build script for Runvoid GUI Installers (Linux & Windows)
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "=== 1. Building Linux Release Binary ==="
cargo build --release

echo "=== 2. Building Windows Release Binary ==="
cargo build --release --target x86_64-pc-windows-gnu

echo "=== 3. Generating Embedded Payloads ==="
mkdir -p installer
python3 scripts/generate_payloads.py runtime/gui.c payload_gui installer/payload_gui.h
python3 scripts/generate_payloads.py target/release/runvoid payload_runvoid_linux installer/payload_runvoid_linux.h
python3 scripts/generate_payloads.py target/x86_64-pc-windows-gnu/release/runvoid.exe payload_runvoid_win installer/payload_runvoid_win.h

echo "=== 4. Compiling Linux GUI Installer ==="
gcc -O2 installer/installer_linux.c -o runvoid-installer-linux -lX11
chmod +x runvoid-installer-linux

echo "=== 5. Compiling Windows GUI Installer ==="
x86_64-w64-mingw32-gcc -O2 -mwindows installer/installer_win.c -o runvoid-setup.exe -lcomctl32 -lshlwapi -lole32

echo "=== SUCCESS: Installers built successfully ==="
ls -lh runvoid-installer-linux runvoid-setup.exe
