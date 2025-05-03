FROM rust:1.86-bookworm

RUN rustup component add clippy

WORKDIR /usr/src/plog
COPY . .

RUN cargo build
