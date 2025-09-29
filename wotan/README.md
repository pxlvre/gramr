# >�

B Wotan - Interactive Smart Contract Wizard

> The wise wizard that guides you through smart contract creation

Wotan is the interactive wizard companion to Nothung that makes creating smart contracts as easy as answering a few questions. No need to memorize CLI flags or options - just follow the guided prompts!

## ( Features

- **<� Interactive Prompts** - Step-by-step guidance with smart defaults
- **=
  Smart Validation** - Input validation with helpful error messages
- **<
  Multi-Language** - Supports both Solidity and Rust/Stylus
- **=� Rich Options** - Token standards, extensions, upgradeable contracts
- **� Fast & Intuitive** - Faster than remembering complex CLI commands
- **<� Colorized Output** - Beautiful, easy-to-read interface

## =� Installation

### Via NothungUp (Recommended)

```bash
# Install everything including Wotan
nothungup install
```

### Manual Installation

```bash
# Install Wotan directly
cargo install --git https://github.com/pxlvre/nothung wotan

# Or build from source
git clone https://github.com/pxlvre/nothung
cd nothung
cargo install --path wotan
```

## =� Usage

### Start the Wizard

```bash
# Direct command
wotan

# Or via nothung CLI
nothung wizard
```

### Non-Interactive Mode

```bash
# Show help without starting wizard
wotan --non-interactive
```

## <� Wizard Flow

1. **Resource Selection** - Contract, library, script, or test
2. **Basic Configuration** - Name and language choice
3. **Contract Options** - Token standards, upgradeable, extensions
4. **Generation Options** - Tests, scripts, Solidity settings
5. **Summary & Confirmation** - Review and generate

## < Smart Adaptations

### Solidity Projects

-  Full feature support
-  All token standards and extensions
-  Upgradeable contract options
-  Test and script generation

### Rust/Stylus Projects

-  Basic contract and library generation
- � Clear warnings about unsupported features
- =� Helpful alternative suggestions

## =� Integration

Wotan seamlessly integrates with the Nothung ecosystem:

- Uses the same core library for generation
- Supports all Nothung features through interactive prompts
- Generates the same high-quality contracts as CLI

## =� License

MIT OR Apache-2.0

## = Related

- **[Nothung](../README.md)** - The main project
- **[Nothung CLI](../cli/README.md)** - Direct command-line interface
- **[NothungUp](../nothungup/README.md)** - The installer utility
