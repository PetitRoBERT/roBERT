##############################
#        BUILD IMAGE         #
##############################
FROM petitrobert/base-rust AS build-reader

WORKDIR /services

# Create empty project
RUN USER=root cargo new --bin reader
WORKDIR /services/reader

# Copy Cargo files
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Cache dependencies
RUN cargo build --release --target-dir="./target_dir"

# Remove empty project
RUN rm src/*.rs
RUN rm ./target_dir/release/deps/reader*

# Copy project files
COPY ./src ./src
COPY ./protos ./protos
COPY ./build.rs ./build.rs

# Clippy check
RUN cargo clippy --release --target-dir="./target_dir" -- -D warnings

# Release build
RUN cargo build --release --target-dir="./target_dir"

##############################
#         PROD IMAGE         #
##############################
FROM debian:jessie-slim as production-reader

WORKDIR /services/reader/

# Copying binaries from the build image
COPY --from=build-reader /services/reader/target_dir/release/reader-service* ./

CMD ["./reader-service"]
