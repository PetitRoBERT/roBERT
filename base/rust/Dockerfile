FROM rust:1.43.0 as rust-base

RUN rustup component add rustfmt
RUN rustup component add clippy

WORKDIR /base/rust

# Copy the available Rust libs
COPY ./libs ./libs
