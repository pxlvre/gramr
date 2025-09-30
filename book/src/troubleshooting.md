# Troubleshooting

Common issues and solutions for Gramr users.

## Installation Issues

### "Command not found" after installation

**Problem:** After running the installer, `gramr`, `wotan`, or `gramrup` commands are not found.

**Solutions:**

1. **Check PATH configuration:**

   ```bash
   echo $PATH | grep gramr
   ```

2. **Add to PATH manually:**

   ```bash
   # Add to your shell profile
   echo 'export PATH="$HOME/.gramr/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

3. **Restart terminal** or reload your shell profile.

4. **Verify installation location:**
   ```bash
   ls -la ~/.gramr/bin/
   ```

### Permission denied errors during installation

**Problem:** Installation fails with permission errors.

**Solutions:**

1. **Don't use sudo** with the installer:

   ```bash
   # Wrong
   sudo curl ... | sh

   # Correct
   curl --proto '=https' --tlsv1.2 -sSf https://getgramr.pxlvre.dev | sh
   ```

2. **Check directory permissions:**

   ```bash
   ls -la ~/.gramr/
   chown -R $USER:$USER ~/.gramr/
   ```

3. **Use custom install directory:**
   ```bash
   GRAMR_INSTALL_DIR=$HOME/tools/gramr curl ... | sh
   ```

### Download fails with SSL/TLS errors

**Problem:** Installation script fails to download files.

**Solutions:**

1. **Update certificates:**

   ```bash
   # Ubuntu/Debian
   sudo apt update && sudo apt install ca-certificates

   # macOS
   brew install ca-certificates
   ```

2. **Check system time** - Ensure your system clock is correct.

3. **Try alternative installation:**
   ```bash
   # Use cargo instead
   cargo install --git https://github.com/pxlvre/gramr gramr-cli
   ```

## Dependency Issues

### "forge: command not found"

**Problem:** Foundry is not installed or not in PATH.

**Solutions:**

1. **Install Foundry:**

   ```bash
   curl -L https://foundry.paradigm.xyz | bash
   foundryup
   ```

2. **Add Foundry to PATH:**

   ```bash
   echo 'export PATH="$HOME/.foundry/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

3. **Verify installation:**
   ```bash
   forge --version
   anvil --version
   cast --version
   ```

### "rustc: command not found"

**Problem:** Rust is not installed.

**Solutions:**

1. **Install Rust:**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Update existing Rust:**

   ```bash
   rustup update
   ```

3. **Check Rust version:**
   ```bash
   rustc --version  # Should be 1.70+
   ```

### OpenZeppelin contracts not installing

**Problem:** `gramr` fails to install OpenZeppelin dependencies.

**Solutions:**

1. **Check Foundry is working:**

   ```bash
   forge install openzeppelin/openzeppelin-contracts
   ```

2. **Check git configuration:**

   ```bash
   git config --global user.name "Your Name"
   git config --global user.email "your.email@example.com"
   ```

3. **Manual installation:**

   ```bash
   forge install openzeppelin/openzeppelin-contracts
   ```

4. **Network connectivity:**
   ```bash
   curl -I https://github.com/OpenZeppelin/openzeppelin-contracts
   ```

## Generation Issues

### "Project not detected" error

**Problem:** Gramr can't detect the project type.

**Solutions:**

1. **Initialize Foundry project:**

   ```bash
   forge init --force
   ```

2. **Check for foundry.toml:**

   ```bash
   ls -la foundry.toml
   ```

3. **Run from project root** - Ensure you're in the directory containing `foundry.toml`.

4. **Force project type:**
   ```bash
   GRAMR_PROJECT_DIR=$(pwd) gramr new contract MyToken --solidity --oz-erc20
   ```

### "Invalid contract name" error

**Problem:** Contract name doesn't meet naming requirements.

**Solutions:**

1. **Use PascalCase:** `MyToken` not `mytoken` or `my_token`
2. **Start with letter:** `Token1` not `1Token`
3. **Avoid keywords:** Don't use Solidity reserved words
4. **Use alphanumeric + underscores only:** `My_Token_V2` is ok

**Valid names:**

- ✅ `MyToken`
- ✅ `ERC20Token`
- ✅ `MyToken_V2`
- ✅ `AdvancedNFT`

**Invalid names:**

- ❌ `my-token` (contains hyphen)
- ❌ `2Token` (starts with number)
- ❌ `contract` (Solidity keyword)
- ❌ `My Token` (contains space)

### Extension compatibility errors

**Problem:** Extension not supported for chosen token type.

**Solutions:**

1. **Check compatibility table:**
   | Extension | ERC20 | ERC721 | ERC1155 |
   |-----------|-------|--------|---------|
   | `burnable` | ✅ | ✅ | ✅ |
   | `pausable` | ✅ | ✅ | ✅ |
   | `enumerable` | ❌ | ✅ | ❌ |
   | `permit` | ✅ | ❌ | ❌ |

2. **Use correct extensions for token type:**

   ```bash
   # Wrong
   gramr new contract MyNFT --solidity --oz-erc721 --extensions permit

   # Correct
   gramr new contract MyNFT --solidity --oz-erc721 --extensions enumerable
   ```

### Compilation errors after generation

**Problem:** Generated contracts don't compile.

**Solutions:**

1. **Check Foundry version:**

   ```bash
   foundryup  # Update to latest
   ```

2. **Clean and rebuild:**

   ```bash
   forge clean
   forge build
   ```

3. **Check Solidity version compatibility:**

   ```bash
   # In foundry.toml
   [profile.default]
   solc_version = "0.8.30"
   ```

4. **Verify OpenZeppelin installation:**
   ```bash
   ls -la lib/openzeppelin-contracts/
   ```

## Rust/Stylus Issues

### "nightly toolchain required" error

**Problem:** Stylus development requires Rust nightly.

**Solutions:**

1. **Install nightly:**

   ```bash
   rustup install nightly
   rustup default nightly
   ```

2. **Set nightly for project:**

   ```bash
   rustup override set nightly
   ```

3. **Verify nightly:**
   ```bash
   rustc --version  # Should show "nightly"
   ```

### WASM compilation fails

**Problem:** Rust/Stylus contracts fail to compile to WebAssembly.

**Solutions:**

1. **Add WASM target:**

   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. **Build with correct target:**

   ```bash
   cargo build --release --target wasm32-unknown-unknown
   ```

3. **Check for nightly features:**
   ```bash
   # Ensure using nightly
   rustup show
   ```

### OpenZeppelin Stylus dependency errors

**Problem:** Stylus contracts fail to compile with OpenZeppelin dependencies.

**Solutions:**

1. **Update Cargo.lock:**

   ```bash
   cargo update
   ```

2. **Clean and rebuild:**

   ```bash
   cargo clean
   cargo build
   ```

3. **Check dependency versions** in `Cargo.toml`:
   ```toml
   [dependencies]
   openzeppelin-stylus = "0.1"
   ```

## Performance Issues

### Slow contract generation

**Problem:** Gramr takes a long time to generate contracts.

**Solutions:**

1. **Check disk space:**

   ```bash
   df -h
   ```

2. **Use SSD storage** for better I/O performance.

3. **Close unnecessary applications** to free memory.

4. **Update Rust for faster compilation:**
   ```bash
   rustup update
   ```

### Slow Foundry builds

**Problem:** `forge build` is slow after contract generation.

**Solutions:**

1. **Enable parallel compilation:**

   ```bash
   # In foundry.toml
   [profile.default]
   optimizer = true
   optimizer_runs = 200
   ```

2. **Use faster linker:**

   ```bash
   # Linux only
   sudo apt install lld
   export RUSTFLAGS="-C link-arg=-fuse-ld=lld"
   ```

3. **Increase build jobs:**
   ```bash
   forge build --jobs 8
   ```

## Network Issues

### GitHub connectivity problems

**Problem:** Can't download dependencies from GitHub.

**Solutions:**

1. **Check connectivity:**

   ```bash
   curl -I https://github.com
   ```

2. **Configure git for HTTPS:**

   ```bash
   git config --global url."https://github.com/".insteadOf git@github.com:
   ```

3. **Use SSH instead:**

   ```bash
   git config --global url."git@github.com:".insteadOf https://github.com/
   ```

4. **Check firewall/proxy settings** - Ensure port 443 (HTTPS) is open.

### Crates.io download failures

**Problem:** Rust dependencies fail to download.

**Solutions:**

1. **Update registry:**

   ```bash
   cargo update
   ```

2. **Clear cargo cache:**

   ```bash
   rm -rf ~/.cargo/registry/cache/
   ```

3. **Use alternative registry mirror:**
   ```bash
   # In ~/.cargo/config.toml
   [source.crates-io]
   replace-with = "sparse"
   ```

## Docker Issues

### Container won't start

**Problem:** Docker containers fail to start or crash.

**Solutions:**

1. **Check Docker is running:**

   ```bash
   docker --version
   docker system info
   ```

2. **Pull latest image:**

   ```bash
   docker pull ghcr.io/pxlvre/gramr:latest
   ```

3. **Check logs:**

   ```bash
   docker logs <container-id>
   ```

4. **Rebuild with no cache:**
   ```bash
   docker-compose build --no-cache
   ```

### Permission issues in container

**Problem:** File permission errors when using Docker.

**Solutions:**

1. **Fix ownership after container use:**

   ```bash
   sudo chown -R $USER:$USER .
   ```

2. **Run with user mapping:**

   ```bash
   docker run --user $(id -u):$(id -g) -v $(pwd):/workspace ghcr.io/pxlvre/gramr:latest
   ```

3. **Use Docker Compose with user setting:**
   ```yaml
   services:
     gramr:
       user: "${UID}:${GID}"
   ```

## Debugging Tips

### Enable verbose logging

```bash
GRAMR_LOG_LEVEL=debug gramr new contract MyToken --solidity --oz-erc20
```

### Check generated files

```bash
# List all generated files
find . -name "*.sol" -newer foundry.toml

# Check file contents
cat src/MyToken.sol
```

### Verify project structure

```bash
# Check Foundry project structure
tree . -I "lib|cache|out"
```

### Test minimal case

```bash
# Try simplest possible generation
gramr new contract Test --solidity
```

## Getting Help

If you can't resolve an issue:

1. **Check existing issues:** [GitHub Issues](https://github.com/pxlvre/gramr/issues)
2. **Search documentation:** Use search in these docs
3. **Create minimal reproduction:** Test with simplest case
4. **Report bug:** Include version info and exact commands used

### Bug Report Template

```bash
# System info
uname -a
rustc --version
forge --version
gramr --version

# Exact command that failed
gramr new contract MyToken --solidity --oz-erc20

# Error output
[paste full error here]

# Project structure
ls -la
cat foundry.toml
```

## Common Error Codes

| Exit Code | Meaning                        | Common Cause                                   |
| --------- | ------------------------------ | ---------------------------------------------- |
| `0`       | Success                        | -                                              |
| `1`       | General error                  | Invalid arguments or configuration             |
| `2`       | Invalid arguments              | Wrong command syntax                           |
| `3`       | Project detection failed       | Not in Foundry project or missing foundry.toml |
| `4`       | File generation failed         | Permissions or disk space                      |
| `5`       | Dependency installation failed | Network or git issues                          |

Most issues are resolved by ensuring dependencies are properly installed and you're in a valid Foundry project directory.
