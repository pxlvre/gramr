# CLI Reference

Complete reference for the Nothung command-line interface.

## Global Commands

### `nothung --help`
Display help information for all commands.

### `nothung --version`  
Display version information.

### `nothung wizard`
Launch the Wotan interactive wizard.

## Main Command: `new`

Create new smart contract resources.

```bash
nothung new <TYPE> <NAME> [OPTIONS]
```

### Resource Types

| Type | Description | Example |
|------|-------------|---------|
| `contract` | Smart contract | `nothung new contract MyToken` |
| `library` | Reusable library | `nothung new library MathUtils` |
| `interface` | Contract interface | `nothung new interface IMyToken` |
| `abstract` | Abstract contract | `nothung new abstract BaseToken` |
| `test` | Test file | `nothung new test TokenTest` |
| `script` | Deployment script | `nothung new script DeployToken` |

## Language Options

### `--solidity`
Generate Solidity code for Foundry projects.

**Supports:**
- All token standards and extensions
- Upgradeable patterns
- Test and script generation
- Complete OpenZeppelin integration

### `--rust-stylus`
Generate Rust code for Arbitrum Stylus projects.

**Supports:**
- Basic ERC20, ERC721, ERC1155 contracts
- Library generation
- OpenZeppelin Stylus integration

**Limitations:**
- No extensions support yet
- No upgradeable patterns
- No test/script generation

## Token Standards

### `--oz-erc20`
Generate OpenZeppelin ERC20 token contract.

```bash
nothung new contract MyToken --solidity --oz-erc20
```

**Available Extensions:**
- `burnable` - Token burning capability
- `pausable` - Emergency pause functionality
- `capped` - Maximum supply limit
- `permit` - Gasless approvals (EIP-2612)
- `votes` - On-chain voting and delegation
- `wrapper` - Wrap other ERC20 tokens
- `flashmint` - Flash loan support
- `temporaryapproval` - Single-transaction approvals
- `bridgeable` - Cross-chain compatibility
- `erc1363` - Payable token standard
- `erc4626` - Tokenized vault standard

### `--oz-erc721`
Generate OpenZeppelin ERC721 NFT contract.

```bash
nothung new contract MyNFT --solidity --oz-erc721
```

**Available Extensions:**
- `enumerable` - Token enumeration and totalSupply
- `burnable` - NFT burning capability
- `pausable` - Emergency pause functionality
- `consecutive` - Efficient batch minting
- `uristorage` - Dynamic metadata URIs
- `votes` - NFT-based voting
- `royalty` - ERC2981 royalty standard
- `wrapper` - Wrap other NFTs

### `--oz-erc1155`
Generate OpenZeppelin ERC1155 multi-token contract.

```bash
nothung new contract GameAssets --solidity --oz-erc1155
```

**Available Extensions:**
- `burnable` - Multi-token burning
- `pausable` - Emergency pause functionality
- `supply` - Track token supplies
- `uristorage` - Per-token URI storage

## Upgrade Patterns

### `--oz-upgradeable`
Use OpenZeppelin upgradeable contract patterns.

```bash
nothung new contract MyToken --solidity --oz-erc20 --oz-upgradeable
```

**Features:**
- Proxy-based upgradeability
- Storage layout protection
- Initializer patterns
- Gap variables for future upgrades

**Compatibility:**
- ✅ Solidity contracts
- ❌ Rust/Stylus (not yet supported)

## Extension Configuration

### `--extensions <LIST>`
Add comma-separated extensions to token contracts.

```bash
# Single extension
nothung new contract MyToken --solidity --oz-erc20 --extensions burnable

# Multiple extensions
nothung new contract MyNFT --solidity --oz-erc721 --extensions enumerable,burnable,royalty

# All compatible extensions
nothung new contract AdvancedToken --solidity --oz-erc20 --extensions burnable,pausable,permit,votes,capped
```

**Extension Compatibility:**

| Extension | ERC20 | ERC721 | ERC1155 |
|-----------|-------|--------|---------|
| `burnable` | ✅ | ✅ | ✅ |
| `pausable` | ✅ | ✅ | ✅ |
| `enumerable` | ❌ | ✅ | ❌ |
| `uristorage` | ❌ | ✅ | ✅ |
| `supply` | ❌ | ❌ | ✅ |
| `votes` | ✅ | ✅ | ❌ |
| `permit` | ✅ | ❌ | ❌ |
| `royalty` | ❌ | ✅ | ❌ |

## Generation Options

### `--with-test`
Generate a corresponding test file.

```bash
nothung new contract MyToken --solidity --oz-erc20 --with-test
```

**Creates:**
- `src/MyToken.sol` - The contract
- `test/MyToken.t.sol` - Foundry test suite

**Test Features:**
- Basic functionality tests
- Extension-specific tests
- Deployment tests
- Integration test examples

### `--with-script`
Generate a deployment script.

```bash
nothung new contract MyToken --solidity --oz-erc20 --with-script
```

**Creates:**
- `src/MyToken.sol` - The contract
- `script/MyToken.s.sol` - Foundry deployment script

**Script Features:**
- Deployment logic
- Constructor parameter configuration
- Post-deployment verification
- Multi-network support

### `--with-section-markers`
Add organized comment sections to contracts.

```bash
nothung new contract MyToken --solidity --oz-erc20 --with-section-markers
```

**Adds Sections:**
```solidity
// ========================================
// IMPORTS
// ========================================

// ========================================
// CONTRACT DECLARATION
// ========================================

// ========================================
// STATE VARIABLES
// ========================================

// ========================================
// CONSTRUCTOR
// ========================================

// ========================================
// PUBLIC FUNCTIONS
// ========================================

// ========================================
// INTERNAL FUNCTIONS
// ========================================
```

## Configuration Options

### `--pragma <VERSION>`
Set Solidity pragma version.

```bash
nothung new contract MyToken --solidity --oz-erc20 --pragma 0.8.25
```

**Default:** `0.8.30`
**Format:** `X.Y.Z` (semantic versioning)

### `--license <LICENSE>`
Set SPDX license identifier.

```bash
nothung new contract MyToken --solidity --oz-erc20 --license MIT
```

**Common Options:**
- `MIT` - MIT License
- `GPL-3.0` - GNU General Public License v3.0
- `Apache-2.0` - Apache License 2.0
- `BSD-3-Clause` - BSD 3-Clause License
- `UNLICENSED` - No license (default)

## Complete Examples

### Basic Contract Generation

```bash
# Minimal ERC20
nothung new contract SimpleToken --solidity --oz-erc20

# Basic NFT
nothung new contract BasicNFT --solidity --oz-erc721

# Multi-token
nothung new contract GameItems --solidity --oz-erc1155
```

### Advanced Contract Generation

```bash
# Feature-rich ERC20 token
nothung new contract AdvancedToken --solidity --oz-erc20 \
  --extensions burnable,pausable,permit,votes \
  --with-test --with-script \
  --pragma 0.8.25 --license MIT

# Complete NFT collection
nothung new contract NFTCollection --solidity --oz-erc721 \
  --extensions enumerable,burnable,royalty,uristorage \
  --with-test --with-script \
  --with-section-markers

# Upgradeable gaming token
nothung new contract GameToken --solidity --oz-erc20 \
  --oz-upgradeable \
  --extensions burnable,pausable \
  --with-test --with-script
```

### Utility Generation

```bash
# Library
nothung new library MathUtils --solidity

# Interface
nothung new interface IGovernance --solidity

# Abstract contract
nothung new abstract BaseVault --solidity

# Standalone test
nothung new test MyContractTest --solidity

# Deployment script
nothung new script DeployAll --solidity
```

### Rust/Stylus Generation

```bash
# Basic Stylus contracts
nothung new contract MyToken --rust-stylus --oz-erc20
nothung new contract MyNFT --rust-stylus --oz-erc721
nothung new library Utils --rust-stylus

# Note: Extensions not yet supported
# This will show helpful error:
nothung new contract MyToken --rust-stylus --oz-erc20 --extensions burnable
```

## Error Handling

### Common Errors

**Invalid Contract Name**
```
Error: Contract name must start with a letter and contain only alphanumeric characters and underscores
```

**Unsupported Extension Combination**
```
Error: Extension 'permit' is not supported for ERC721 contracts
```

**Missing Language Flag**
```
Error: Must specify either --solidity or --rust-stylus
```

**Rust/Stylus Limitations**
```
Error: Extensions are not yet supported for Rust/Stylus contracts
Help: Use basic token generation or switch to --solidity
```

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error |
| `2` | Invalid arguments |
| `3` | Project detection failed |
| `4` | File generation failed |
| `5` | Dependency installation failed |

## Environment Variables

### `NOTHUNG_PROJECT_DIR`
Override project directory detection.

```bash
NOTHUNG_PROJECT_DIR=/path/to/project nothung new contract MyToken --solidity --oz-erc20
```

### `NOTHUNG_LOG_LEVEL`
Set logging level.

```bash
NOTHUNG_LOG_LEVEL=debug nothung new contract MyToken --solidity --oz-erc20
```

**Levels:** `error`, `warn`, `info`, `debug`, `trace`

## Integration with Other Tools

### Foundry Integration

```bash
# Generate and build
nothung new contract MyToken --solidity --oz-erc20 --with-test
forge build

# Generate and test
forge test

# Generate with script and deploy
forge script script/MyToken.s.sol --rpc-url $RPC_URL --broadcast
```

### CI/CD Usage

```yaml
# GitHub Actions example
- name: Generate contracts
  run: |
    nothung new contract ProductionToken --solidity --oz-erc20 \
      --extensions burnable,pausable --with-test
    forge build
    forge test
```

### Docker Usage

```bash
# Using Docker image
docker run --rm -v $(pwd):/workspace ghcr.io/pxlvre/nothung:latest \
  nothung new contract MyToken --solidity --oz-erc20
```

## Tips & Best Practices

1. **Always specify language** - Use `--solidity` or `--rust-stylus`
2. **Use descriptive names** - Follow PascalCase for contracts
3. **Generate tests** - Always use `--with-test` for production contracts
4. **Start simple** - Begin with basic contracts, add extensions later
5. **Check compatibility** - Not all extensions work with all token types
6. **Version consistency** - Use consistent pragma versions across projects
7. **License clarity** - Always specify appropriate license
8. **Build after generation** - Run `forge build` to verify generated code