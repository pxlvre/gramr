# Multi-stage build for optimal image size
# Stage 1: Builder
FROM rust:1.82-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /usr/src/gramr

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY lib/Cargo.toml lib/
COPY cli/Cargo.toml cli/
COPY wotan/Cargo.toml wotan/
COPY gramrup/Cargo.toml gramrup/

# Copy source code
COPY lib/src lib/src
COPY cli/src cli/src
COPY wotan/src wotan/src
COPY gramrup/src gramrup/src

# Build for release
RUN cargo build --release --all

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Foundry
RUN curl -L https://foundry.paradigm.xyz | bash && \
    /root/.foundry/bin/foundryup

# Copy binaries from builder
COPY --from=builder /usr/src/gramr/target/release/gramr /usr/local/bin/
COPY --from=builder /usr/src/gramr/target/release/wotan /usr/local/bin/
COPY --from=builder /usr/src/gramr/target/release/gramrup /usr/local/bin/

# Add foundry to PATH
ENV PATH="/root/.foundry/bin:${PATH}"

# Create workspace directory
WORKDIR /workspace

# Verify installations
RUN gramr --version && \
    wotan --help > /dev/null && \
    forge --version

# Default command
CMD ["gramr", "--help"]