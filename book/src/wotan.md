# 🧙‍♂️ Wotan Interactive Wizard

Wotan is Gramr's interactive wizard that guides you through smart contract creation step-by-step. Perfect for beginners and faster for experienced developers who want to explore options.

## Starting the Wizard

```bash
# Direct command
wotan

# Or via gramr CLI
gramr wizard
```

## Wizard Flow

### 1. Welcome & Resource Selection

The wizard starts by asking what you want to create:

```
🧙‍♂️ Welcome to Wotan - The Gramr Smart Contract Wizard!
⚔️  Let's forge your smart contract step by step...

? What type of resource would you like to create?
❯ Contract    - Smart contract with optional extensions
  Library     - Reusable utility functions
  Script      - Deployment or interaction script
  Test        - Test file for existing contract
```

### 2. Language Selection

Choose your target platform:

```
? Which language would you like to use?
❯ Solidity     - Full support (Foundry projects)
  Rust/Stylus  - Experimental (Arbitrum Stylus)
```

**Language Features:**

| Feature         | Solidity | Rust/Stylus |
| --------------- | -------- | ----------- |
| Basic Contracts | ✅       | ✅          |
| Extensions      | ✅       | ❌          |
| Upgradeable     | ✅       | ❌          |
| Tests           | ✅       | ❌          |
| Scripts         | ✅       | ❌          |

### 3. Contract Type Selection (for Contracts)

When creating contracts, choose the base type:

```
? What type of contract would you like to create?
❯ Basic contract      - Empty contract template
  ERC20 Token        - Fungible token standard
  ERC721 NFT         - Non-fungible token standard
  ERC1155 Multi-token - Multi-token standard
```

### 4. Extension Configuration

For token contracts, select extensions:

```
? Select extensions for your ERC20 token: (Press <space> to select)
❯ ◯ burnable           - Allow token burning
  ◯ pausable           - Emergency pause functionality
  ◯ mintable           - Additional minting capability
  ◯ capped             - Maximum supply limit
  ◯ permit             - Gasless approvals
  ◯ votes              - On-chain voting support
  ◯ wrapper            - Wrap other ERC20 tokens
```

### 5. Upgradeable Patterns (Solidity Only)

```
? Do you want to make this contract upgradeable?
❯ No  - Standard contract
  Yes - Use OpenZeppelin upgradeable patterns
```

### 6. Generation Options

Configure additional files:

```
? Would you like to generate additional files?
❯ ◯ Test file           - Foundry test suite
  ◯ Deployment script   - Script for contract deployment
  ◯ Section markers     - Organized comment blocks
```

### 7. Contract Configuration

Set contract metadata:

```
? Enter the contract name: MyAwesomeToken

? Solidity version: 0.8.30

? License identifier: MIT
```

### 8. Generation Summary

Review your choices before generation:

```
📋 Generation Summary:
  Resource: Contract
  Language: Solidity
  Name: MyAwesomeToken
  Type: ERC20 with extensions [burnable, pausable]
  Upgradeable: No
  Additional: Test file, Deployment script

? Proceed with generation? (Y/n)
```

## Interactive Features

### Real-time Help

The wizard provides contextual help for each option:

```
? Select extensions for your ERC721 NFT:
❯ ◯ enumerable    - Adds totalSupply() and tokenByIndex() functions
  ◯ burnable      - Allows token holders to burn their tokens
  ◯ pausable      - Adds emergency pause/unpause functionality
  ◯ uristorage    - Enables per-token URI management
  ◯ royalty       - ERC2981 royalty standard implementation
```

### Error Prevention

The wizard prevents invalid combinations:

```
⚠️  Note: Rust/Stylus support is experimental with limited features
   Extensions are not yet supported for Rust/Stylus contracts
```

### Extension Compatibility

Smart extension filtering based on token type:

- **ERC20**: Shows ERC20-specific extensions (permit, votes, flashmint, etc.)
- **ERC721**: Shows NFT extensions (enumerable, royalty, uristorage, etc.)
- **ERC1155**: Shows multi-token extensions (supply, uristorage, etc.)

## Advanced Usage

### Batch Mode (Coming Soon)

For creating multiple contracts:

```bash
# Future feature
wotan --batch contracts.yaml
```

### Configuration Files (Coming Soon)

Save wizard preferences:

```bash
# Future feature
wotan --save-config my-token-config.toml
wotan --load-config my-token-config.toml
```

## Tips for Using Wotan

### For Beginners

1. **Start with Basic** - Try a basic contract first to understand the output
2. **One Extension at a Time** - Add extensions gradually to learn their impact
3. **Read the Generated Code** - Examine what Wotan creates to learn patterns
4. **Use Test Generation** - Always generate tests to see how to interact with your contract

### For Experienced Users

1. **Explore Extensions** - Use Wotan to discover available OpenZeppelin extensions
2. **Quick Prototyping** - Rapidly try different extension combinations
3. **Template Creation** - Use Wotan to create base templates for customization
4. **CI/CD Integration** - Script Wotan responses for automated generation

## Wizard Keyboard Shortcuts

| Key      | Action                         |
| -------- | ------------------------------ |
| `↑/↓`    | Navigate options               |
| `Space`  | Select/deselect (multi-select) |
| `Enter`  | Confirm selection              |
| `Ctrl+C` | Exit wizard                    |
| `?`      | Show help (context-dependent)  |

## Integration with Development Workflow

### 1. Project Setup

```bash
mkdir my-project && cd my-project
forge init --force
```

### 2. Run Wizard

```bash
wotan
# Follow prompts to create your contract
```

### 3. Verify Generation

```bash
forge build
forge test
```

### 4. Customize as Needed

Edit the generated files to add your specific business logic.

## Troubleshooting

### Common Issues

**"Project not detected"**

- Make sure you're in a Foundry project directory
- Run `forge init` if needed

**"Foundry not found"**

- Install Foundry: `curl -L https://foundry.paradigm.xyz | bash && foundryup`
- Ensure `forge` is in your PATH

**"Invalid contract name"**

- Contract names must start with a letter
- Use only letters, numbers, and underscores
- Avoid Solidity reserved keywords

### Getting Help

If you encounter issues:

1. **Check Requirements** - Ensure Rust and Foundry are installed
2. **Update Gramr** - Run the installer again to get the latest version
3. **File Issues** - Report bugs on [GitHub Issues](https://github.com/pxlvre/gramr/issues)

## Comparison: Wizard vs CLI

| Aspect              | Wotan Wizard             | Direct CLI           |
| ------------------- | ------------------------ | -------------------- |
| **Speed**           | Slower (guided)          | Faster (direct)      |
| **Discovery**       | Excellent                | Requires knowledge   |
| **Beginners**       | Perfect                  | Challenging          |
| **Automation**      | Manual                   | Scriptable           |
| **Exploration**     | Great for trying options | Need to know options |
| **Complex Configs** | Step-by-step guidance    | Single command       |

## Next Steps

After using Wotan:

1. **Examine Generated Code** - Understand OpenZeppelin patterns
2. **Try Different Extensions** - Explore various token capabilities
3. **Learn CLI** - Graduate to direct CLI usage for speed
4. **Customize Contracts** - Add your business logic to generated templates
5. **Deploy and Test** - Use generated deployment scripts

The wizard is designed to be your learning companion and rapid prototyping tool. As you become more familiar with Gramr, you might find yourself using the direct CLI more often, but Wotan remains invaluable for exploring new extensions and teaching others.

Happy wizarding! 🧙‍♂️✨
