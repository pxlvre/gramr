# Nothungup

⚔️ Smart installer for Nothung and its dependencies on Unix-based systems.

## About

Nothungup is the official installer for Nothung that automatically checks for system dependencies (Rust, Cargo, Foundry) and offers to install missing components. It provides a seamless installation experience similar to `rustup` and `foundryup`.

## Installation

### Install nothungup

```bash
# From source
cargo install --path nothungup

# From GitHub
cargo install --git https://github.com/pxlvre/nothung nothungup
```

## Usage

### Install Nothung

```bash
# Interactive installation (checks dependencies)
nothungup install

# Skip dependency checks  
nothungup install --skip-checks

# Force reinstall
nothungup install --force

# Install from local repository
nothungup install --local
```

### Other Commands

```bash
# Check system dependencies
nothungup check

# Update to latest version
nothungup update

# Uninstall Nothung
nothungup uninstall

# Show help
nothungup --help
```

## System Requirements

### Supported Platforms
- ✅ Linux (all distributions)
- ✅ macOS
- ✅ Unix-like systems
- ❌ Windows (planned for future release)

### Dependencies Managed

**Required:**
- **Rust** - Programming language and toolchain
- **Cargo** - Rust package manager

**Recommended:**
- **Foundry** - Smart contract development framework
  - `forge` - Compiler and test runner
  - `anvil` - Local blockchain
  - `cast` - Contract interaction CLI

## Installation Process

1. **Dependency Check** - Scans system for required tools
2. **Interactive Prompts** - Asks permission to install missing dependencies
3. **Automatic Installation** - Downloads and installs Rust/Foundry if needed
4. **Nothung Installation** - Builds and installs the Nothung CLI
5. **Verification** - Confirms successful installation

## Example Sessions

### First-time Installation

```bash
$ nothungup install
⚔️  Nothungup - Forging the legendary sword...

Checking system dependencies...

  ✓ Rust
  ✓ Cargo  
  ✗ Forge (not installed)
  ✗ Anvil (not installed)
  ✗ Cast (not installed)

? Foundry (forge, anvil, cast) is not installed. Install it now? (yes/no): yes

Installing Foundry...
✓ Foundry installed!
  Add '$HOME/.foundry/bin' to your PATH

Installing Nothung from GitHub...
✓ Nothung installed successfully!

Get started with:
  → nothung new contract MyToken --solidity --oz-erc20
```

### Update Existing Installation

```bash
$ nothungup update
⚔️  Updating Nothung...

Installing Nothung from GitHub...
✓ Nothung updated successfully!
```

### Dependency Check Only

```bash
$ nothungup check
Checking system dependencies...

  ✓ Rust
  ✓ Cargo
  ✓ Forge
  ✓ Anvil  
  ✓ Cast
```

## Command Reference

### `nothungup install`

Installs Nothung and its dependencies.

**Options:**
- `--skip-checks` - Skip dependency verification
- `--local` - Install from local repository instead of GitHub
- `--force` - Force reinstall even if already installed

### `nothungup update`

Updates Nothung to the latest version from GitHub. Equivalent to `install --force` but skips dependency checks.

### `nothungup uninstall`

Removes Nothung from the system using `cargo uninstall nothung-cli`.

### `nothungup check`

Displays the status of all system dependencies without making changes.

## Installation Sources

### GitHub (Default)
Downloads and installs from the official repository at `github.com/pxlvre/nothung`.

### Local Development
Uses `--local` flag to build and install from the current directory. Useful for testing local changes.

## Troubleshooting

### Permission Issues
If you encounter permission errors, ensure you have write access to:
- `~/.cargo/bin` (for Rust tools)
- `~/.foundry/bin` (for Foundry tools)

### PATH Configuration
After installation, you may need to update your shell's PATH:

```bash
# Add to ~/.bashrc, ~/.zshrc, etc.
export PATH="$HOME/.cargo/bin:$HOME/.foundry/bin:$PATH"

# Reload your shell
source ~/.bashrc  # or ~/.zshrc
```

### Network Issues
If downloads fail, check your internet connection and try again. The installer uses HTTPS for all downloads.

## Under the Hood

Nothungup uses these installation methods:

- **Rust**: Downloads and runs the official rustup installer
- **Foundry**: Downloads and runs the official foundryup installer  
- **Nothung**: Uses `cargo install` to build from source

## Future Plans

- Windows support via PowerShell scripts
- macOS package installer (.pkg)  
- Linux distribution packages (apt, yum, pacman)
- Shell completion installation
- Configuration file support

## See Also

- [Nothung CLI](../cli/) - The tool being installed
- [Nothung library](../lib/) - Core functionality
- [Main README](../README.md) - Full project documentation