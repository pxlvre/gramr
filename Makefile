# Gramr Development Makefile

.PHONY: test test-lib test-cli test-all coverage coverage-html lint fmt check build build-release clean install-tools help

# Default target
help:
	@echo "🔨 Gramr Development Commands"
	@echo ""
	@echo "Testing:"
	@echo "  test          Run all tests"
	@echo "  test-lib      Run library tests only"
	@echo "  test-cli      Run CLI tests only"
	@echo "  test-all      Run all tests with verbose output"
	@echo ""
	@echo "Coverage:"
	@echo "  coverage      Generate test coverage report"
	@echo "  coverage-html Generate HTML coverage report and open it"
	@echo ""
	@echo "Code Quality:"
	@echo "  lint          Run clippy linter"
	@echo "  fmt           Format code with rustfmt"
	@echo "  check         Check code without building"
	@echo ""
	@echo "Building:"
	@echo "  build         Build all packages"
	@echo "  build-release Build release version"
	@echo ""
	@echo "Maintenance:"
	@echo "  clean         Clean build artifacts"
	@echo "  install-tools Install development tools"

# Testing
test:
	@echo "🧪 Running tests..."
	cargo test --workspace

test-lib:
	@echo "📚 Running library tests..."
	cargo test -p gramr --lib

test-cli:
	@echo "⚡ Running CLI tests..."
	cargo test -p gramr-cli
	cargo test -p wotan
	cargo test -p gramrup

test-all:
	@echo "🧪 Running all tests with verbose output..."
	cargo test --workspace --verbose

# Coverage
coverage:
	@echo "📊 Generating test coverage..."
	@if ! command -v cargo-tarpaulin >/dev/null 2>&1; then \
		echo "Installing cargo-tarpaulin..."; \
		cargo install cargo-tarpaulin; \
	fi
	cargo tarpaulin --config tarpaulin.toml --workspace --lib

coverage-html:
	@echo "📊 Generating HTML coverage report..."
	@if ! command -v cargo-tarpaulin >/dev/null 2>&1; then \
		echo "Installing cargo-tarpaulin..."; \
		cargo install cargo-tarpaulin; \
	fi
	cargo tarpaulin --config tarpaulin.toml --workspace --lib --out Html
	@echo "Opening coverage report..."
	@if [ -f "coverage/tarpaulin-report.html" ]; then \
		if command -v open >/dev/null 2>&1; then \
			open coverage/tarpaulin-report.html; \
		elif command -v xdg-open >/dev/null 2>&1; then \
			xdg-open coverage/tarpaulin-report.html; \
		else \
			echo "Coverage report generated at coverage/tarpaulin-report.html"; \
		fi \
	fi

# Code Quality
lint:
	@echo "🔍 Running clippy..."
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	@echo "🎨 Formatting code..."
	cargo fmt --all

check:
	@echo "🔍 Checking code..."
	cargo check --workspace --all-targets

# Building
build:
	@echo "🔨 Building all packages..."
	cargo build --workspace

build-release:
	@echo "🚀 Building release version..."
	cargo build --workspace --release

# Maintenance
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	rm -rf coverage/

install-tools:
	@echo "🛠️  Installing development tools..."
	cargo install cargo-tarpaulin
	cargo install cargo-watch
	@echo "✅ Development tools installed"

# Quick development workflow
dev: fmt lint test
	@echo "✅ Development workflow complete"

# CI workflow
ci: fmt lint test coverage
	@echo "✅ CI workflow complete"