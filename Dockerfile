FROM rust:1.53

RUN rustup component add clippy

WORKDIR /usr/src/myapp
COPY . .

RUN cargo build
