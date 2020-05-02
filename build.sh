#!/bin/bash
set -e
# Any subsequent(*) commands which fail will cause the shell script to exit immediately

# NOTE: This sequential script is temporary and will be replaced by a Makefile after.

# Base builds

docker pull petitrobert/base-rust:latest || true
docker pull petitrobert/base-node:latest || true

### Rust
docker build ./base/rust --tag petitrobert/base-rust:latest

### Node
docker build ./base/node --tag petitrobert/base-node:latest

# Services builds

### Reader Service

docker pull petitrobert/reader:latest-build || true
docker pull petitrobert/reader:latest || true

docker build ./services/reader \
            --target build-reader \
            --tag petitrobert/reader:latest-build

docker build ./services/reader \
            --target production-reader \
            --tag petitrobert/reader:latest

### Nest Service

docker pull petitrobert/api-gateway:latest-build || true
docker pull petitrobert/api-gateway:latest || true

docker build ./services/api-gateway \
            --target build-api-gateway \
            --tag petitrobert/api-gateway:latest-build

docker build ./services/api-gateway \
            --target production-api-gateway \
            --tag petitrobert/api-gateway:latest

### Front Service

docker pull petitrobert/front:latest-build || true
docker pull petitrobert/front:latest || true

docker build ./services/front \
            --target build-front \
            --tag petitrobert/front:latest-build

docker build ./services/front \
            --target production-front \
            --tag petitrobert/front:latest
