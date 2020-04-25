#!/bin/bash
set -e
# Any subsequent(*) commands which fail will cause the shell script to exit immediately

# NOTE: This sequential script is temporary and will be replaced by a Makefile after.

# Base builds

### Rust
docker build ./base/rust --tag base-rust

### Node
docker build ./base/node --tag base-node

# Services builds

### Reader Service

docker build ./services/reader \
            --target build-reader \
            --tag reader:build

docker build ./services/reader \
            --target production-reader \
            --tag reader:latest

### Nest Service

docker build ./services/api-gateway \
            --target build-api-gateway \
            --tag api-gateway:build

docker build ./services/api-gateway \
            --target production-api-gateway \
            --tag api-gateway:latest
