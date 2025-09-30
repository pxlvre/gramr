# Gramr Library

⚔️ Core Rust library for scaffolding smart contracts with programmatic API.

## About

The `gramr` library provides the core functionality for generating Solidity contracts, tests, and deployment scripts. It can be used programmatically in Rust applications or as a foundation for building custom tooling.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
gramr = { git = "https://github.com/pxlvre/gramr" }
```

Or for local development:

```toml
[dependencies]
gramr = { path = "../lib" }
```

## API Overview

### Core Types

- **`ContractBuilder`** - Builder pattern for creating contracts
- **`ContractType`** - Enum defining contract types (Basic, ERC20, ERC721, etc.)
- **`TokenExtension`** - Enum for OpenZeppelin extensions
- **`FoundryProject`** - Represents a Foundry project structure
- **`SolidityTemplate`** - Template generator for Solidity code

### Generators

- **`ContractGenerator`** - Generates contract files
- **`TestGenerator`** - Generates test files
- **`ScriptGenerator`** - Generates deployment scripts

## Quick Start

### Generate Contract Source

```rust
use gramr::{ContractBuilder, ContractType};

let source = ContractBuilder::new("MyToken")
    .contract_type(ContractType::ERC20)
    .pragma("0.8.30")
    .license("MIT")
    .build();

println!("{}", source);
```

### Write to Foundry Project

```rust
use gramr::{ContractBuilder, ContractType, FoundryProject};

fn main() -> gramr::Result<()> {
    let project = FoundryProject::detect()?;

    ContractBuilder::new("MyNFT")
        .contract_type(ContractType::ERC721)
        .generate(project)?;

    Ok(())
}
```

### Complex Contract with Extensions

```rust
use gramr::{ContractBuilder, ContractType, TokenExtension};

let contract_type = ContractType::MultiInheritance {
    base_type: Box::new(ContractType::ERC721),
    extensions: vec![
        TokenExtension::ERC721Enumerable,
        TokenExtension::ERC721Burnable,
        TokenExtension::ERC721Royalty,
    ],
};

let source = ContractBuilder::new("ComplexNFT")
    .contract_type(contract_type)
    .build();
```

### Using Generators Directly

```rust
use gramr::{ContractGenerator, ContractType, FoundryProject};

fn main() -> gramr::Result<()> {
    let project = FoundryProject::detect()?;

    let generator = ContractGenerator::new(
        project,
        "MyToken".to_string(),
        ContractType::ERC20,
        true,  // with_test
        true,  // with_script
        "0.8.30".to_string(),
        "MIT".to_string(),
    );

    generator.generate()?;
    Ok(())
}
```

### Parse Extensions from Strings

```rust
use gramr::parse_extensions;

let extensions = parse_extensions(&[
    "burnable".to_string(),
    "pausable".to_string(),
    "votes".to_string(),
])?;
```

## Contract Types

### Basic Types

- `ContractType::Basic` - Empty contract
- `ContractType::ERC20` - Standard ERC20 token
- `ContractType::ERC721` - Standard ERC721 NFT
- `ContractType::ERC1155` - Standard ERC1155 multi-token

### Upgradeable Types

- `ContractType::ERC20Upgradeable`
- `ContractType::ERC721Upgradeable`
- `ContractType::ERC1155Upgradeable`

### Multi-Inheritance

```rust
ContractType::MultiInheritance {
    base_type: Box<ContractType>,
    extensions: Vec<TokenExtension>,
}
```

## Token Extensions

### ERC20 Extensions (11 available)

`ERC20Permit`, `ERC20Burnable`, `ERC20Capped`, `ERC20Pausable`, `ERC20Votes`, `ERC20Wrapper`, `ERC20FlashMint`, `ERC20TemporaryApproval`, `ERC20Bridgeable`, `ERC1363`, `ERC4626`

### ERC721 Extensions (8 available)

`ERC721Pausable`, `ERC721Burnable`, `ERC721Consecutive`, `ERC721URIStorage`, `ERC721Votes`, `ERC721Royalty`, `ERC721Wrapper`, `ERC721Enumerable`

### ERC1155 Extensions (4 available)

`ERC1155Pausable`, `ERC1155Burnable`, `ERC1155Supply`, `ERC1155URIStorage`

## Error Handling

The library uses `anyhow::Result` for error handling:

```rust
use gramr::Result;

fn my_function() -> Result<()> {
    // Library functions return Result<T>
    let project = FoundryProject::detect()?;
    // ...
    Ok(())
}
```

## Features

- **`default`** - Standard functionality
- **`macros`** - Future: Procedural macros (planned)

## Dependencies

- `anyhow` - Error handling
- `colored` - Terminal colors
- `fs_extra` - File operations
- `serde` - Serialization
- `thiserror` - Error types
- `which` - Executable detection

## See Also

- [CLI tool](../cli/) - Command-line interface
- [gramrup installer](../gramrup/) - System installer
- [Main README](../README.md) - Full project documentation
