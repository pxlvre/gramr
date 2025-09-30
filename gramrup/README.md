# gramrup

⚔️ Smart installer for Gramr and its dependencies on Unix-based systems.

## About

gramrup is the official installer for Gramr that automatically checks for system dependencies (Rust, Cargo, Foundry) and offers to install missing components. It installs both the Gramr CLI and Wotan interactive wizard, providing a seamless installation experience similar to `rustup` and `foundryup`.

## Installation

### Install gramrup

```bash
# From source
cargo install --path gramrup

# From GitHub
cargo install --git https://github.com/pxlvre/gramr gramrup
```

## Usage

### Install Gramr

```bash
# Interactive installation (checks dependencies)
gramrup install

# Skip dependency checks
gramrup install --skip-checks

# Force reinstall
gramrup install --force

# Install from local repository
gramrup install --local
```

### Other Commands

```bash
# Check system dependencies
gramrup check

# Update to latest version
gramrup update

# Uninstall Gramr
gramrup uninstall

# Show help
gramrup --help
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
4. **Gramr Installation** - Builds and installs the Gramr CLI and Wotan wizard
5. **Verification** - Confirms successful installation

## Example Sessions

### First-time Installation

```bash
$ gramrup install
⚔️  gramrup - Forging the legendary sword...

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

Installing Gramr from GitHub...
✓ Wotan wizard installed!
✓ Gramr installed successfully!

Get started with:
  → wotan                                    # Interactive wizard
  → gramr wizard                           # Also launches wizard
  → gramr new contract MyToken --solidity --oz-erc20  # Direct CLI
  → gramr new interface IMyToken --solidity           # Generate interface
  → gramr new abstract BaseToken --solidity           # Generate abstract contract
```

### Update Existing Installation

```bash
$ gramrup update
⚔️  Updating Gramr...

Installing Gramr from GitHub...
✓ Wotan wizard installed!
✓ Gramr updated successfully!
```

### Dependency Check Only

```bash
$ gramrup check
Checking system dependencies...

  ✓ Rust
  ✓ Cargo
  ✓ Forge
  ✓ Anvil
  ✓ Cast
```

## Command Reference

### `gramrup install`

Installs Gramr CLI, Wotan interactive wizard, and their dependencies.

**Options:**

- `--skip-checks` - Skip dependency verification
- `--local` - Install from local repository instead of GitHub
- `--force` - Force reinstall even if already installed

### `gramrup update`

Updates Gramr CLI and Wotan wizard to the latest version from GitHub. Equivalent to `install --force` but skips dependency checks.

### `gramrup uninstall`

Removes Gramr CLI from the system using `cargo uninstall gramr-cli`. Note: This doesn't automatically uninstall Wotan - use `cargo uninstall wotan` separately if needed.

### `gramrup check`

Displays the status of all system dependencies without making changes.

## Installation Sources

### GitHub (Default)

Downloads and installs from the official repository at `github.com/pxlvre/gramr`.

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

gramrup uses these installation methods:

- **Rust**: Downloads and runs the official rustup installer
- **Foundry**: Downloads and runs the official foundryup installer
- **Gramr**: Uses `cargo install` to build both CLI and Wotan wizard from source

## Future Plans

- Windows support via PowerShell scripts
- macOS package installer (.pkg)
- Linux distribution packages (apt, yum, pacman)
- Shell completion installation
- Configuration file support

## See Also

- [Gramr CLI](../cli/) - Direct command-line interface
- [Wotan](../wotan/) - Interactive wizard for guided contract creation
- [Gramr library](../lib/) - Core functionality
- [Main README](../README.md) - Full project documentation
