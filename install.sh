#!/bin/bash
set -euo pipefail

# ClaudeForge Installer Script
# This script installs the latest release of claudeforge from GitHub

REPO="iepathos/claudeforge"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
BINARY_NAME="claudeforge"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture names
case "$ARCH" in
    x86_64)
        ARCH="x86_64"
        ;;
    aarch64|arm64)
        ARCH="aarch64"
        ;;
    *)
        echo "Error: Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Map OS names to match release artifacts
case "$OS" in
    darwin)
        TARGET="${ARCH}-apple-darwin"
        ;;
    linux)
        TARGET="${ARCH}-unknown-linux-gnu"
        ;;
    msys*|mingw*|cygwin*)
        TARGET="${ARCH}-pc-windows-msvc"
        echo "Note: Windows detected. You may need to add $INSTALL_DIR to your PATH."
        ;;
    *)
        echo "Error: Unsupported operating system: $OS"
        exit 1
        ;;
esac

# Get latest release version
echo "Fetching latest release..."
LATEST_VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"v?([^"]+)".*/\1/')

if [ -z "$LATEST_VERSION" ]; then
    echo "Error: Could not determine latest version"
    exit 1
fi

echo "Latest version: $LATEST_VERSION"

# Construct download URL
DOWNLOAD_URL="https://github.com/$REPO/releases/download/v${LATEST_VERSION}/${BINARY_NAME}-${LATEST_VERSION}-${TARGET}.tar.gz"

# Create temporary directory
TMP_DIR=$(mktemp -d)
trap "rm -rf $TMP_DIR" EXIT

# Download and extract
echo "Downloading claudeforge v$LATEST_VERSION for $TARGET..."
curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/claudeforge.tar.gz"

echo "Extracting..."
tar -xzf "$TMP_DIR/claudeforge.tar.gz" -C "$TMP_DIR"

# Install binary
echo "Installing to $INSTALL_DIR..."
if [ -w "$INSTALL_DIR" ]; then
    mv "$TMP_DIR/$BINARY_NAME" "$INSTALL_DIR/"
else
    sudo mv "$TMP_DIR/$BINARY_NAME" "$INSTALL_DIR/"
fi

# Make executable
if [ -w "$INSTALL_DIR/$BINARY_NAME" ]; then
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
else
    sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"
fi

# Verify installation
if command -v claudeforge >/dev/null 2>&1; then
    echo "✅ ClaudeForge v$LATEST_VERSION installed successfully!"
    echo "Run 'claudeforge --help' to get started."
else
    echo "⚠️  Installation complete, but claudeforge is not in your PATH."
    echo "Add $INSTALL_DIR to your PATH or specify the full path to run claudeforge."
fi