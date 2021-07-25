FROM rust:1.53

RUN rustup component add clippy

WORKDIR /usr/src/plog
COPY . .

RUN cargo build
