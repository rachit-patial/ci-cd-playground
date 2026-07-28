#BUILD stage
FROM rust:1.90 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

#Runtime Stage
FROM debian:bookwork-slim

COPY --from=builder /app/target/release/hola /usr/loca/bin/hola

CMD ["hola"]