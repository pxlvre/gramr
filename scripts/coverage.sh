#!/bin/bash

# Test Coverage Script for Gramr
# Generates HTML, LCOV, and JSON coverage reports

set -e

echo "🧪 Running test coverage analysis..."

# Check if cargo-tarpaulin is installed
if ! command -v cargo-tarpaulin &> /dev/null; then
    echo "📦 Installing cargo-tarpaulin..."
    cargo install cargo-tarpaulin
fi

# Clean previous coverage data
echo "🧹 Cleaning previous coverage data..."
rm -rf coverage/
mkdir -p coverage/

# Run coverage analysis
echo "🔍 Analyzing test coverage..."
cargo tarpaulin --config tarpaulin.toml --workspace --lib

# Check if coverage meets minimum threshold
COVERAGE_FILE="coverage/tarpaulin-report.json"
if [[ -f "$COVERAGE_FILE" ]]; then
    # Extract coverage percentage (requires jq)
    if command -v jq &> /dev/null; then
        COVERAGE=$(jq -r '.coverage' "$COVERAGE_FILE" 2>/dev/null || echo "unknown")
        echo "📊 Total coverage: ${COVERAGE}%"
        
        # Check threshold
        THRESHOLD=80
        if [[ "$COVERAGE" != "unknown" && $(echo "$COVERAGE >= $THRESHOLD" | bc -l) -eq 1 ]]; then
            echo "✅ Coverage meets minimum threshold of ${THRESHOLD}%"
        elif [[ "$COVERAGE" != "unknown" ]]; then
            echo "❌ Coverage below minimum threshold of ${THRESHOLD}%"
            exit 1
        fi
    else
        echo "📊 Coverage report generated (install jq for percentage analysis)"
    fi
fi

echo "📝 Coverage reports generated in coverage/ directory:"
echo "  - coverage/tarpaulin-report.html (HTML report)"
echo "  - coverage/lcov.info (LCOV format for CI/CD)"
echo "  - coverage/tarpaulin-report.json (JSON format)"

echo "🎉 Coverage analysis complete!"

# Open HTML report on macOS
if [[ "$OSTYPE" == "darwin"* ]] && [[ -f "coverage/tarpaulin-report.html" ]]; then
    echo "🌐 Opening HTML report..."
    open coverage/tarpaulin-report.html
fi