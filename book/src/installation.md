# Installation

There are several ways to install Gramr depending on your needs and platform.

## 🚀 Quick Install (Recommended)

The fastest way to get started:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://getgramr.pxlvre.dev/install.sh | sh
```

This script will:

- Detect your platform (Linux/macOS, x86/ARM)
- Download the latest binaries
- Install `gramr`, `wotan`, and `gramrup`
- Provide PATH setup instructions

**Supported Platforms:**

- ✅ Linux (x86_64, ARM64)
- ✅ macOS (Intel, Apple Silicon)
- ❌ Windows (planned for v0.2.0)

## 📦 Alternative Installation Methods

### Using Cargo (Rust Package Manager)

```bash
# Install library from crates.io
cargo install gramr

# Install CLI tools from GitHub
cargo install --git https://github.com/pxlvre/gramr gramr-cli
cargo install --git https://github.com/pxlvre/gramr wotan
cargo install --git https://github.com/pxlvre/gramr gramrup
```

### Using Docker

```bash
# Pull the production image
docker pull ghcr.io/pxlvre/gramr:latest

# Run commands
docker run --rm -v $(pwd):/workspace ghcr.io/pxlvre/gramr:latest gramr --help

# Development environment
git clone https://github.com/pxlvre/gramr
cd gramr
docker-compose up -d gramr-dev
```

### Build from Source

```bash
# Clone the repository
git clone https://github.com/pxlvre/gramr
cd gramr

# Build all components
cargo build --release --all

# Install locally
cargo install --path cli
cargo install --path wotan
cargo install --path gramrup
```

## 🔧 Installing Dependencies

Gramr works best with these tools installed:

### Required

- **Rust** 1.70+ and **Cargo** - [Install Rust](https://rustup.rs/)

### For Solidity Development (Recommended)

- **Foundry** - Smart contract development framework
  ```bash
  curl -L https://foundry.paradigm.xyz | bash
  foundryup
  ```

### For Rust/Stylus Development (Experimental)

- **Rust Nightly** - Required for OpenZeppelin Stylus
  ```bash
  rustup install nightly
  rustup default nightly
  ```
- **cargo-stylus** - Stylus development tools (optional)
  ```bash
  cargo install cargo-stylus
  ```

## 🛠️ Using gramrup (Advanced)

`gramrup` is Gramr's installer that can manage dependencies:

```bash
# Check system dependencies
gramrup check

# Install Gramr with dependency checking
gramrup install

# Install options
gramrup install --force          # Force reinstall
gramrup install --local          # Install from local source
gramrup install --skip-checks    # Skip dependency verification

# Update to latest version
gramrup update

# Uninstall
gramrup uninstall
```

### gramrup Features

- **Dependency Detection** - Automatically checks for Rust, Cargo, Foundry
- **Interactive Installation** - Prompts to install missing dependencies
- **Platform Support** - Linux and macOS with architecture detection
- **Self-Management** - Update and uninstall capabilities

## 📍 Installation Verification

After installation, verify everything works:

```bash
# Check versions
gramr --version
wotan --version
gramrup --version

# Verify Foundry integration (if installed)
forge --version
anvil --version
cast --version

# Test contract generation
mkdir test-project && cd test-project
forge init --force
gramr new contract TestToken --solidity --oz-erc20
forge build
```

## 🔧 PATH Configuration

After installation, you may need to update your PATH:

### Bash/Zsh

```bash
echo 'export PATH="$HOME/.gramr/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Fish Shell

```bash
echo 'set -gx PATH $HOME/.gramr/bin $PATH' >> ~/.config/fish/config.fish
```

### Manual PATH Setup

Add this to your shell profile:

```bash
export PATH="$HOME/.gramr/bin:$PATH"
```

## 🐳 Docker Setup

For containerized development:

```bash
# Clone repository
git clone https://github.com/pxlvre/gramr
cd gramr

# Start development environment
docker-compose up -d gramr-dev

# Enter container
docker-compose exec gramr-dev bash

# Test inside container
gramr --version
forge --version
```

## 🚨 Troubleshooting

### Common Issues

**"Command not found" after installation**

- Check your PATH includes the installation directory
- Restart your terminal or run `source ~/.bashrc`

**Permission denied errors**

- Make sure you have write access to the installation directory
- Try installing to a different directory with `GRAMR_INSTALL_DIR`

**Foundry not detected**

- Install Foundry: `curl -L https://foundry.paradigm.xyz | bash && foundryup`
- Make sure `forge` is in your PATH

**Rust version issues**

- Update Rust: `rustup update`
- For Stylus: `rustup install nightly && rustup default nightly`

### Manual Uninstall

If you need to manually remove Gramr:

```bash
# Remove binaries
rm -rf ~/.gramr

# Remove from PATH (edit your shell profile)
# Remove the line: export PATH="$HOME/.gramr/bin:$PATH"
```

## 🔄 Updating

To update to the latest version:

```bash
# Using the installer
curl --proto '=https' --tlsv1.2 -sSf https://getgramr.pxlvre.dev/install.sh | sh

# Using gramrup
gramrup update

# Using cargo (if installed via cargo)
cargo install --git https://github.com/pxlvre/gramr gramr-cli --force
```

## 🎯 Next Steps

Once installed, check out:

- [Quick Start Guide](./quick-start.md) - Get up and running in 5 minutes
- [Wotan Wizard](./wotan.md) - Interactive contract creation
- [CLI Reference](./cli-reference.md) - Complete command documentation
