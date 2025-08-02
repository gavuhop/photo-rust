FROM rust:1.88-slim as builder

# Install basic system dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo.toml and Cargo.lock first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo 'fn main() { println!("Hello World"); }' > src/main.rs

# Build dependencies
RUN cargo build --release

# Remove dummy files and copy real source code
RUN rm -rf src
COPY src/ ./src/

# Build the application
RUN cargo build --release

FROM debian:bookworm-slim

# Install basic runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/photo-rust .

# Set environment variables
ENV RUST_PORT=8081
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1
ENV PORT=8081

# Default command
CMD ["./photo-rust"] 