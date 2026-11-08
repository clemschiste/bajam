# --- Build ---
FROM rust:1.98.1 AS builder

WORKDIR /app

RUN mkdir -p /data

COPY Cargo.toml Cargo.lock ./
COPY setup ./setup
COPY .sqlx .sqlx
COPY src ./src
COPY migrations ./migrations
COPY templates ./templates

ENV SQLX_OFFLINE=true

RUN cargo build --release

# --- Runtime ---
FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/bajam /app/bajam
COPY static ./static

EXPOSE 3000

CMD ["/app/bajam"]
