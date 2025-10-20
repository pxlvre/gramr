# Deployment Guide

## Docker Deployment

### Using Docker Hub Image

```bash
docker run --rm -v $(pwd):/workspace ghcr.io/pxlvre/gramr:latest gramr --help
```

### Building Locally

```bash
docker build -t gramr .
docker run --rm -v $(pwd):/workspace gramr gramr --help
```

## Direct Installation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://gramr.pxlvre.eth.limo/install.sh | sh
```

## Development

See [README.md](README.md) for development setup instructions.