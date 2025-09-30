# ⚔️ Nothung

> The legendary sword that forges smart contracts

A blazing-fast toolkit for scaffolding OpenZeppelin-powered smart contracts, tests, and deployment scripts. Supports Solidity (Foundry) and Rust (Arbitrum Stylus) projects.

🦀 Built with Rust 🦀

[![CI](https://github.com/pxlvre/nothung/workflows/CI/badge.svg)](https://github.com/pxlvre/nothung/actions)
[![Release](https://github.com/pxlvre/nothung/workflows/Release/badge.svg)](https://github.com/pxlvre/nothung/releases)
[![Docker](https://github.com/pxlvre/nothung/workflows/Docker/badge.svg)](https://github.com/pxlvre/nothung/pkgs/container/nothung)

## 🚀 Quick Start

### One-Line Installation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://getnothung.pxlvre.dev | sh
```

### Interactive Wizard

```bash
wotan
```

The wizard guides you through creating any type of smart contract with zero configuration!

### CLI Usage

```bash
# ERC20 Token
nothung new contract MyToken --solidity --oz-erc20

# ERC721 NFT with extensions
nothung new contract MyNFT --solidity --oz-erc721 --extensions enumerable,burnable
```

## 📖 Documentation

- **[Installation Guide](https://getnothung.pxlvre.dev/docs/installation)** - Complete installation instructions
- **[Quick Start](https://getnothung.pxlvre.dev/docs/quick-start)** - Get up and running in 5 minutes
- **[CLI Reference](https://getnothung.pxlvre.dev/docs/cli-reference)** - Complete command documentation
- **[Wotan Wizard](https://getnothung.pxlvre.dev/docs/wotan)** - Interactive mode guide
- **[API Documentation](https://docs.rs/nothung)** - Rust library docs

## 🏗️ Architecture

Nothung is organized as a Rust workspace with four components:

### 📦 Components

- **`nothung`** (library) - Core functionality for contract generation
- **`nothung-cli`** - Command-line interface 
- **`wotan`** - Interactive wizard for guided creation
- **`nothungup`** - Installer with dependency management

## 🎯 Features

### Smart Contract Types

✅ **Contracts** - ERC20, ERC721, ERC1155 tokens  
✅ **Libraries** - Reusable utility functions  
✅ **Interfaces** - Contract interface definitions  
✅ **Abstract Contracts** - Abstract base contracts  
✅ **Tests** - Foundry test files  
✅ **Scripts** - Deployment scripts  

### Token Standards & Extensions

**ERC20** (11 extensions): permit, burnable, capped, pausable, votes, wrapper, flashmint, temporaryapproval, bridgeable, erc1363, erc4626

**ERC721** (8 extensions): pausable, burnable, consecutive, uristorage, votes, royalty, wrapper, enumerable

**ERC1155** (4 extensions): pausable, burnable, supply, uristorage

### Language Support

#### Solidity (Full Support)
- ✅ All contract types and extensions
- ✅ Upgradeable patterns
- ✅ Test and script generation
- ✅ Complete OpenZeppelin integration

#### Rust/Stylus (Experimental)  
- ✅ Basic ERC20, ERC721, ERC1155 contracts
- ✅ Library generation
- ❌ Extensions (coming soon)
- ❌ Upgradeable contracts (not yet supported by OpenZeppelin Stylus)

## 📥 Installation

### Option 1: One-Line Installer (Recommended)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://getnothung.pxlvre.dev | sh
```

This installs `nothung`, `wotan`, and `nothungup` binaries.

### Option 2: Using Cargo

```bash
# Install from crates.io (library only)
cargo install nothung

# Install from GitHub (all tools)
cargo install --git https://github.com/pxlvre/nothung nothung-cli
cargo install --git https://github.com/pxlvre/nothung wotan
cargo install --git https://github.com/pxlvre/nothung nothungup
```

### Option 3: Using Docker

```bash
# Run with Docker
docker run --rm -v $(pwd):/workspace ghcr.io/pxlvre/nothung:latest nothung --help

# Development environment
docker-compose up -d nothung-dev
```

### Option 4: Build from Source

```bash
git clone https://github.com/pxlvre/nothung
cd nothung
cargo build --release --all
```

## 🧙‍♂️ Interactive Wizard

Start the interactive wizard for guided contract creation:

```bash
wotan
# OR
nothung wizard
```

The wizard handles:
- Resource type selection (contract, library, script, test)
- Language choice (Solidity or Rust/Stylus)
- Token standard selection with previews
- Extension configuration
- Generation options (tests, scripts, upgradeable)

## 🔧 CLI Reference

### Basic Usage

```bash
nothung new <TYPE> <NAME> [OPTIONS]
```

### Contract Generation

```bash
# Basic contract
nothung new contract MyContract --solidity

# ERC20 Token
nothung new contract MyToken --solidity --oz-erc20

# ERC721 NFT with extensions  
nothung new contract MyNFT --solidity --oz-erc721 --extensions enumerable,burnable

# ERC1155 Multi-token
nothung new contract GameAssets --solidity --oz-erc1155 --extensions supply,pausable

# Upgradeable contract
nothung new contract MyToken --solidity --oz-erc20 --oz-upgradeable

# With tests and deployment script
nothung new contract MyToken --solidity --oz-erc20 --with-test --with-script
```

### Other Resource Types

```bash
# Library
nothung new library MathUtils --solidity

# Interface  
nothung new interface IMyToken --solidity

# Abstract contract
nothung new abstract BaseToken --solidity

# Test file
nothung new test TokenTest --solidity

# Deployment script
nothung new script DeployToken --solidity
```

### Rust/Stylus Contracts

```bash
# Basic Rust contracts for Arbitrum Stylus
nothung new contract MyToken --rust-stylus --oz-erc20
nothung new contract MyNFT --rust-stylus --oz-erc721
nothung new library Utils --rust-stylus
```

### Options

**Languages:**
- `--solidity` - Generate Solidity code (Foundry projects)
- `--rust-stylus` - Generate Rust code (Arbitrum Stylus projects)

**Token Standards:**
- `--oz-erc20` - OpenZeppelin ERC20 token
- `--oz-erc721` - OpenZeppelin ERC721 NFT
- `--oz-erc1155` - OpenZeppelin ERC1155 multi-token

**Patterns:**
- `--oz-upgradeable` - Use upgradeable contract patterns
- `--extensions <LIST>` - Comma-separated extensions

**Generation:**
- `--with-test` - Generate test file
- `--with-script` - Generate deployment script
- `--with-section-markers` - Add organized comment sections

**Configuration:**
- `--pragma <VERSION>` - Solidity version (default: 0.8.30)
- `--license <LICENSE>` - SPDX license identifier (default: MIT)

## 🐳 Docker Usage

### Quick Commands

```bash
# Production container
docker run --rm -v $(pwd):/workspace ghcr.io/pxlvre/nothung:latest \
  nothung new contract MyToken --solidity --oz-erc20

# Development environment  
docker-compose up -d nothung-dev
docker-compose exec nothung-dev bash
```

### Available Images

- `ghcr.io/pxlvre/nothung:latest` - Production (200MB)
- `ghcr.io/pxlvre/nothung-dev:latest` - Development (1.5GB)
- `ghcr.io/pxlvre/nothung-docs:latest` - Documentation server

## 🛠️ System Requirements

### Required
- **Rust** 1.70+ and **Cargo**

### For Solidity Projects (Recommended)
- **Foundry** (`forge`, `anvil`, `cast`)

### For Rust/Stylus Projects (Experimental)
- **Rust nightly**
- **cargo-stylus** (optional)

## 📚 Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
nothung = "0.1"
```

Use programmatically:

```rust
use nothung::{GenericContractGenerator, ContractType, ProjectType, Language};

fn main() -> nothung::Result<()> {
    let project = ProjectType::detect_project_type(".")?;
    
    let generator = GenericContractGenerator::new(
        project,
        Language::Solidity,
        "MyToken",
        ContractType::ERC20,
        None,
    );
    
    generator.generate()?;
    Ok(())
}
```

## 🗺️ Roadmap

### v0.1.0 (Current)
- ✅ Complete Solidity support
- ✅ Interactive wizard (Wotan)
- ✅ Experimental Rust/Stylus support
- ✅ Docker containerization
- ✅ One-line installer

### v0.2.0 (Planned)
- [ ] Full Rust/Stylus extension support
- [ ] Cairo/StarkNet support
- [ ] Template customization
- [ ] Windows support

## 🤝 Contributing

We welcome contributions! See our [Contributing Guide](https://getnothung.pxlvre.dev/docs/contributing).

## 📜 License

MIT or Apache-2.0

## 🙏 Acknowledgments

Built with ❤️ by [Pol Vidal](https://github.com/pxlvre) • pxlvre.eth

Special thanks to the OpenZeppelin, Foundry, and Arbitrum teams.

---

**Links:**
- [Documentation](https://getnothung.pxlvre.dev/docs)
- [GitHub Releases](https://github.com/pxlvre/nothung/releases)  
- [Docker Images](https://github.com/pxlvre/nothung/pkgs/container/nothung)
- [Rust Docs](https://docs.rs/nothung)