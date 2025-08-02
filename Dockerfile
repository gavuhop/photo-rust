# Use the official Rust image as a base
FROM rust:1.70 as builder

# Set working directory
WORKDIR /usr/src/app

# Copy the manifests
COPY Cargo.lock Cargo.toml ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Remove the dummy main.rs and copy the real source code
RUN rm src/main.rs
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install system dependencies including FFmpeg
RUN apt-get update && apt-get install -y \
    ca-certificates \
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1000 app

# Set working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /usr/src/app/target/release/media-processing-service /app/

# Change ownership to the app user
RUN chown -R app:app /app

# Switch to the app user
USER app

# Expose the port
EXPOSE 8081

# Set environment variables
ENV RUST_LOG=info
ENV PORT=8081

# Run the binary
CMD ["./media-processing-service"] 