#!/bin/bash
set -e
# Any subsequent(*) commands which fail will cause the shell script to exit immediately

# NOTE: This sequential script is temporary and will be replaced by a Makefile after.

# Base builds

### Rust
docker build ./base/rust --tag base-rust

### Javascript
docker build ./base/javascript --tag base-javascript

# Services builds

### Reader Service

docker build ./services/reader \
            --target build-reader \
            --tag reader:build-reader

docker build ./services/reader \
            --target production-reader \
            --tag reader:latest

### Nest Service
