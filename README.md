# ⚔️ Nothung

> The legendary sword that forges smart contracts

A toolkit for scaffolding OpenZeppelin-powered smart contracts, tests, and deployment scripts. Supports Solidity (Foundry) and Rust (Arbitrum Stylus) projects.

🦀 Built with Rust 🦀

## 🏗️ Architecture

Nothung is organized as a Rust workspace with four distinct components:

### 📦 Packages

- **`nothung`** (library) - Core functionality for contract generation that can be used programmatically
- **`nothung-cli`** - Command-line interface built on top of the library
- **`wotan`** - 🧙‍♂️ Interactive wizard for guided smart contract creation
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

# Install the interactive wizard
cargo install --git https://github.com/pxlvre/nothung wotan
```

### Build from Source

```bash
git clone https://github.com/pxlvre/nothung
cd nothung
cargo install --path cli
```

## 📖 Usage

### 🧙‍♂️ Interactive Wizard (Recommended)

The easiest way to get started is with the interactive wizard:

```bash
# Start the wizard (two ways)
wotan           # Direct wizard command
nothung wizard  # Via nothung CLI
```

The wizard guides you through:
- **Resource selection** - Contract, library, script, or test
- **Language choice** - Solidity (full support) or Rust/Stylus (experimental)  
- **Token standards** - ERC20, ERC721, ERC1155 with extensions
- **Generation options** - Tests, scripts, upgradeable contracts
- **Configuration** - Pragma versions, licenses, and more

Perfect for beginners and faster for experienced developers!

### 🔧 Direct CLI Usage

For direct command-line usage without the wizard:

#### Solidity Contracts (Full Support)

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

# Generate with section markers (organized comment blocks)
nothung new contract MyToken --solidity --oz-erc20 --with-section-markers

# Generate upgradeable contracts
nothung new contract MyToken --solidity --oz-erc20 --upgradeable

# Generate libraries
nothung new library MathUtils --solidity

# Generate interfaces
nothung new interface IMyToken --solidity

# Generate abstract contracts
nothung new abstract BaseToken --solidity

# Generate config files (placeholder for future)
nothung new config slither --solidity
```

#### Rust/Stylus Contracts (Experimental)

```bash
# Generate basic Rust contracts for Arbitrum Stylus
nothung new contract MyToken --rust-stylus --oz-erc20
nothung new contract MyNFT --rust-stylus --oz-erc721
nothung new contract MyMultiToken --rust-stylus --oz-erc1155

# Generate libraries  
nothung new library DataUtils --rust-stylus

# Note: Limited support - see Rust/Stylus section below
```

#### Command Structure

```bash
nothung new <TYPE> <NAME> [OPTIONS]
```

**Arguments:**

- `TYPE`: `contract`, `library`, `interface`, `abstract`, `script`, `test`, or `config`
- `NAME`: Name of the resource

**Language Options:**

- `--solidity` - Generate Solidity code for Foundry projects
- `--rust-stylus` - Generate Rust code for Arbitrum Stylus projects

**Token Options:**

- `--oz-erc20` - Inherit from OpenZeppelin ERC20
- `--oz-erc721` - Inherit from OpenZeppelin ERC721  
- `--oz-erc1155` - Inherit from OpenZeppelin ERC1155
- `--upgradeable` - Use upgradeable contracts (Solidity only)
- `--extensions <LIST>` - Comma-separated token extensions

**Generation Options:**

- `--with-test` - Generate test file (Solidity only)
- `--with-script` - Generate deployment script (Solidity only)
- `--with-section-markers` - Include organized comment blocks for code sections
- `--pragma <VERSION>` - Solidity version (default: 0.8.30, Solidity only)
- `--license <LICENSE>` - SPDX identifier (default: UNLICENSED, Solidity only)

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

### Languages Supported

#### Solidity (Full Support)
- ✅ **Contracts** - ERC20, ERC721, ERC1155 tokens
- ✅ **Libraries** - Reusable utility functions
- ✅ **Interfaces** - Contract interface definitions
- ✅ **Abstract Contracts** - Abstract base contracts
- ✅ **Upgradeable** variants of all token standards
- ✅ **Test generation** - Foundry test files
- ✅ **Script generation** - Deployment scripts
- ✅ **Section markers** - Organized comment blocks for code structure
- ✅ **Config scaffolding** - Quick config file generation (coming soon)
- ✅ **All extensions** - Complete OpenZeppelin extension support
- ✅ **Interactive wizard** - Guided creation process

#### Rust/Stylus (Experimental)
- ✅ **Contracts** - Basic ERC20, ERC721, ERC1155 tokens
- ✅ **Libraries** - Trait-based reusable patterns
- ❌ **Interfaces** - Use traits instead (guided by error messages)
- ❌ **Abstract Contracts** - Use traits with default implementations instead
- ❌ **Upgradeable contracts** - Not yet supported by OpenZeppelin Stylus
- ❌ **Test generation** - Use `cargo test` instead
- ❌ **Script generation** - Use `stylus deploy` instead
- ❌ **Extensions** - Limited extension support (coming soon)
- ❌ **Config generation** - Not applicable for Rust/Stylus projects
- ✅ **Interactive wizard** - Guided creation with clear limitations

### OpenZeppelin Extensions (Solidity)

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

## 🦀 Rust/Stylus Support (Experimental)

Nothung now supports generating Rust contracts for [Arbitrum Stylus](https://arbitrum.io/stylus), a next-generation smart contract environment that runs WebAssembly alongside the EVM.

### What Works

```bash
# Basic token contracts
nothung new contract MyToken --rust-stylus --oz-erc20
nothung new contract MyNFT --rust-stylus --oz-erc721
nothung new contract MyMultiToken --rust-stylus --oz-erc1155
```

Generated contracts use [OpenZeppelin Contracts for Stylus](https://github.com/OpenZeppelin/rust-contracts-stylus) and include:

- **Standard token functionality** - All basic ERC20/721/1155 methods
- **Automatic dependency management** - Installs `openzeppelin-stylus` crate
- **Idiomatic Rust code** - Follows Rust best practices and Stylus patterns
- **WebAssembly compilation** - Ready for Stylus deployment

### Current Limitations

❌ **Upgradeable contracts** - OpenZeppelin Stylus doesn't support upgradeable patterns yet
❌ **Extensions** - Limited to basic token functionality for now  
❌ **Test generation** - Use standard `cargo test` workflow instead
❌ **Script generation** - Use `stylus deploy` for deployment

### Error Messages

Nothung provides clear error messages for unsupported combinations:

```bash
# This will show a helpful error
nothung new contract MyToken --rust-stylus --oz-erc20 --upgradeable
# Error: "Upgradeable contracts are not yet supported for Rust/Stylus..."

nothung new contract MyToken --rust-stylus --oz-erc20 --with-test  
# Error: "Test generation is not supported for Rust/Stylus projects. Use 'cargo test' instead."
```

### Development Workflow

1. **Generate contract**: `nothung new contract MyToken --rust-stylus --oz-erc20`
2. **Build**: `cargo build --release --target wasm32-unknown-unknown`
3. **Test**: `cargo test`
4. **Deploy**: `stylus deploy --private-key $PRIVATE_KEY`

### Future Roadmap

- 🔮 **Extension support** - Token extensions like burnable, pausable
- 🔮 **Upgradeable patterns** - When OpenZeppelin Stylus adds support  
- 🔮 **Test generation** - Stylus-specific test templates
- 🔮 **Better Cargo integration** - Automatic toolchain setup

## 🛠️ System Requirements

### Required

- **Rust** 1.70+ - Programming language and toolchain
- **Cargo** - Rust package manager

### For Solidity Projects (Recommended)

- **Foundry** - Smart contract development framework
  - `forge` - Compiler and test runner
  - `anvil` - Local blockchain
  - `cast` - Contract interaction CLI

### For Rust/Stylus Projects (Experimental)

- **Rust nightly** - Required for OpenZeppelin Stylus contracts
- **cargo-stylus** - Stylus development tools (optional)
- **stylus** CLI - For deployment to Arbitrum Stylus

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

### v0.1.0 (Current)

- ✅ **Interactive wizard** - 🧙‍♂️ Wotan guided contract creation
- ✅ **Solidity support** - Full contract, library, script, test generation
- ✅ **Library generation** - Reusable utility contracts and modules
- ✅ **Full OpenZeppelin integration** - All token standards and extensions
- ✅ **Multi-inheritance support** - Complex token combinations
- ✅ **Unix installer** - Dependency checking and management

### Experimental Features

- ✅ **Rust/Stylus contracts** - Basic token generation for Arbitrum Stylus
  - ⚠️ **Limited support**: Contract and library generation, no upgradeable contracts yet
  - 🔮 **Coming soon**: Extensions, upgradeable patterns, better Cargo integration

### Future Versions

- [ ] **Full Rust/Stylus support** - Extensions, upgradeable contracts, test generation
- [ ] **Cairo support** - StarkNet with OpenZeppelin Contracts for Cairo
- [ ] **Template customization** - Custom contract templates
- [ ] **Interactive mode** - Step-by-step contract creation
- [ ] **Windows installer** - Native Windows support
- [ ] **Contract verification helpers** - Automated verification workflows
- [ ] **Gas optimization templates** - Performance-optimized contract variants

## 📝 License

MIT

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 🙏 Acknowledgments

Built with ❤️ for the degen- and crab people communities by Pol Vidal (@pxlvre) • pxlvre.eth
