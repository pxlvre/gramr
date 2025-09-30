# Quick Start

Get up and running with gramr in under 5 minutes!

## 📦 Installation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://getgramr.pxlvre.dev | sh
```

## 🎯 Your First Contract

### Option 1: Interactive Wizard (Recommended for Beginners)

Start the interactive wizard:

```bash
wotan
```

The wizard will guide you through:

1. **Resource Type** - Contract, library, script, or test
2. **Language** - Solidity or Rust/Stylus
3. **Token Standard** - ERC20, ERC721, ERC1155, or basic
4. **Extensions** - Burnable, pausable, votes, etc.
5. **Options** - Tests, scripts, upgradeable patterns

### Option 2: Direct CLI (For Experienced Users)

```bash
# Create an ERC20 token
gramr new contract MyToken --solidity --oz-erc20

# Create an NFT with extensions
gramr new contract MyNFT --solidity --oz-erc721 --extensions enumerable,burnable

# Create a multi-token with tests and scripts
gramr new contract GameAssets --solidity --oz-erc1155 --extensions supply,pausable --with-test --with-script
```

## 🏗️ Setting Up a Project

### 1. Create a New Foundry Project

```bash
# Create project directory
mkdir my-token-project && cd my-token-project

# Initialize Foundry project
forge init --force

# Verify setup
forge build
```

### 2. Generate Your First Contract

```bash
# Generate an ERC20 token with extensions
gramr new contract MyAwesomeToken --solidity --oz-erc20 --extensions burnable,pausable --with-test --with-script

# This creates:
# - src/MyAwesomeToken.sol (contract)
# - test/MyAwesomeToken.t.sol (tests)
# - script/MyAwesomeToken.s.sol (deployment script)
```

### 3. Build and Test

```bash
# Compile contracts
forge build

# Run tests
forge test

# Check test coverage
forge coverage
```

## 📋 Common Patterns

### ERC20 Token Examples

```bash
# Basic ERC20
gramr new contract BasicToken --solidity --oz-erc20

# Token with common extensions
gramr new contract UtilityToken --solidity --oz-erc20 --extensions burnable,pausable

# Governance token
gramr new contract GovernanceToken --solidity --oz-erc20 --extensions votes,permit

# DeFi token with all features
gramr new contract DeFiToken --solidity --oz-erc20 --extensions burnable,pausable,votes,permit,capped
```

### ERC721 NFT Examples

```bash
# Basic NFT
gramr new contract BasicNFT --solidity --oz-erc721

# NFT with metadata and enumeration
gramr new contract CollectibleNFT --solidity --oz-erc721 --extensions enumerable,uristorage

# Gaming NFT with all features
gramr new contract GameNFT --solidity --oz-erc721 --extensions enumerable,burnable,pausable,royalty
```

### ERC1155 Multi-Token Examples

```bash
# Basic multi-token
gramr new contract GameAssets --solidity --oz-erc1155

# Multi-token with supply tracking
gramr new contract TradingCards --solidity --oz-erc1155 --extensions supply,uristorage

# Advanced multi-token
gramr new contract AdvancedAssets --solidity --oz-erc1155 --extensions supply,burnable,pausable
```

### Upgradeable Contracts

```bash
# Upgradeable ERC20
gramr new contract UpgradeableToken --solidity --oz-erc20 --oz-upgradeable

# Upgradeable NFT
gramr new contract UpgradeableNFT --solidity --oz-erc721 --oz-upgradeable --extensions enumerable
```

## 🛠️ Development Workflow

### 1. Plan Your Contract

```bash
# Use the wizard to explore options
wotan
```

### 2. Generate Code

```bash
# Generate with tests and deployment scripts
gramr new contract MyContract --solidity --oz-erc20 --extensions burnable --with-test --with-script
```

### 3. Develop and Test

```bash
# Continuous compilation during development
forge build --watch

# Run tests with gas reports
forge test --gas-report

# Run specific tests
forge test --match-test testMint
```

### 4. Deploy

```bash
# Deploy to local network
anvil  # In another terminal

# Run deployment script
forge script script/MyContract.s.sol --rpc-url http://localhost:8545 --broadcast
```

## 🔧 Customization

### Custom Pragma and License

```bash
gramr new contract MyContract --solidity --oz-erc20 --pragma 0.8.25 --license MIT
```

### Section Markers for Organization

```bash
gramr new contract MyContract --solidity --oz-erc20 --with-section-markers
```

This adds organized comment sections to your contract:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

/// @title MyContract
/// @notice A brief description of the contract
contract MyContract is ERC20 {
    // ========================================
    // CONSTRUCTOR
    // ========================================

    constructor() ERC20("MyContract", "MC") {
        // Constructor implementation
    }

    // ========================================
    // PUBLIC FUNCTIONS
    // ========================================

    // Your public functions here
}
```

## 🦀 Rust/Stylus Quick Start

For Arbitrum Stylus development:

```bash
# Setup Rust nightly (required)
rustup install nightly
rustup default nightly

# Generate Rust contract
gramr new contract MyToken --rust-stylus --oz-erc20

# Build for Stylus
cargo build --release --target wasm32-unknown-unknown

# Test with standard Rust tools
cargo test
```

## 🐳 Docker Development

```bash
# Clone and start development environment
git clone https://github.com/pxlvre/gramr
cd gramr
docker-compose up -d gramr-dev

# Use gramr inside container
docker-compose exec gramr-dev bash
gramr new contract MyToken --solidity --oz-erc20
```

## 🚀 Production Usage

### CI/CD Integration

```yaml
# .github/workflows/contracts.yml
- name: Generate contracts
  run: |
    gramr new contract ProductionToken --solidity --oz-erc20 --extensions burnable,pausable
    forge build
    forge test
```

### Docker Deployment

```bash
# Use production image
docker run --rm -v $(pwd):/workspace ghcr.io/pxlvre/gramr:latest \
  gramr new contract MyToken --solidity --oz-erc20
```

## 🔍 What's Next?

Now that you have the basics:

1. **Explore the Wizard** - Run `wotan` to see all available options
2. **Learn Extensions** - Check out the [Extensions Guide](./extensions.md)
3. **Read CLI Reference** - Master all commands in [CLI Reference](./cli-reference.md)
4. **Integrate Libraries** - Use gramr programmatically with the [Library API](./library.md)
5. **Deploy Contracts** - Learn deployment patterns in [Project Integration](./integration.md)

## 💡 Tips & Tricks

- **Start with the wizard** (`wotan`) to explore all options
- **Use `--with-test --with-script`** for complete project scaffolding
- **Combine extensions** like `burnable,pausable,votes` for feature-rich tokens
- **Check generated code** to learn OpenZeppelin patterns
- **Run `forge build`** after generation to ensure everything compiles
- **Use Docker** for consistent development environments

Happy forging! ⚔️
