# ⚔️ Nothung

> The legendary sword that reforges smart contracts

A toolkit for scaffolding OpenZeppelin-powered Solidity smart contracts, tests, and deployment scripts in Foundry projects.

🦀 Built with Rust 🦀

## 🏗️ Architecture

Nothung is organized as a Rust workspace with three distinct components:

### 📦 Packages

- **`nothung`** (library) - Core functionality for contract generation that can be used programmatically
- **`nothung-cli`** - Command-line interface built on top of the library
- **`nothungup`** - Installer utility for Unix-based systems (Linux/macOS) with dependency checking

## 🚀 Installation

### Using nothungup (Recommended for Linux/macOS)

```bash
# Install the installer
cargo install --git https://github.com/pxlvre/nothung nothungup

# Run installer (checks and installs Rust, Cargo, Foundry if needed)
nothungup install

# Available options with short flags
nothungup install -f          # Force reinstall
nothungup install -l          # Install from local repository (for development)
nothungup install -s          # Skip dependency checks (faster if deps exist)

# Self-management
nothungup self uninstall      # Uninstall nothungup itself
```

**Platform Support:**

- ✅ **Linux** - All major distributions
- ✅ **macOS** - Intel and Apple Silicon (M1/M2/M3)

### Manual Installation

```bash
# Install the CLI directly
cargo install --git https://github.com/pxlvre/nothung nothung-cli
```

### Build from Source

```bash
git clone https://github.com/pxlvre/nothung
cd nothung
cargo install --path cli
```

## 📖 Usage

### CLI Usage

#### Basic Commands

```bash
# Generate a basic contract
nothung new contract MyToken --solidity

# Generate an ERC20 token
nothung new contract MyToken --solidity --oz-erc20

# Generate an ERC721 NFT with extensions
nothung new contract MyNFT --solidity --oz-erc721 --extensions enumerable,burnable,royalty

# Generate an ERC1155 multi-token with extensions
nothung new contract MultiToken --solidity --oz-erc1155 --extensions supply,pausable

# Generate with test and deployment script
nothung new contract MyToken --solidity --oz-erc20 --with-test --with-script
```

#### Command Structure

```bash
nothung new <TYPE> <NAME> [OPTIONS]
```

**Arguments:**

- `TYPE`: `contract`, `script`, or `test`
- `NAME`: Name of the resource

**Options:**

- `--solidity` - Generate Solidity code (required)
- `--pragma <VERSION>` - Solidity version (default: 0.8.30)
- `--license <LICENSE>` - SPDX identifier (default: UNLICENSED)
- `--oz-erc20` - Inherit from OpenZeppelin ERC20
- `--oz-erc721` - Inherit from OpenZeppelin ERC721
- `--oz-erc1155` - Inherit from OpenZeppelin ERC1155
- `--oz-upgradeable` - Use upgradeable contracts
- `--extensions <LIST>` - Comma-separated token extensions
- `--with-test` - Generate test file
- `--with-script` - Generate deployment script

### Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
nothung = { git = "https://github.com/pxlvre/nothung" }
```

Use programmatically in Rust:

```rust
use nothung::{ContractBuilder, ContractType, TokenExtension, FoundryProject};

fn main() -> nothung::Result<()> {
    // Generate contract source code
    let source = ContractBuilder::new("MyToken")
        .contract_type(ContractType::ERC20)
        .pragma("0.8.30")
        .license("MIT")
        .build();

    println!("{}", source);

    // Or write directly to a Foundry project
    let project = FoundryProject::detect()?;

    ContractBuilder::new("MyNFT")
        .contract_type(ContractType::MultiInheritance {
            base_type: Box::new(ContractType::ERC721),
            extensions: vec![
                TokenExtension::ERC721Enumerable,
                TokenExtension::ERC721Burnable,
            ],
        })
        .generate(project)?;

    Ok(())
}
```

## 🎯 Features

### Token Standards

- ✅ **ERC20** - Fungible tokens
- ✅ **ERC721** - Non-fungible tokens (NFTs)
- ✅ **ERC1155** - Multi-token standard
- ✅ **Upgradeable** variants of all standards

### OpenZeppelin Extensions

#### ERC20 Extensions (11 supported)

- `permit` - Gasless approvals (EIP-2612)
- `burnable` - Token burning capability
- `capped` - Maximum supply limit
- `pausable` - Emergency pause functionality
- `votes` - On-chain voting & delegation
- `wrapper` - Wrap other ERC20 tokens
- `flashmint` - Flash loan support
- `temporaryapproval` - Single-transaction approvals
- `bridgeable` - Cross-chain compatibility
- `erc1363` - Payable token standard
- `erc4626` - Tokenized vault standard

#### ERC721 Extensions (8 supported)

- `pausable` - Emergency pause functionality
- `burnable` - NFT burning capability
- `consecutive` - Efficient batch minting
- `uristorage` - Dynamic metadata URIs
- `votes` - NFT-based voting
- `royalty` - ERC2981 royalty standard
- `wrapper` - Wrap other NFTs
- `enumerable` - Token enumeration

#### ERC1155 Extensions (4 supported)

- `pausable` - Emergency pause functionality
- `burnable` - Multi-token burning
- `supply` - Track token supplies
- `uristorage` - Per-token URI storage

### Cross-Compatible Extensions

Extensions like `burnable` and `pausable` automatically adapt to the base token type:

- Using `--oz-erc721 --extensions burnable` applies ERC721Burnable
- Using `--oz-erc1155 --extensions burnable` applies ERC1155Burnable

## 🛠️ System Requirements

### Required

- **Rust** 1.70+ - Programming language and toolchain
- **Cargo** - Rust package manager

### Recommended

- **Foundry** - Smart contract development framework
  - `forge` - Compiler and test runner
  - `anvil` - Local blockchain
  - `cast` - Contract interaction CLI

## 🔧 Development

```bash
# Clone the repository
git clone https://github.com/pxlvre/nothung
cd nothung

# Build all packages
cargo build --workspace

# Run tests
cargo test --workspace

# Build optimized binaries
cargo build --release --workspace

# Install locally for testing
cargo install --path cli
cargo install --path nothungup
```

## 🗺️ Roadmap

### v1.0 (Current)

- ✅ Solidity contract generation
- ✅ Full OpenZeppelin integration
- ✅ All token extensions support
- ✅ Library/CLI separation
- ✅ Unix installer with dependency management
- ✅ Multi-inheritance support

### Future Versions

- [ ] Vyper support
- [ ] Huff support
- [ ] Yul support
- [ ] Rust contracts (Stylus)
- [ ] Template customization
- [ ] Interactive mode
- [ ] Windows installer (macOS already supported)
- [ ] Contract verification helpers
- [ ] Gas optimization templates

## 📝 License

MIT

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 🙏 Acknowledgments

Built with ❤️ for the degen- and crab people communities by Pol Vidal (@pxlvre) • pxlvre.eth
