FROM rust:1.82.0 as builder

WORKDIR /usr/src/app

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock .env ./
COPY src src
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine
RUN mkdir -p /opt/app
WORKDIR /opt/app

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/sandbox_rust_torch /opt/app/sandbox_rust_torch
COPY --from=builder /usr/src/app/.env /opt/app/.env
CMD ["./sandbox_rust_torch"]
