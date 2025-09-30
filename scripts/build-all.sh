#!/bin/bash
# Build all Gramr binaries for release

set -e

echo "⚔️  Building Gramr binaries..."

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Build in release mode
echo -e "${YELLOW}Building gramr CLI...${NC}"
cargo build --release --bin gramr

echo -e "${YELLOW}Building wotan wizard...${NC}"
cargo build --release --bin wotan

echo -e "${YELLOW}Building gramrup installer...${NC}"
cargo build --release --bin gramrup

echo -e "${GREEN}✓ All binaries built successfully!${NC}"
echo ""
echo "Binaries location: target/release/"
ls -lh target/release/gramr target/release/wotan target/release/gramrup