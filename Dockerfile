# Build stage
FROM rust:1.75-slim as builder

WORKDIR /usr/src/app
# Copy over files
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build for release
RUN cargo build --release

# Distroless or small runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/backend-hyperliquid-candles .

# Setup environment variables needed
ENV PORT=3000
EXPOSE 3000

CMD ["./backend-hyperliquid-candles"]
