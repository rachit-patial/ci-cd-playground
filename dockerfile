#BUILD stage
FROM rust:1.90 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

#Runtime Stage
FROM debian:bookworm-slim

COPY --from=builder /app/target/release/hola /usr/local/bin/hola

CMD ["hola"]