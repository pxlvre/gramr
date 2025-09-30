#!/bin/bash
# Build all Nothung binaries for release

set -e

echo "⚔️  Building Nothung binaries..."

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Build in release mode
echo -e "${YELLOW}Building nothung CLI...${NC}"
cargo build --release --bin nothung

echo -e "${YELLOW}Building wotan wizard...${NC}"
cargo build --release --bin wotan

echo -e "${YELLOW}Building nothungup installer...${NC}"
cargo build --release --bin nothungup

echo -e "${GREEN}✓ All binaries built successfully!${NC}"
echo ""
echo "Binaries location: target/release/"
ls -lh target/release/nothung target/release/wotan target/release/nothungup