# Build Stage
FROM rust:slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Create a new empty shell project
WORKDIR /usr/src/app

# Copy over manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build and cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Now copy actual source code
COPY src ./src

# Touch main.rs to force cargo to recompile our code, not just use the cached dummy
RUN touch src/main.rs && cargo build --release

# Runtime Stage
FROM rust:slim

# Install dependencies (openssl is often needed for reqwest/axum)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/zero-messaging-api /usr/local/bin/

# Expose port 3000
EXPOSE 3000

# Run the binary
CMD ["zero-messaging-api"]
