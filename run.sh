#!/bin/bash

# Simple script to run Media Processing Service

echo "🚀 Starting Media Processing Service..."

# Set default port if not provided
PORT=${PORT:-8082}
RUST_LOG=${RUST_LOG:-info}

echo "📡 Server will run on port: $PORT"
echo "📊 Log level: $RUST_LOG"

# Check if FFmpeg is installed
if ! command -v ffmpeg &> /dev/null; then
    echo "⚠️  Warning: FFmpeg not found. Please install FFmpeg:"
    echo "   sudo apt update && sudo apt install ffmpeg"
    echo ""
fi

# Run the server
echo "🔄 Starting server..."
PORT=$PORT RUST_LOG=$RUST_LOG cargo run

echo "✅ Server stopped." 