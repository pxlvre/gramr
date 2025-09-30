# Installation

There are several ways to install Nothung depending on your needs and platform.

## 🚀 Quick Install (Recommended)

The fastest way to get started:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://getnothung.pxlvre.dev | sh
```

This script will:
- Detect your platform (Linux/macOS, x86/ARM)
- Download the latest binaries
- Install `nothung`, `wotan`, and `nothungup`
- Provide PATH setup instructions

**Supported Platforms:**
- ✅ Linux (x86_64, ARM64)
- ✅ macOS (Intel, Apple Silicon)
- ❌ Windows (planned for v0.2.0)

## 📦 Alternative Installation Methods

### Using Cargo (Rust Package Manager)

```bash
# Install library from crates.io
cargo install nothung

# Install CLI tools from GitHub
cargo install --git https://github.com/pxlvre/nothung nothung-cli
cargo install --git https://github.com/pxlvre/nothung wotan
cargo install --git https://github.com/pxlvre/nothung nothungup
```

### Using Docker

```bash
# Pull the production image
docker pull ghcr.io/pxlvre/nothung:latest

# Run commands
docker run --rm -v $(pwd):/workspace ghcr.io/pxlvre/nothung:latest nothung --help

# Development environment
git clone https://github.com/pxlvre/nothung
cd nothung
docker-compose up -d nothung-dev
```

### Build from Source

```bash
# Clone the repository
git clone https://github.com/pxlvre/nothung
cd nothung

# Build all components
cargo build --release --all

# Install locally
cargo install --path cli
cargo install --path wotan
cargo install --path nothungup
```

## 🔧 Installing Dependencies

Nothung works best with these tools installed:

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

## 🛠️ Using Nothungup (Advanced)

`nothungup` is Nothung's installer that can manage dependencies:

```bash
# Check system dependencies
nothungup check

# Install Nothung with dependency checking
nothungup install

# Install options
nothungup install --force          # Force reinstall
nothungup install --local          # Install from local source
nothungup install --skip-checks    # Skip dependency verification

# Update to latest version
nothungup update

# Uninstall
nothungup uninstall
```

### Nothungup Features

- **Dependency Detection** - Automatically checks for Rust, Cargo, Foundry
- **Interactive Installation** - Prompts to install missing dependencies
- **Platform Support** - Linux and macOS with architecture detection
- **Self-Management** - Update and uninstall capabilities

## 📍 Installation Verification

After installation, verify everything works:

```bash
# Check versions
nothung --version
wotan --version
nothungup --version

# Verify Foundry integration (if installed)
forge --version
anvil --version
cast --version

# Test contract generation
mkdir test-project && cd test-project
forge init --force
nothung new contract TestToken --solidity --oz-erc20
forge build
```

## 🔧 PATH Configuration

After installation, you may need to update your PATH:

### Bash/Zsh
```bash
echo 'export PATH="$HOME/.nothung/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Fish Shell
```bash
echo 'set -gx PATH $HOME/.nothung/bin $PATH' >> ~/.config/fish/config.fish
```

### Manual PATH Setup
Add this to your shell profile:
```bash
export PATH="$HOME/.nothung/bin:$PATH"
```

## 🐳 Docker Setup

For containerized development:

```bash
# Clone repository
git clone https://github.com/pxlvre/nothung
cd nothung

# Start development environment
docker-compose up -d nothung-dev

# Enter container
docker-compose exec nothung-dev bash

# Test inside container
nothung --version
forge --version
```

## 🚨 Troubleshooting

### Common Issues

**"Command not found" after installation**
- Check your PATH includes the installation directory
- Restart your terminal or run `source ~/.bashrc`

**Permission denied errors**
- Make sure you have write access to the installation directory
- Try installing to a different directory with `NOTHUNG_INSTALL_DIR`

**Foundry not detected**
- Install Foundry: `curl -L https://foundry.paradigm.xyz | bash && foundryup`
- Make sure `forge` is in your PATH

**Rust version issues**
- Update Rust: `rustup update`
- For Stylus: `rustup install nightly && rustup default nightly`

### Manual Uninstall

If you need to manually remove Nothung:

```bash
# Remove binaries
rm -rf ~/.nothung

# Remove from PATH (edit your shell profile)
# Remove the line: export PATH="$HOME/.nothung/bin:$PATH"
```

## 🔄 Updating

To update to the latest version:

```bash
# Using the installer
curl --proto '=https' --tlsv1.2 -sSf https://getnothung.pxlvre.dev | sh

# Using nothungup
nothungup update

# Using cargo (if installed via cargo)
cargo install --git https://github.com/pxlvre/nothung nothung-cli --force
```

## 🎯 Next Steps

Once installed, check out:
- [Quick Start Guide](./quick-start.md) - Get up and running in 5 minutes
- [Wotan Wizard](./wotan.md) - Interactive contract creation
- [CLI Reference](./cli-reference.md) - Complete command documentation