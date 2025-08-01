# Multi-stage build for Rust media processing service
FROM rust:1.75-slim-bullseye as builder

# Install system dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libclang-dev \
    clang \
    cmake \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs files to build dependencies
RUN mkdir -p src
RUN echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Remove dummy files and copy real source code
RUN rm src/main.rs
COPY src src/

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ffmpeg \
    imagemagick \
    libssl1.1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Create necessary directories
RUN mkdir -p /app/tmp/{media,processing,uploads} /app/logs/rust

# Copy binary from builder stage
COPY --from=builder /app/target/release/media-processing-service /app/

# Copy run script
COPY run_rust.sh /app/
RUN chmod +x /app/run_rust.sh

# Set ownership
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Set working directory
WORKDIR /app

# Expose port
EXPOSE 8081

# Set environment variables
ENV RUST_PORT=8081
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1
ENV PORT=8081

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8081/api/v1/health || exit 1

# Default command
CMD ["./media-processing-service"] 