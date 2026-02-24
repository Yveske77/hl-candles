# Build stage
FROM rust:1.75-slim as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
# Copy and rename the binary to match Railway's historical expected patterns
COPY --from=builder /app/target/release/backend-hyperliquid-candles ./server

ENV PORT=3000
EXPOSE 3000

CMD ["./server"]
