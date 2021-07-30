FROM rust:1.54-slim-buster

RUN rustup component add clippy

WORKDIR /usr/src/plog
COPY . .

RUN cargo build
