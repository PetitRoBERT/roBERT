#!/bin/bash
set -e
# Any subsequent(*) commands which fail will cause the shell script to exit immediately

# NOTE: This sequential script is temporary and will be replaced by a Makefile after.

# Base builds

docker pull petitrobert/base-rust:latest || true
docker pull petitrobert/base-node:latest || true

docker pull petitrobert/reader:latest-build || true
docker pull petitrobert/reader:latest || true

docker pull petitrobert/front:latest-build || true
docker pull petitrobert/front:latest || true

docker pull petitrobert/node-services:latest-build || true
docker pull petitrobert/node-services:latest || true


### Rust
docker build ./base/rust \
    --cache-from petitrobert/base-rust:latest \
    --tag petitrobert/base-rust:latest

### Node
docker build ./base/node \
    --cache-from petitrobert/base-node:latest \
    --tag petitrobert/base-node:latest

# Services builds

### Reader Service

docker build ./services/reader \
            --target build-reader \
            --cache-from petitrobert/base-rust:latest \
            --cache-from petitrobert/reader:latest-build \
            --tag petitrobert/reader:latest-build

docker build ./services/reader \
            --target production-reader \
            --cache-from petitrobert/base-rust:latest \
            --cache-from petitrobert/reader:latest-build \
            --cache-from petitrobert/reader:latest \
            --tag petitrobert/reader:latest

### Nest Service

docker build ./services/node-services \
            --target build-node-services \
            --cache-from petitrobert/base-node:latest \
            --cache-from petitrobert/node-services:latest-build \
            --tag petitrobert/node-services:latest-build

docker build ./services/node-services \
            --target production-node-services \
            --cache-from petitrobert/base-node:latest \
            --cache-from petitrobert/node-services:latest-build \
            --cache-from petitrobert/node-services:latest \
            --tag petitrobert/node-services:latest

### Front Service

docker build ./services/front \
            --target build-front \
            --cache-from petitrobert/base-node:latest \
            --cache-from petitrobert/front:latest-build \
            --tag petitrobert/front:latest-build

docker build ./services/front \
            --target production-front \
            --cache-from petitrobert/base-node:latest \
            --cache-from petitrobert/front:latest-build \
            --cache-from petitrobert/front:latest \
            --tag petitrobert/front:latest
