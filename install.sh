#!/usr/bin/env bash
# ==============================================================================
# Runvoid Universal Linux Installer
# Supported Architecture: Linux x86_64
# ==============================================================================

set -e

RED="\033[1;31m"
GREEN="\033[1;32m"
YELLOW="\033[1;33m"
BLUE="\033[1;34m"
CYAN="\033[1;36m"
BOLD="\033[1m"
RESET="\033[0m"

INSTALL_DIR="/usr/local/bin"
USER_INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="runvoid"

echo -e "${CYAN}╔══════════════════════════════════════════════════════════════╗${RESET}"
echo -e "${CYAN}║             🚀 Runvoid Programming Language Installer         ║${RESET}"
echo -e "${CYAN}║                    Target: Linux x86_64                      ║${RESET}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════════╝${RESET}\n"

# Handle uninstall
if [ "$1" = "--uninstall" ]; then
    echo -e "${YELLOW}Uninstalling Runvoid...${RESET}"
    if [ -f "$INSTALL_DIR/$BINARY_NAME" ]; then
        if [ "$EUID" -ne 0 ]; then
            sudo rm -f "$INSTALL_DIR/$BINARY_NAME"
        else
            rm -f "$INSTALL_DIR/$BINARY_NAME"
        fi
        echo -e "${GREEN}✓ Removed $INSTALL_DIR/$BINARY_NAME${RESET}"
    fi
    if [ -f "$USER_INSTALL_DIR/$BINARY_NAME" ]; then
        rm -f "$USER_INSTALL_DIR/$BINARY_NAME"
        echo -e "${GREEN}✓ Removed $USER_INSTALL_DIR/$BINARY_NAME${RESET}"
    fi
    echo -e "${GREEN}Runvoid has been successfully uninstalled.${RESET}"
    exit 0
fi

# 1. Architecture Check
ARCH=$(uname -m)
if [ "$ARCH" != "x86_64" ]; then
    echo -e "${RED}Error: Runvoid currently requires an x86_64 processor architecture.${RESET}"
    echo -e "Detected architecture: $ARCH"
    exit 1
fi
echo -e "${GREEN}✓ Architecture: x86_64 detected.${RESET}"

# 2. Dependency Check & Installation Helper
MISSING_DEPS=()
command -v nasm >/dev/null 2>&1 || MISSING_DEPS+=("nasm")
command -v gcc >/dev/null 2>&1 || MISSING_DEPS+=("gcc")
command -v cargo >/dev/null 2>&1 || MISSING_DEPS+=("cargo")
command -v rustc >/dev/null 2>&1 || MISSING_DEPS+=("rust")

if [ ${#MISSING_DEPS[@]} -gt 0 ]; then
    echo -e "\n${YELLOW}Missing required build dependencies: ${MISSING_DEPS[*]}${RESET}"
    echo -e "Attempting to install dependencies via system package manager..."

    if command -v pacman >/dev/null 2>&1; then
        sudo pacman -S --needed --noconfirm nasm gcc rust
    elif command -v apt-get >/dev/null 2>&1; then
        sudo apt-get update && sudo apt-get install -y nasm gcc rustc cargo libx11-dev
    elif command -v dnf >/dev/null 2>&1; then
        sudo dnf install -y nasm gcc rust cargo libX11-devel
    elif command -v zypper >/dev/null 2>&1; then
        sudo zypper install -y nasm gcc rust cargo libX11-devel
    elif command -v apk >/dev/null 2>&1; then
        sudo apk add nasm gcc musl-dev rust cargo libx11-dev
    else
        echo -e "${RED}Could not detect supported package manager.${RESET}"
        echo -e "Please install the following tools manually: ${MISSING_DEPS[*]}"
        exit 1
    fi
else
    echo -e "${GREEN}✓ All core dependencies are installed (nasm, gcc, cargo, rustc).${RESET}"
fi

# 3. Build Runvoid
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
echo -e "\n${BLUE}Compiling Runvoid compiler (release mode)...${RESET}"
cd "$SCRIPT_DIR"
cargo build --release

COMPILED_BIN="$SCRIPT_DIR/target/release/runvoid"
if [ ! -f "$COMPILED_BIN" ]; then
    echo -e "${RED}Error: Build failed. Binary not found at $COMPILED_BIN${RESET}"
    exit 1
fi

# 4. Install Binary
echo -e "\n${BLUE}Installing $BINARY_NAME binary...${RESET}"
TARGET_DIR="$INSTALL_DIR"

if [ "$EUID" -ne 0 ]; then
    if sudo -n true 2>/dev/null || [ -t 0 ]; then
        sudo cp "$COMPILED_BIN" "$INSTALL_DIR/$BINARY_NAME"
        sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"
    else
        TARGET_DIR="$USER_INSTALL_DIR"
        mkdir -p "$USER_INSTALL_DIR"
        cp "$COMPILED_BIN" "$USER_INSTALL_DIR/$BINARY_NAME"
        chmod +x "$USER_INSTALL_DIR/$BINARY_NAME"
        if [[ ":$PATH:" != *":$USER_INSTALL_DIR:"* ]]; then
            echo -e "${YELLOW}Note: Add '$USER_INSTALL_DIR' to your PATH in ~/.bashrc or ~/.zshrc:${RESET}"
            echo -e "  export PATH=\"\$HOME/.local/bin:\$PATH\""
        fi
    fi
else
    cp "$COMPILED_BIN" "$INSTALL_DIR/$BINARY_NAME"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
fi

echo -e "${GREEN}✓ Successfully installed Runvoid to $TARGET_DIR/$BINARY_NAME${RESET}"

# 5. Verify Installation
echo -e "\n${CYAN}Verifying installation:${RESET}"
"$TARGET_DIR/$BINARY_NAME" --version || true

echo -e "\n${GREEN}🎉 Runvoid is ready to use!${RESET}"
echo -e "Try it now:"
echo -e "  ${BOLD}runvoid cheat${RESET}       - View interactive syntax cheat sheet"
echo -e "  ${BOLD}runvoid repl${RESET}        - Launch interactive coding shell"
echo -e "  ${BOLD}runvoid new game${RESET}    - Create a starter game template"
echo -e "  ${BOLD}runvoid test${RESET}        - Run automated tests"
