# System Requirements

This page outlines the system requirements and dependencies for using Nothung effectively.

## Minimum Requirements

### Operating System

**Supported Platforms:**
- ✅ **Linux** - All major distributions (Ubuntu, Fedora, Arch, etc.)
- ✅ **macOS** - Intel (x86_64) and Apple Silicon (ARM64)
- ❌ **Windows** - Planned for v0.2.0 (use WSL2 for now)

### Core Dependencies

#### Rust & Cargo
**Required for all functionality**

```bash
# Install via rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Minimum version
rustc --version  # Should be 1.70+
```

**Why needed:**
- Nothung is built in Rust
- Required for building from source
- Needed for Rust/Stylus contract generation

## Solidity Development

### Foundry (Highly Recommended)

**Components:**
- `forge` - Solidity compiler and test runner
- `anvil` - Local blockchain for testing  
- `cast` - Command-line tool for Ethereum interaction

**Installation:**
```bash
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

**Verification:**
```bash
forge --version
anvil --version
cast --version
```

**Why needed:**
- Primary target for Solidity contract generation
- Automatic dependency management (OpenZeppelin installation)
- Test compilation and execution
- Deployment script execution

### Alternative: Hardhat (Limited Support)

While Nothung generates Foundry-compatible code, you can use generated contracts in Hardhat projects:

```bash
npm install --save-dev hardhat @openzeppelin/contracts
```

**Limitations:**
- No automatic OpenZeppelin installation
- Generated tests are Foundry-specific
- Deployment scripts require adaptation

## Rust/Stylus Development

### Rust Nightly (Required for Stylus)

```bash
# Install nightly toolchain
rustup install nightly

# Set as default (recommended for Stylus development)
rustup default nightly

# Or use for specific projects
rustup override set nightly
```

**Why nightly:**
- OpenZeppelin Stylus contracts require nightly features
- WASM compilation optimizations
- Latest language features for smart contracts

### WebAssembly Target

```bash
# Add WASM compilation target
rustup target add wasm32-unknown-unknown
```

### Stylus CLI (Optional)

```bash
# For deployment to Arbitrum Stylus
cargo install cargo-stylus
```

**Features:**
- Deploy contracts to Arbitrum Stylus
- Local testing and simulation
- Contract interaction tools

## Development Tools (Optional)

### Version Control
```bash
# Git (usually pre-installed)
git --version
```

### Code Editors

**VS Code (Recommended)**
- Rust Analyzer extension
- Solidity extensions
- Foundry extensions

**Other Options:**
- Vim/Neovim with rust-analyzer
- IntelliJ IDEA with Rust plugin
- Emacs with rust-mode

### Container Runtime (Optional)

**Docker (for containerized development)**
```bash
# Docker Engine
docker --version

# Docker Compose
docker-compose --version
```

## Hardware Requirements

### Minimum Specifications

- **CPU:** Any modern 64-bit processor
- **RAM:** 4GB (8GB recommended for large projects)
- **Storage:** 2GB free space (more for dependencies)
- **Network:** Internet connection for dependency downloads

### Recommended Specifications

- **CPU:** Multi-core processor (faster compilation)
- **RAM:** 8GB+ (16GB for large workspaces)
- **Storage:** SSD for faster builds
- **Network:** Stable broadband for dependency management

## Network Requirements

### Required Connections

**For Installation:**
- `sh.rustup.rs` - Rust installation
- `foundry.paradigm.xyz` - Foundry installation
- `getnothung.pxlvre.dev` - Nothung installer
- `github.com` - Source code and releases
- `crates.io` - Rust dependencies

**For Development:**
- `github.com` - OpenZeppelin contract downloads
- `registry.npmjs.org` - Node.js dependencies (if using Hardhat)

### Firewall Considerations

Ensure outbound HTTPS (443) access to:
- GitHub (github.com)
- Rust registry (crates.io)
- NPM registry (registry.npmjs.org)
- Installation domains

## Platform-Specific Notes

### Linux

**Debian/Ubuntu:**
```bash
# Required packages for compilation
sudo apt update
sudo apt install build-essential curl git

# Additional dependencies for some crates
sudo apt install pkg-config libssl-dev
```

**Fedora/RHEL:**
```bash
# Development tools
sudo dnf groupinstall "Development Tools"
sudo dnf install curl git openssl-devel
```

**Arch Linux:**
```bash
# Base development packages
sudo pacman -S base-devel curl git openssl
```

### macOS

**Xcode Command Line Tools:**
```bash
xcode-select --install
```

**Homebrew (recommended for additional tools):**
```bash
# Install Homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install additional tools
brew install git
```

**Apple Silicon Notes:**
- All Nothung binaries support ARM64
- Foundry supports Apple Silicon natively
- Rust compilation is optimized for M1/M2/M3

### Windows (WSL2)

Since native Windows support is planned for v0.2.0, current Windows users should use WSL2:

```bash
# Install WSL2
wsl --install

# Install Ubuntu or preferred distribution
wsl --install -d Ubuntu

# Follow Linux instructions inside WSL2
```

## Verification Script

Use this script to check all requirements:

```bash
#!/bin/bash
echo "🔍 Checking Nothung requirements..."

# Check Rust
if command -v rustc &> /dev/null; then
    echo "✅ Rust: $(rustc --version)"
else
    echo "❌ Rust: Not installed"
fi

# Check Cargo
if command -v cargo &> /dev/null; then
    echo "✅ Cargo: $(cargo --version)"
else
    echo "❌ Cargo: Not installed"
fi

# Check Foundry
if command -v forge &> /dev/null; then
    echo "✅ Forge: $(forge --version)"
else
    echo "⚠️  Forge: Not installed (recommended for Solidity)"
fi

if command -v anvil &> /dev/null; then
    echo "✅ Anvil: $(anvil --version)"
else
    echo "⚠️  Anvil: Not installed (recommended for testing)"
fi

# Check Git
if command -v git &> /dev/null; then
    echo "✅ Git: $(git --version)"
else
    echo "⚠️  Git: Not installed (recommended)"
fi

# Check Docker (optional)
if command -v docker &> /dev/null; then
    echo "✅ Docker: $(docker --version)"
else
    echo "ℹ️  Docker: Not installed (optional)"
fi

echo "🎯 Requirements check complete!"
```

Save as `check-requirements.sh` and run:
```bash
chmod +x check-requirements.sh
./check-requirements.sh
```

## Troubleshooting

### Common Issues

**"rustc: command not found"**
- Install Rust via rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Restart terminal or run: `source ~/.cargo/env`

**"forge: command not found"**
- Install Foundry: `curl -L https://foundry.paradigm.xyz | bash && foundryup`
- Check PATH includes `~/.foundry/bin`

**Compilation errors with Stylus**
- Ensure using Rust nightly: `rustup default nightly`
- Add WASM target: `rustup target add wasm32-unknown-unknown`

**Permission denied errors**
- Check file permissions for installation directories
- Use package managers (apt, brew) for system dependencies
- Avoid using `sudo` with Rust/Cargo commands

### Performance Optimization

**Faster Rust Compilation:**
```bash
# Use more CPU cores
export CARGO_BUILD_JOBS=8

# Enable parallel frontend
export RUSTFLAGS="-C target-cpu=native"

# Use faster linker (Linux)
sudo apt install lld
export RUSTFLAGS="-C link-arg=-fuse-ld=lld"
```

**SSD Storage:**
- Install dependencies on SSD storage
- Use SSD for Cargo cache (`~/.cargo`)
- Place project files on SSD

## Upgrading Dependencies

### Keep Rust Updated
```bash
rustup update stable
rustup update nightly
```

### Keep Foundry Updated
```bash
foundryup
```

### Keep Nothung Updated
```bash
# Using installer
curl --proto '=https' --tlsv1.2 -sSf https://getnothung.pxlvre.dev | sh

# Using nothungup
nothungup update
```

## Next Steps

Once you have the requirements installed:

1. **Install Nothung** - Follow the [Installation Guide](./installation.md)
2. **Test Installation** - Try the [Quick Start](./quick-start.md)
3. **Explore Features** - Read about [CLI Reference](./cli-reference.md)
4. **Get Help** - Check [Troubleshooting](./troubleshooting.md) if needed