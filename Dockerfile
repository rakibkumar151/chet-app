# Build Stage
FROM rust:slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Create a new empty shell project
WORKDIR /usr/src/app
COPY . .

# Build for release
RUN cargo build --release

# Runtime Stage
FROM debian:bookworm-slim

# Install dependencies (openssl is often needed for reqwest/axum)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/zero-messaging-api /usr/local/bin/

# Expose port 3000
EXPOSE 3000

# Run the binary
CMD ["zero-messaging-api"]
