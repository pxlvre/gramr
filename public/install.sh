#!/bin/sh
# Nothung installer script
# Usage: curl --proto '=https' --tlsv1.2 -sSf https://getnothung.pxlvre.dev | sh

set -e

# Configuration
NOTHUNG_VERSION="${NOTHUNG_VERSION:-latest}"
INSTALL_DIR="${NOTHUNG_INSTALL_DIR:-$HOME/.nothung}"
BIN_DIR="$INSTALL_DIR/bin"
REPO_URL="https://github.com/pxlvre/nothung"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
info() {
    printf "${GREEN}info${NC}: %s\n" "$1"
}

warn() {
    printf "${YELLOW}warning${NC}: %s\n" "$1" >&2
}

err() {
    printf "${RED}error${NC}: %s\n" "$1" >&2
    exit 1
}

# Detect OS and architecture
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"
    
    case "$OS" in
        Linux*)
            case "$ARCH" in
                x86_64) PLATFORM="linux-amd64" ;;
                aarch64|arm64) PLATFORM="linux-arm64" ;;
                *) err "Unsupported architecture: $ARCH" ;;
            esac
            ;;
        Darwin*)
            case "$ARCH" in
                x86_64) PLATFORM="darwin-amd64" ;;
                arm64|aarch64) PLATFORM="darwin-arm64" ;;
                *) err "Unsupported architecture: $ARCH" ;;
            esac
            ;;
        *)
            err "Unsupported OS: $OS"
            ;;
    esac
    
    info "Detected platform: $PLATFORM"
}

# Check for required tools
check_requirements() {
    # Check for curl or wget
    if command -v curl >/dev/null 2>&1; then
        DOWNLOADER="curl"
    elif command -v wget >/dev/null 2>&1; then
        DOWNLOADER="wget"
    else
        err "Neither curl nor wget found. Please install one of them."
    fi
    
    # Check for tar
    if ! command -v tar >/dev/null 2>&1; then
        err "tar is required but not installed."
    fi
}

# Get the latest version from GitHub
get_latest_version() {
    if [ "$NOTHUNG_VERSION" = "latest" ]; then
        info "Fetching latest version..."
        if [ "$DOWNLOADER" = "curl" ]; then
            NOTHUNG_VERSION=$(curl -sL "$REPO_URL/releases/latest" | grep -o 'tag/[v.0-9]*' | head -1 | cut -d/ -f2)
        else
            NOTHUNG_VERSION=$(wget -qO- "$REPO_URL/releases/latest" | grep -o 'tag/[v.0-9]*' | head -1 | cut -d/ -f2)
        fi
        
        if [ -z "$NOTHUNG_VERSION" ]; then
            err "Failed to fetch latest version"
        fi
    fi
    
    info "Installing Nothung $NOTHUNG_VERSION"
}

# Download and install binaries
install_nothung() {
    DOWNLOAD_URL="$REPO_URL/releases/download/$NOTHUNG_VERSION/nothung-$PLATFORM.tar.gz"
    TMP_DIR="$(mktemp -d)"
    
    info "Downloading from $DOWNLOAD_URL"
    
    # Download the archive
    if [ "$DOWNLOADER" = "curl" ]; then
        curl -SfL "$DOWNLOAD_URL" -o "$TMP_DIR/nothung.tar.gz" || err "Failed to download Nothung"
    else
        wget -q "$DOWNLOAD_URL" -O "$TMP_DIR/nothung.tar.gz" || err "Failed to download Nothung"
    fi
    
    # Create installation directory
    mkdir -p "$BIN_DIR"
    
    # Extract binaries
    info "Extracting binaries to $BIN_DIR"
    tar -xzf "$TMP_DIR/nothung.tar.gz" -C "$TMP_DIR"
    
    # Move binaries to installation directory
    for binary in nothung wotan nothungup; do
        if [ -f "$TMP_DIR/$binary" ]; then
            mv "$TMP_DIR/$binary" "$BIN_DIR/"
            chmod +x "$BIN_DIR/$binary"
            info "Installed $binary"
        fi
    done
    
    # Clean up
    rm -rf "$TMP_DIR"
}

# Check if installation directory is in PATH
check_path() {
    case ":$PATH:" in
        *":$BIN_DIR:"*)
            info "Installation directory is already in PATH"
            ;;
        *)
            warn "Installation directory is not in PATH"
            
            # Detect shell and provide instructions
            SHELL_NAME="$(basename "$SHELL")"
            case "$SHELL_NAME" in
                bash)
                    PROFILE="$HOME/.bashrc"
                    ;;
                zsh)
                    PROFILE="$HOME/.zshrc"
                    ;;
                fish)
                    PROFILE="$HOME/.config/fish/config.fish"
                    ;;
                *)
                    PROFILE="your shell profile"
                    ;;
            esac
            
            echo ""
            echo "To add Nothung to your PATH, run:"
            echo ""
            echo "    echo 'export PATH=\"$BIN_DIR:\$PATH\"' >> $PROFILE"
            echo "    source $PROFILE"
            echo ""
            ;;
    esac
}

# Main installation flow
main() {
    echo ""
    echo "⚔️  Nothung Installer"
    echo "===================="
    echo ""
    
    detect_platform
    check_requirements
    get_latest_version
    install_nothung
    check_path
    
    echo ""
    info "Installation complete!"
    echo ""
    echo "Get started with:"
    echo "    nothung --help    # CLI usage"
    echo "    wotan            # Interactive wizard"
    echo ""
    echo "Documentation: https://pxlvre.github.io/nothung"
    echo ""
}

# Run the installer
main "$@"