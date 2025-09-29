# ⚔️ Nothung CLI

> Command-line interface for the Nothung smart contract scaffolding toolkit

The Nothung CLI provides a powerful command-line interface for rapidly generating smart contracts, libraries, tests, and deployment scripts. Supports both Solidity (Foundry) and Rust/Stylus (Arbitrum) projects with full OpenZeppelin integration.

## 🧙‍♂️ Prefer Interactive? Try Wotan!

For a guided experience, use the interactive wizard:

```bash
nothung wizard  # Launch interactive wizard
# or
wotan          # Direct wizard command
```

## Installation

### From Source (Recommended)

```bash
# Install from the workspace root
cargo install --path cli

# Or install from GitHub
cargo install --git https://github.com/pxlvre/nothung nothung-cli
```

### Using nothungup

```bash
nothungup install
```

## Usage

### Basic Commands

```bash
# Create a basic contract
nothung new contract MyContract --solidity

# Create an ERC20 token
nothung new contract MyToken --solidity --oz-erc20

# Create an ERC721 with extensions
nothung new contract MyNFT --solidity --oz-erc721 --extensions enumerable,burnable

# Create with test and script
nothung new contract MyToken --solidity --oz-erc20 --with-test --with-script

# Generate test or script separately
nothung new test MyTest --solidity
nothung new script DeployScript --solidity
```

### Command Structure

```bash
nothung new <TYPE> <NAME> [OPTIONS]
```

**Arguments:**
- `TYPE`: Resource type (`contract`, `library`, `script`, `test`)
- `NAME`: Name of the resource to generate

**Global Options:**
- `--solidity` - Generate Solidity code (required)
- `--pragma <VERSION>` - Solidity version (default: 0.8.30)
- `--license <LICENSE>` - SPDX license (default: UNLICENSED)

**Contract Options:**
- `--oz-erc20` - Inherit from OpenZeppelin ERC20
- `--oz-erc721` - Inherit from OpenZeppelin ERC721  
- `--oz-erc1155` - Inherit from OpenZeppelin ERC1155
- `--oz-upgradeable` - Use upgradeable variants
- `--extensions <LIST>` - Comma-separated extensions
- `--with-test` - Generate test file
- `--with-script` - Generate deployment script

### Available Extensions

**ERC20:** `permit`, `burnable`, `capped`, `pausable`, `votes`, `wrapper`, `flashmint`, `temporaryapproval`, `bridgeable`, `erc1363`, `erc4626`

**ERC721:** `pausable`, `burnable`, `consecutive`, `uristorage`, `votes`, `royalty`, `wrapper`, `enumerable`

**ERC1155:** `pausable`, `burnable`, `supply`, `uristorage`

**Cross-compatible:** `burnable` and `pausable` work with any token type

## Examples

```bash
# Complex ERC20 with multiple extensions
nothung new contract GovernanceToken --solidity \
  --oz-erc20 \
  --extensions permit,votes,pausable \
  --with-test --with-script

# NFT collection with enumeration and royalties
nothung new contract ArtCollection --solidity \
  --oz-erc721 \
  --extensions enumerable,royalty,uristorage \
  --license MIT

# Multi-token with supply tracking
nothung new contract GameAssets --solidity \
  --oz-erc1155 \
  --extensions supply,pausable \
  --pragma 0.8.25
```

## Requirements

- Must be run within a Foundry project directory
- Foundry must be installed for OpenZeppelin dependency management

## See Also

- [nothung library](../lib/) - Core functionality
- [nothungup installer](../nothungup/) - System installer
- [Main README](../README.md) - Full project documentation