FROM rust:1.83-bookworm

RUN rustup component add clippy

WORKDIR /usr/src/plog
COPY . .

RUN cargo build
