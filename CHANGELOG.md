# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial release of Gramr CLI tool
- Support for Solidity smart contract generation
- Support for Rust/Stylus contract generation
- OpenZeppelin templates for ERC20, ERC721, and ERC1155
- Upgradeable contract patterns
- Token extensions (burnable, pausable, votes, etc.)
- Wotan interactive wizard for guided contract creation
- gramrup installer for easy setup
- Interface and abstract contract generation
- Test and deployment script generation
- Automatic dependency management (OpenZeppelin installation)
- Multi-platform support (Linux, macOS)

### Features

- **Smart Contract Templates**

  - Basic contracts
  - ERC20, ERC721, ERC1155 tokens
  - Upgradeable patterns
  - Multiple inheritance support
  - Extensive extension library

- **Developer Experience**

  - Interactive wizard (Wotan)
  - Direct CLI commands
  - Automatic Foundry project detection
  - Smart dependency installation

- **Platform Support**
  - macOS (Intel & Apple Silicon)
  - Linux (x86_64 & ARM64)
  - Foundry integration
  - Future: Hardhat support planned

## [0.0.1]

First public release.

### Installation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://gramr.pxlvre.eth.limo/install.sh | sh
```
