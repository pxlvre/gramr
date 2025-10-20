# ⚔️ Gramr

> **The legendary sword that forges smart contracts**

Gramr is a blazing-fast toolkit for scaffolding OpenZeppelin-powered smart contracts, tests, and deployment scripts. Built with Rust for maximum performance and reliability.

## What is Gramr?

Gramr streamlines smart contract development by providing:

- **🧙‍♂️ Interactive Wizard** - Zero-configuration contract creation with Wotan
- **⚡ Lightning Fast CLI** - Direct command-line contract generation
- **🏗️ Multiple Languages** - Solidity (Foundry) and Rust (Arbitrum Stylus)
- **🔧 Complete Toolchain** - Tests, deployment scripts, and documentation
- **📦 Easy Installation** - One-line installer with dependency management

## Key Features

### Smart Contract Types

✅ **Contracts** - ERC20, ERC721, ERC1155 tokens with all extensions  
✅ **Libraries** - Reusable utility functions and modules  
✅ **Interfaces** - Contract interface definitions  
✅ **Abstract Contracts** - Abstract base contracts  
✅ **Tests** - Complete Foundry test suites  
✅ **Scripts** - Deployment and interaction scripts

### Language Support

**Solidity (Full Support)**

- Complete OpenZeppelin integration
- All token standards and extensions
- Upgradeable contract patterns
- Test and script generation
- Foundry project integration

**Rust/Stylus (Experimental)**

- Basic ERC20, ERC721, ERC1155 contracts
- Library generation with traits
- Arbitrum Stylus compatibility
- OpenZeppelin Stylus integration

## Quick Example

```bash
# Install Gramr
curl --proto '=https' --tlsv1.2 -sSf https://gramr.pxlvre.eth.limo/install.sh | sh

# Start interactive wizard
wotan

# Or use CLI directly
gramr new contract MyToken --solidity --oz-erc20 --extensions burnable,pausable
```

## Why Gramr?

**🚀 Blazing Fast** - Generate complete contracts in seconds, not hours

**🛡️ Best Practices** - Uses official OpenZeppelin templates and patterns

**🔄 Zero Configuration** - Works out of the box with Foundry and Stylus projects

**🧙‍♂️ Beginner Friendly** - Interactive wizard guides you through every step

**⚡ Developer Focused** - Powerful CLI for experienced developers

**🐳 Container Ready** - Docker support for consistent environments

## Architecture

Gramr is built as a Rust workspace with four core components:

- **`gramr`** - Core library for programmatic use
- **`gramr-cli`** - Command-line interface
- **`wotan`** - Interactive wizard
- **`gramrup`** - Installer and dependency manager

## Community & Support

- **📖 Documentation** - Complete guides and references
- **🐛 Issues** - Bug reports and feature requests on GitHub
- **💬 Discussions** - Community support and questions
- **🔄 Updates** - Follow releases for new features

Ready to start forging? Check out the [Installation Guide](./installation.md) or jump straight to the [Quick Start](./quick-start.md)!
