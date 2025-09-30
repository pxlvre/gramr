# Extensions Guide

Complete guide to OpenZeppelin extensions supported by Gramr.

## Overview

Extensions add specialized functionality to base token contracts. Gramr supports all major OpenZeppelin extensions with automatic compatibility checking.

## How Extensions Work

Extensions are implemented as additional contract inheritance:

```solidity
// Without extensions
contract MyToken is ERC20 { }

// With extensions
contract MyToken is ERC20, ERC20Burnable, ERC20Pausable { }
```

gramr automatically:

- ✅ Handles inheritance order
- ✅ Resolves function conflicts
- ✅ Adds required overrides
- ✅ Includes proper imports

## ERC20 Extensions (11 Available)

### `burnable` - Token Burning

Allows token holders to permanently destroy tokens.

```bash
gramr new contract BurnableToken --solidity --oz-erc20 --extensions burnable
```

**Features:**

- `burn(amount)` - Burn tokens from caller's account
- `burnFrom(account, amount)` - Burn tokens from another account (with allowance)

**Use Cases:**

- Deflationary tokenomics
- Token buyback and burn programs
- Gaming item destruction

### `pausable` - Emergency Controls

Adds emergency pause/unpause functionality.

```bash
gramr new contract PausableToken --solidity --oz-erc20 --extensions pausable
```

**Features:**

- `pause()` - Stop all token transfers
- `unpause()` - Resume token transfers
- `paused()` - Check pause status

**Use Cases:**

- Emergency response to exploits
- Regulatory compliance
- Maintenance windows

### `capped` - Supply Limits

Sets a maximum total supply that cannot be exceeded.

```bash
gramr new contract CappedToken --solidity --oz-erc20 --extensions capped
```

**Features:**

- `cap()` - View the maximum supply
- Automatic enforcement in minting

**Use Cases:**

- Fixed supply tokens
- Preventing inflation
- Collectible tokens

### `permit` - Gasless Approvals

Implements EIP-2612 for gasless token approvals.

```bash
gramr new contract PermitToken --solidity --oz-erc20 --extensions permit
```

**Features:**

- `permit()` - Approve spending via signature
- `nonces()` - Get current nonce for account
- `DOMAIN_SEPARATOR()` - EIP-712 domain separator

**Use Cases:**

- DeFi protocols with gasless UX
- Meta-transactions
- Reducing transaction costs

### `votes` - Governance

Adds voting power tracking for governance systems.

```bash
gramr new contract GovernanceToken --solidity --oz-erc20 --extensions votes
```

**Features:**

- `getVotes(account)` - Current voting power
- `getPastVotes(account, blockNumber)` - Historical voting power
- `delegate(delegatee)` - Delegate voting power

**Use Cases:**

- DAO governance tokens
- Voting-based protocols
- Delegation mechanisms

### `wrapper` - Token Wrapping

Wrap existing ERC20 tokens with additional functionality.

```bash
gramr new contract WrappedToken --solidity --oz-erc20 --extensions wrapper
```

**Features:**

- `depositFor(account, amount)` - Wrap tokens for another account
- `withdrawTo(account, amount)` - Unwrap tokens to another account
- `underlying()` - Get wrapped token address

**Use Cases:**

- Adding functionality to existing tokens
- Bridge tokens
- Yield-bearing wrappers

### `flashmint` - Flash Loans

Enables flash minting for arbitrage and liquidations.

```bash
gramr new contract FlashToken --solidity --oz-erc20 --extensions flashmint
```

**Features:**

- `flashLoan()` - Borrow tokens within single transaction
- `flashFee()` - Calculate flash loan fee
- `maxFlashLoan()` - Maximum available for flash loan

**Use Cases:**

- Arbitrage opportunities
- Liquidation mechanisms
- DeFi protocol integrations

### `temporaryapproval` - Single-Use Approvals

Approvals that expire after one use.

```bash
gramr new contract SecureToken --solidity --oz-erc20 --extensions temporaryapproval
```

**Features:**

- Time-limited approvals
- Single-use approvals
- Enhanced security

**Use Cases:**

- High-security applications
- Reduced approval attack surface
- Time-sensitive operations

### `bridgeable` - Cross-Chain Support

Adds cross-chain bridge compatibility.

```bash
gramr new contract BridgeToken --solidity --oz-erc20 --extensions bridgeable
```

**Features:**

- Bridge-compatible minting/burning
- Cross-chain transfer support
- Bridge operator controls

**Use Cases:**

- Multi-chain tokens
- Bridge protocols
- Cross-chain DeFi

### `erc1363` - Payable Token

Implements ERC1363 for tokens that can trigger contract calls.

```bash
gramr new contract PayableToken --solidity --oz-erc20 --extensions erc1363
```

**Features:**

- `transferAndCall()` - Transfer with callback
- `approveAndCall()` - Approve with callback
- Receiver contract integration

**Use Cases:**

- Payment processing
- Automatic contract interactions
- Gas-efficient operations

### `erc4626` - Tokenized Vaults

Implements ERC4626 standard for yield-bearing tokens.

```bash
gramr new contract VaultToken --solidity --oz-erc20 --extensions erc4626
```

**Features:**

- `deposit()` / `withdraw()` - Vault operations
- `previewDeposit()` / `previewWithdraw()` - Preview operations
- Asset/share conversion

**Use Cases:**

- Yield farming vaults
- Staking derivatives
- Liquidity mining

## ERC721 Extensions (8 Available)

### `enumerable` - Token Enumeration

Adds token enumeration and total supply tracking.

```bash
gramr new contract EnumerableNFT --solidity --oz-erc721 --extensions enumerable
```

**Features:**

- `totalSupply()` - Total number of tokens
- `tokenByIndex(index)` - Get token by index
- `tokenOfOwnerByIndex(owner, index)` - Get owner's token by index

**Use Cases:**

- NFT marketplaces
- Collection browsing
- Analytics and statistics

### `burnable` - NFT Burning

Allows token holders to permanently destroy NFTs.

```bash
gramr new contract BurnableNFT --solidity --oz-erc721 --extensions burnable
```

**Features:**

- `burn(tokenId)` - Burn a specific NFT

**Use Cases:**

- Gaming item destruction
- NFT evolution mechanics
- Deflationary collections

### `pausable` - Transfer Controls

Emergency pause/unpause for all NFT transfers.

```bash
gramr new contract PausableNFT --solidity --oz-erc721 --extensions pausable
```

**Features:**

- `pause()` / `unpause()` - Control transfers
- `paused()` - Check pause status

**Use Cases:**

- Emergency response
- Maintenance periods
- Regulatory compliance

### `consecutive` - Batch Minting

Efficient batch minting with consecutive token IDs.

```bash
gramr new contract BatchNFT --solidity --oz-erc721 --extensions consecutive
```

**Features:**

- `_mintConsecutive()` - Mint multiple tokens efficiently
- Gas-optimized batch operations

**Use Cases:**

- Large NFT drops
- Bulk minting operations
- Gas cost optimization

### `uristorage` - Dynamic Metadata

Per-token URI storage for dynamic metadata.

```bash
gramr new contract DynamicNFT --solidity --oz-erc721 --extensions uristorage
```

**Features:**

- `setTokenURI(tokenId, uri)` - Set individual token URI
- `tokenURI(tokenId)` - Get token-specific URI

**Use Cases:**

- Evolving NFTs
- Dynamic artwork
- Game state reflection

### `votes` - NFT-Based Governance

Voting power based on NFT ownership.

```bash
gramr new contract GovernanceNFT --solidity --oz-erc721 --extensions votes
```

**Features:**

- `getVotes(account)` - Voting power from NFTs
- `delegate()` - Delegate NFT voting power

**Use Cases:**

- NFT-based DAOs
- Membership voting
- Weighted governance

### `royalty` - ERC2981 Royalties

Implements ERC2981 royalty standard.

```bash
gramr new contract RoyaltyNFT --solidity --oz-erc721 --extensions royalty
```

**Features:**

- `setDefaultRoyalty()` - Set collection-wide royalty
- `setTokenRoyalty()` - Set per-token royalty
- `royaltyInfo()` - Query royalty information

**Use Cases:**

- Artist royalties
- Creator monetization
- Marketplace integration

### `wrapper` - NFT Wrapping

Wrap existing ERC721 tokens.

```bash
gramr new contract WrappedNFT --solidity --oz-erc721 --extensions wrapper
```

**Features:**

- Wrap/unwrap existing NFTs
- Add functionality to existing collections

**Use Cases:**

- Adding royalties to old collections
- Bridging NFTs
- Functionality upgrades

## ERC1155 Extensions (4 Available)

### `burnable` - Multi-Token Burning

Burn specific amounts of fungible or non-fungible tokens.

```bash
gramr new contract BurnableMultiToken --solidity --oz-erc1155 --extensions burnable
```

**Features:**

- `burn(account, id, amount)` - Burn specific tokens
- `burnBatch()` - Burn multiple token types

**Use Cases:**

- Gaming item consumption
- Token evolution
- Supply reduction

### `pausable` - Transfer Controls

Emergency controls for all token transfers.

```bash
gramr new contract PausableMultiToken --solidity --oz-erc1155 --extensions pausable
```

**Features:**

- Global pause/unpause for all tokens
- Emergency transfer controls

**Use Cases:**

- Security incidents
- Maintenance windows
- Regulatory requirements

### `supply` - Supply Tracking

Track total supply for each token type.

```bash
gramr new contract SupplyTrackedMultiToken --solidity --oz-erc1155 --extensions supply
```

**Features:**

- `totalSupply(id)` - Total supply of token type
- `exists(id)` - Check if token type exists

**Use Cases:**

- Analytics and statistics
- Supply cap enforcement
- Marketplace data

### `uristorage` - Per-Token URIs

Individual URI storage for each token type.

```bash
gramr new contract DynamicMultiToken --solidity --oz-erc1155 --extensions uristorage
```

**Features:**

- `setURI(tokenId, uri)` - Set URI for token type
- Individual metadata per token type

**Use Cases:**

- Different metadata per token type
- Dynamic game assets
- Evolving collections

## Extension Combinations

### Common Combinations

**Standard ERC20 Token:**

```bash
gramr new contract StandardToken --solidity --oz-erc20 --extensions burnable,pausable
```

**Governance Token:**

```bash
gramr new contract GovernanceToken --solidity --oz-erc20 --extensions votes,permit,pausable
```

**DeFi Token:**

```bash
gramr new contract DeFiToken --solidity --oz-erc20 --extensions permit,flashmint,votes
```

**Gaming NFT:**

```bash
gramr new contract GameNFT --solidity --oz-erc721 --extensions enumerable,burnable,uristorage
```

**Premium NFT Collection:**

```bash
gramr new contract PremiumNFT --solidity --oz-erc721 --extensions enumerable,royalty,pausable
```

**Gaming Assets:**

```bash
gramr new contract GameAssets --solidity --oz-erc1155 --extensions supply,burnable,uristorage
```

### Compatibility Matrix

| Extension    | ERC20 | ERC721 | ERC1155 | Notes         |
| ------------ | ----- | ------ | ------- | ------------- |
| `burnable`   | ✅    | ✅     | ✅      | Universal     |
| `pausable`   | ✅    | ✅     | ✅      | Universal     |
| `enumerable` | ❌    | ✅     | ❌      | NFT-specific  |
| `permit`     | ✅    | ❌     | ❌      | ERC20-only    |
| `votes`      | ✅    | ✅     | ❌      | Governance    |
| `royalty`    | ❌    | ✅     | ❌      | NFT royalties |
| `supply`     | ❌    | ❌     | ✅      | Multi-token   |
| `uristorage` | ❌    | ✅     | ✅      | Metadata      |

## Advanced Usage

### Multiple Extensions

```bash
# Combine multiple extensions
gramr new contract AdvancedToken --solidity --oz-erc20 \
  --extensions burnable,pausable,permit,votes,capped
```

### With Upgradeable Patterns

```bash
# Extensions + upgradeable
gramr new contract UpgradeableToken --solidity --oz-erc20 \
  --upgradeable --extensions burnable,pausable
```

### Extension Order

Gramr automatically handles inheritance order to prevent conflicts:

```solidity
// Gramr ensures correct order
contract MyToken is
    ERC20,           // Base first
    ERC20Burnable,   // Extensions in dependency order
    ERC20Pausable,
    ERC20Permit
{ }
```

## Custom Extension Development

For functionality not covered by OpenZeppelin extensions:

1. **Generate base contract with closest extensions**
2. **Add custom functionality manually**
3. **Follow OpenZeppelin patterns for consistency**

Example custom extension:

```solidity
// Add after Gramr generation
contract MyTokenWithCustom is ERC20, ERC20Burnable {
    // Custom functionality here
    function customMint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}
```

## Best Practices

### Security Considerations

1. **Understand each extension** - Read OpenZeppelin docs
2. **Test thoroughly** - Use generated test files
3. **Audit combinations** - Some extensions may interact unexpectedly
4. **Use access controls** - Most extensions need proper permissions

### Gas Optimization

1. **Only include needed extensions** - Each adds gas overhead
2. **Consider inheritance order** - May affect gas costs
3. **Test deployment costs** - More extensions = higher deployment gas

### Development Workflow

1. **Start simple** - Begin with basic contract
2. **Add extensions incrementally** - Test each addition
3. **Use wizard for exploration** - `wotan` helps discover options
4. **Read generated code** - Understand what each extension does

## Extension Reference

For complete extension documentation, see:

- [OpenZeppelin Documentation](https://docs.openzeppelin.com/)
- [OpenZeppelin Contracts](https://github.com/OpenZeppelin/openzeppelin-contracts)

Generated contracts include links to relevant OpenZeppelin documentation in comments.
