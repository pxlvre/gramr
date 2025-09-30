#!/bin/sh
# Gramr installer script
# Usage: curl --proto '=https' --tlsv1.2 -sSf https://getgramr.pxlvre.dev/install.sh | sh

set -e

# Configuration
GRAMR_VERSION="${GRAMR_VERSION:-latest}"
INSTALL_DIR="${GRAMR_INSTALL_DIR:-$HOME/.gramr}"
BIN_DIR="$INSTALL_DIR/bin"
REPO_URL="https://github.com/pxlvre/gramr"

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
    if [ "$GRAMR_VERSION" = "latest" ]; then
        info "Fetching latest version..."
        if [ "$DOWNLOADER" = "curl" ]; then
            GRAMR_VERSION=$(curl -sL "https://api.github.com/repos/pxlvre/gramr/releases/latest" | grep '"tag_name"' | sed -E 's/.*"tag_name":\s*"([^"]+)".*/\1/')
        else
            GRAMR_VERSION=$(wget -qO- "https://api.github.com/repos/pxlvre/gramr/releases/latest" | grep '"tag_name"' | sed -E 's/.*"tag_name":\s*"([^"]+)".*/\1/')
        fi
        
        if [ -z "$GRAMR_VERSION" ]; then
            err "Failed to fetch latest version"
        fi
    fi
    
    info "Installing Gramr $GRAMR_VERSION"
}

# Download and install binaries
install_gramr() {
    DOWNLOAD_URL="$REPO_URL/releases/download/$GRAMR_VERSION/gramr-$PLATFORM.tar.gz"
    TMP_DIR="$(mktemp -d)"
    
    info "Downloading from $DOWNLOAD_URL"
    
    # Download the archive
    if [ "$DOWNLOADER" = "curl" ]; then
        curl -SfL "$DOWNLOAD_URL" -o "$TMP_DIR/gramr.tar.gz" || err "Failed to download Gramr"
    else
        wget -q "$DOWNLOAD_URL" -O "$TMP_DIR/gramr.tar.gz" || err "Failed to download Gramr"
    fi
    
    # Create installation directory
    mkdir -p "$BIN_DIR"
    
    # Extract binaries
    info "Extracting binaries to $BIN_DIR"
    tar -xzf "$TMP_DIR/gramr.tar.gz" -C "$TMP_DIR"
    
    # Move binaries to installation directory
    for binary in gramr wotan gramrup; do
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
            echo "To add Gramr to your PATH, run:"
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
    echo "⚔️  Gramr Installer"
    echo "===================="
    echo ""
    
    detect_platform
    check_requirements
    get_latest_version
    install_gramr
    check_path
    
    echo ""
    info "Installation complete!"
    echo ""
    echo "Get started with:"
    echo "    gramr --help    # CLI usage"
    echo "    wotan            # Interactive wizard"
    echo ""
    echo "Documentation: https://getgramr.pxlvre.dev/docs"
    echo ""
}

# Run the installer
main "$@"