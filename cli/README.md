# ⚔️ Gramr CLI

> Command-line interface for the Gramr smart contract scaffolding toolkit

The Gramr CLI provides a powerful command-line interface for rapidly generating smart contracts, libraries, tests, and deployment scripts. Supports both Solidity (Foundry) and Rust/Stylus (Arbitrum) projects with full OpenZeppelin integration.

## 🧙‍♂️ Prefer Interactive? Try Wotan!

For a guided experience, use the interactive wizard:

```bash
gramr wizard  # Launch interactive wizard
# or
wotan          # Direct wizard command
```

## Installation

### From Source (Recommended)

```bash
# Install from the workspace root
cargo install --path cli

# Or install from GitHub
cargo install --git https://github.com/pxlvre/gramr gramr-cli
```

### Using gramrup

```bash
gramrup install
```

## Usage

### Basic Commands

```bash
# Create a basic contract
gramr new contract MyContract --solidity

# Create an ERC20 token
gramr new contract MyToken --solidity --oz-erc20

# Create an ERC721 with extensions
gramr new contract MyNFT --solidity --oz-erc721 --extensions enumerable,burnable

# Create with test and script
gramr new contract MyToken --solidity --oz-erc20 --with-test --with-script

# Create with organized section markers
gramr new contract MyToken --solidity --oz-erc20 --with-section-markers

# Generate different resource types
gramr new library MathUtils --solidity
gramr new interface IMyToken --solidity
gramr new abstract BaseToken --solidity
gramr new test MyTest --solidity
gramr new script DeployScript --solidity

# Generate config files (placeholder)
gramr new config slither --solidity
```

### Command Structure

```bash
gramr new <TYPE> <NAME> [OPTIONS]
```

**Arguments:**

- `TYPE`: Resource type (`contract`, `library`, `interface`, `abstract`, `script`, `test`, `config`)
- `NAME`: Name of the resource to generate

**Global Options:**

- `--solidity` - Generate Solidity code (required)
- `--pragma <VERSION>` - Solidity version (default: 0.8.30)
- `--license <LICENSE>` - SPDX license (default: UNLICENSED)

**Contract Options:**

- `--oz-erc20` - Inherit from OpenZeppelin ERC20
- `--oz-erc721` - Inherit from OpenZeppelin ERC721
- `--oz-erc1155` - Inherit from OpenZeppelin ERC1155
- `--upgradeable` - Use upgradeable variants
- `--extensions <LIST>` - Comma-separated extensions
- `--with-test` - Generate test file
- `--with-script` - Generate deployment script
- `--with-section-markers` - Include organized comment blocks for code sections

### Available Extensions

**ERC20:** `permit`, `burnable`, `capped`, `pausable`, `votes`, `wrapper`, `flashmint`, `temporaryapproval`, `bridgeable`, `erc1363`, `erc4626`

**ERC721:** `pausable`, `burnable`, `consecutive`, `uristorage`, `votes`, `royalty`, `wrapper`, `enumerable`

**ERC1155:** `pausable`, `burnable`, `supply`, `uristorage`

**Cross-compatible:** `burnable` and `pausable` work with any token type

## Examples

```bash
# Complex ERC20 with multiple extensions and section markers
gramr new contract GovernanceToken --solidity \
  --oz-erc20 \
  --extensions permit,votes,pausable \
  --with-test --with-script --with-section-markers

# NFT collection with enumeration and royalties
gramr new contract ArtCollection --solidity \
  --oz-erc721 \
  --extensions enumerable,royalty,uristorage \
  --license MIT

# Multi-token with supply tracking
gramr new contract GameAssets --solidity \
  --oz-erc1155 \
  --extensions supply,pausable \
  --pragma 0.8.25

# Generate supporting files
gramr new interface IArtCollection --solidity --license MIT
gramr new abstract BaseCollection --solidity --with-section-markers
gramr new library CollectionUtils --solidity
```

## Requirements

- Must be run within a Foundry project directory
- Foundry must be installed for OpenZeppelin dependency management

## See Also

- [gramr library](../lib/) - Core functionality
- [gramrup installer](../gramrup/) - System installer
- [Main README](../README.md) - Full project documentation
