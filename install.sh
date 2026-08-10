#!/bin/bash
set -e

echo "Installing burr CLI..."

# Check if we are running inside the cloned git repository
if [ -f "Cargo.toml" ] && [ -d "crates/cli" ]; then
    echo "Source repository detected. Building from source via 'make install'..."
    make install
    exit 0
fi

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
    linux*)     OS="linux" ;;
    darwin*)    OS="macos" ;;
    msys*|cygwin*|mingw*) OS="windows" ;;
    *)          echo "Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
    x86_64|amd64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *)          echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

# Find the latest release version
LATEST_RELEASE_URL="https://api.github.com/repos/guyco3/burr/releases/latest"
VERSION=$(curl -sL $LATEST_RELEASE_URL | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$VERSION" ]; then
    echo "Failed to fetch latest release version from GitHub."
    echo "Please build from source using 'make install'."
    exit 1
fi

BINARY_URL="https://github.com/guyco3/burr/releases/download/$VERSION/burr-$OS-$ARCH"

if [ "$OS" == "windows" ]; then
    BINARY_URL="${BINARY_URL}.exe"
fi

INSTALL_DIR="${HOME}/.burr/bin"
mkdir -p "$INSTALL_DIR"
DEST="$INSTALL_DIR/burr"

echo "Downloading burr $VERSION for $OS ($ARCH)..."
curl -sL --fail "$BINARY_URL" -o "$DEST" || {
    echo "Failed to download pre-compiled binary. It may not exist for your platform."
    echo "Please build from source instead: 'make install'"
    exit 1
}

chmod +x "$DEST"

echo "Installation complete! burr has been installed to $DEST"
echo "Make sure $INSTALL_DIR is in your PATH."
