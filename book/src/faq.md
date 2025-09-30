# Frequently Asked Questions

Common questions and answers about Nothung.

## General Questions

### What is Nothung?

Nothung is a fast, Rust-based toolkit for generating OpenZeppelin-powered smart contracts. It supports Solidity (Foundry) and experimental Rust (Arbitrum Stylus) development.

### Why is it called "Nothung"?

Nothung is the legendary sword from Germanic mythology, famously featured in Wagner's Ring cycle. Just as the sword was reforged to be legendary, Nothung helps you forge legendary smart contracts! ⚔️

### Is Nothung free to use?

Yes! Nothung is completely free and open-source under MIT/Apache-2.0 licenses.

## Installation & Setup

### How do I install Nothung?

The fastest way is our one-line installer:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://getnothung.pxlvre.dev | sh
```

See the [Installation Guide](./installation.md) for all options.

### Do I need to install Foundry separately?

No, the installer can automatically install Foundry if needed. However, you can install it manually:
```bash
curl -L https://foundry.paradigm.xyz | bash && foundryup
```

### Can I use Nothung with Hardhat instead of Foundry?

Nothung generates Foundry-compatible contracts, but they work in Hardhat too. You'll need to:
1. Copy generated contracts to your Hardhat project
2. Manually install OpenZeppelin: `npm install @openzeppelin/contracts`
3. Adapt deployment scripts and tests

### Does Nothung work on Windows?

Not natively yet. Windows support is planned for v0.2.0. Current options:
- Use WSL2 (recommended)
- Use Docker containers
- Use GitHub Codespaces

## Usage Questions

### Should I use the wizard or CLI?

**Use the wizard (`wotan`) if:**
- You're new to Nothung or OpenZeppelin
- You want to explore available options
- You're prototyping or learning

**Use the CLI directly if:**
- You know exactly what you want
- You're scripting/automating
- You want maximum speed

### What's the difference between extensions?

Extensions add specific functionality:
- **`burnable`** - Users can destroy tokens
- **`pausable`** - Admin can stop transfers
- **`permit`** - Gasless approvals (ERC20 only)
- **`votes`** - Governance functionality
- **`enumerable`** - Token enumeration (NFTs)

See the [Extensions Guide](./extensions.md) for complete details.

### Can I combine multiple extensions?

Yes! Many extensions work together:
```bash
nothung new contract MyToken --solidity --oz-erc20 --extensions burnable,pausable,permit,votes
```

Nothung automatically handles compatibility and inheritance order.

### Can I modify generated contracts?

Absolutely! Generated contracts are meant to be starting points. Add your custom business logic, modify functions, and customize as needed.

### How do I add admin functions?

Use OpenZeppelin's `Ownable` or `AccessControl`. Example:
```solidity
import "@openzeppelin/contracts/access/Ownable.sol";

contract MyToken is ERC20, Ownable {
    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}
```

## Technical Questions

### Why are some features experimental for Rust/Stylus?

Arbitrum Stylus is a new platform, and the OpenZeppelin Stylus contracts are still developing. Features like extensions and upgradeable patterns aren't available yet.

### Can I use upgradeable contracts?

Yes, for Solidity contracts:
```bash
nothung new contract MyToken --solidity --oz-erc20 --oz-upgradeable
```

This uses OpenZeppelin's proxy patterns for safe upgradeability.

### How do I deploy generated contracts?

**With Foundry:**
```bash
# Generate with deployment script
nothung new contract MyToken --solidity --oz-erc20 --with-script

# Deploy
forge script script/MyToken.s.sol --rpc-url $RPC_URL --broadcast
```

**With Hardhat:** Adapt the generated Foundry script to Hardhat deployment patterns.

### Why do I get compilation errors?

Common causes:
1. **Foundry not installed** - Install with `foundryup`
2. **Wrong directory** - Run from Foundry project root
3. **Missing dependencies** - Run `forge install` or regenerate with Nothung
4. **Version mismatch** - Update Foundry with `foundryup`

### How do I test generated contracts?

Generate tests with the `--with-test` flag:
```bash
nothung new contract MyToken --solidity --oz-erc20 --with-test
forge test
```

### Can I generate libraries?

Yes! Nothung supports utility libraries:
```bash
nothung new library MathUtils --solidity
nothung new library DataStructures --rust-stylus
```

### How do I generate interfaces?

For defining contract interfaces:
```bash
nothung new interface IMyToken --solidity
```

### What about abstract contracts?

For base contracts that others inherit from:
```bash
nothung new abstract BaseToken --solidity
```

## Troubleshooting

### "Command not found" after installation

Check your PATH includes the installation directory:
```bash
echo 'export PATH="$HOME/.nothung/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### "Project not detected" error

Make sure you're in a Foundry project:
```bash
forge init --force
```

### "forge: command not found"

Install Foundry:
```bash
curl -L https://foundry.paradigm.xyz | bash && foundryup
```

### OpenZeppelin contracts not installing

Check git configuration and network connectivity:
```bash
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
```

### Compilation fails with "nightly required" (Rust/Stylus)

Install and use Rust nightly:
```bash
rustup install nightly
rustup default nightly
```

See [Troubleshooting](./troubleshooting.md) for more solutions.

## Development Questions

### How do I contribute to Nothung?

We welcome contributions! See our [Contributing Guide](./contributing.md) for details.

### Can I add custom extensions?

Currently, extensions are based on OpenZeppelin contracts. For custom functionality:
1. Generate the closest base contract
2. Add your custom code manually
3. Follow OpenZeppelin patterns

### How do I report bugs?

Please report bugs on [GitHub Issues](https://github.com/pxlvre/nothung/issues) with:
- Steps to reproduce
- Expected vs actual behavior
- System information
- Generated code (if relevant)

### Can I request new features?

Yes! Feature requests are welcome on GitHub Issues. Consider:
- Use case description
- How it fits with existing functionality
- Whether it should be an OpenZeppelin extension

### How do I stay updated?

- Follow releases on [GitHub](https://github.com/pxlvre/nothung/releases)
- Update with: `curl --proto '=https' --tlsv1.2 -sSf https://getnothung.pxlvre.dev | sh`
- Or use: `nothungup update`

## Comparison Questions

### How does Nothung compare to OpenZeppelin Wizard?

| Feature | Nothung | OZ Wizard |
|---------|---------|-----------|
| **Interface** | CLI + Interactive | Web-based |
| **Speed** | Very fast | Moderate |
| **Offline** | Yes | No |
| **Automation** | CLI scriptable | Manual copy/paste |
| **Languages** | Solidity + Rust/Stylus | Solidity only |
| **Tests/Scripts** | Auto-generated | Manual |

Both are great tools - Nothung focuses on developer workflow and automation.

### Nothung vs Foundry's forge create?

`forge create` compiles and deploys existing contracts. Nothung generates the contract source code first. They complement each other:

```bash
# Generate contract with Nothung
nothung new contract MyToken --solidity --oz-erc20 --with-script

# Deploy with Foundry
forge script script/MyToken.s.sol --broadcast
```

### Nothung vs manual OpenZeppelin setup?

Manual setup requires understanding all the imports, inheritance order, and function overrides. Nothung handles this automatically and provides working examples.

## Performance Questions

### Is Nothung fast?

Yes! Written in Rust for maximum performance:
- Contract generation: ~100ms
- With tests and scripts: ~200ms
- OpenZeppelin installation: ~2-5s (cached after first use)

### Does it work offline?

Mostly yes, after initial setup:
- ✅ Contract generation (offline)
- ✅ CLI operations (offline)
- ❌ OpenZeppelin installation (requires internet)
- ❌ Initial dependency downloads (requires internet)

### How much disk space does it need?

- Nothung binaries: ~50MB
- Rust toolchain: ~1GB
- Foundry: ~100MB
- OpenZeppelin contracts: ~50MB per project

## Future Questions

### What's planned for future versions?

**v0.2.0:**
- Full Rust/Stylus extension support
- Windows native support
- Template customization
- Cairo/StarkNet support

**Beyond:**
- Custom template system
- Plugin architecture
- Web interface
- IDE integrations

### Will Nothung always be free?

Yes! Nothung will always be free and open-source. We believe in making smart contract development accessible to everyone.

### How can I support the project?

- ⭐ Star the [GitHub repository](https://github.com/pxlvre/nothung)
- 🐛 Report bugs and issues
- 💡 Suggest new features
- 🤝 Contribute code or documentation
- 📢 Share with other developers

---

**Still have questions?** Ask on [GitHub Discussions](https://github.com/pxlvre/nothung/discussions) or check our [Documentation](https://getnothung.pxlvre.dev/docs).