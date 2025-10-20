# Release Strategy & Branching Model

## Version Management

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR.MINOR.PATCH** (e.g., 1.2.3)
- All crates in workspace share the same version
- Version bumps are automated via GitHub Actions

## Branch Strategy

### Protected Branches

- **`main`** - Production ready code, protected
- **`develop`** - Integration branch for features

### Feature Branches

- **`feature/*`** - New features
- **`fix/*`** - Bug fixes
- **`docs/*`** - Documentation updates
- **`release/*`** - Release preparation

### Workflow

1. Create feature branch from `develop`
2. Make changes and test locally
3. Create PR to `develop`
4. After review, merge to `develop`
5. When ready for release, create `release/v*` branch
6. Merge release to `main` and tag
7. Merge back to `develop`

## Release Process

### Automated via GitHub Actions

1. Push tag `v*.*.*` to trigger release
2. CI builds all targets
3. Creates GitHub Release with binaries
4. Publishes `gramr` crate to crates.io
5. Updates installation script
6. Deploys docs to GitHub Pages

### Manual Release Steps

```bash
# 1. Update version in Cargo.toml
# 2. Update CHANGELOG.md
# 3. Commit changes
git add -A
git commit -m "chore: prepare release v0.2.0"

# 4. Create and push tag
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

## Publishing Strategy

### Crates.io

- Only `gramr` library crate is published
- CLI and other binaries distributed via GitHub Releases
- Auto-published on version tags

### GitHub Releases

- Pre-built binaries for all platforms
- Installation script included
- Release notes auto-generated from commits

### Installation Script

- Hosted at `https://gramr.pxlvre.eth.limo`
- Auto-updated on new releases
- Detects platform and downloads appropriate binary
