#!/bin/bash

# Photo-Go Media Processing Service Runner (Rust)
echo "🦀 Starting Photo-Go Media Processing Service"
echo "============================================="
echo "📺 Video Transcoding + 🖼️ Image Processing"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
RUST_PORT=${RUST_PORT:-8081}
SERVICE_DIR="internal/transcode"
BUILD_MODE="dev"

# Function to check prerequisites
check_prerequisites() {
    echo "🔍 Checking prerequisites..."
    
    # Check Rust installation
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}❌ Rust/Cargo is not installed${NC}"
        echo "Please install Rust from https://rustup.rs/"
        exit 1
    fi
    
    local rust_version=$(rustc --version 2>/dev/null || echo "unknown")
    local cargo_version=$(cargo --version 2>/dev/null || echo "unknown")
    echo -e "${GREEN}✅ Rust found: $rust_version${NC}"
    echo -e "${GREEN}✅ Cargo found: $cargo_version${NC}"
    
    # Check if we're in the right directory
    if [ ! -d "$SERVICE_DIR" ]; then
        echo -e "${RED}❌ Service directory not found: $SERVICE_DIR${NC}"
        echo "Please run this script from the Photo-Go root directory"
        exit 1
    fi
    
    if [ ! -f "$SERVICE_DIR/Cargo.toml" ]; then
        echo -e "${RED}❌ Cargo.toml not found in $SERVICE_DIR${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ Media Processing Service found${NC}"
}

# Function to check system dependencies
check_system_dependencies() {
    echo "🔧 Checking system dependencies..."
    
    # Check for FFmpeg (required for video processing)
    if command -v ffmpeg &> /dev/null; then
        local ffmpeg_version=$(ffmpeg -version 2>/dev/null | head -n1 | awk '{print $3}')
        echo -e "${GREEN}✅ FFmpeg found: $ffmpeg_version${NC}"
    else
        echo -e "${YELLOW}⚠️ FFmpeg not found${NC}"
        echo "   Video transcoding may not work properly"
        echo "   Install FFmpeg: https://ffmpeg.org/download.html"
    fi
    
    # Check for ImageMagick (optional, for advanced image operations)
    if command -v convert &> /dev/null; then
        local imagemagick_version=$(convert -version | head -n1 | awk '{print $3}')
        echo -e "${GREEN}✅ ImageMagick found: $imagemagick_version${NC}"
    else
        echo -e "${YELLOW}⚠️ ImageMagick not found${NC}"
        echo "   Some advanced image operations may not be available"
        echo "   Install ImageMagick: https://imagemagick.org/script/download.php"
    fi
    
    # Check for development tools
    if command -v pkg-config &> /dev/null; then
        echo -e "${GREEN}✅ pkg-config found${NC}"
    else
        echo -e "${YELLOW}⚠️ pkg-config not found${NC}"
        echo "   May be needed for some native dependencies"
    fi
}

# Function to setup environment
setup_environment() {
    echo "⚙️ Setting up environment..."
    
    # Create necessary directories
    mkdir -p tmp/{media,processing,uploads}
    mkdir -p logs/rust
    echo -e "${GREEN}✅ Created required directories${NC}"
    
    # Set environment variables for Rust service
    export RUST_LOG=${RUST_LOG:-info}
    export RUST_BACKTRACE=${RUST_BACKTRACE:-1}
    export PORT=$RUST_PORT
    
    echo "Environment variables:"
    echo "  • RUST_LOG: $RUST_LOG"
    echo "  • PORT: $PORT"
    echo "  • RUST_BACKTRACE: $RUST_BACKTRACE"
}

# Function to check and install dependencies
check_dependencies() {
    echo "📦 Checking Rust dependencies..."
    
    cd "$SERVICE_DIR"
    
    # Check if Cargo.lock exists
    if [ ! -f "Cargo.lock" ]; then
        echo -e "${YELLOW}⚠️ Cargo.lock not found, this may take a while for first build${NC}"
    fi
    
    # Update dependencies if requested
    if [[ "$1" == "--update" ]]; then
        echo "🔄 Updating dependencies..."
        cargo update
    fi
    
    # Check dependencies
    echo "Checking dependencies..."
    cargo check --quiet
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ All dependencies are ready${NC}"
    else
        echo -e "${RED}❌ Failed to check dependencies${NC}"
        echo "Try running with --update flag to update dependencies"
        exit 1
    fi
    
    cd - > /dev/null
}

# Function to build the service
build_service() {
    echo "🔨 Building Media Processing Service..."
    
    cd "$SERVICE_DIR"
    
    if [[ "$BUILD_MODE" == "release" ]]; then
        echo "Building in release mode (optimized)..."
        cargo build --release
        local build_result=$?
    else
        echo "Building in development mode..."
        cargo build
        local build_result=$?
    fi
    
    cd - > /dev/null
    
    if [ $build_result -eq 0 ]; then
        echo -e "${GREEN}✅ Build completed successfully${NC}"
    else
        echo -e "${RED}❌ Build failed${NC}"
        exit 1
    fi
}

# Function to run tests
run_tests() {
    echo "🧪 Running tests..."
    
    cd "$SERVICE_DIR"
    cargo test --quiet
    local test_result=$?
    cd - > /dev/null
    
    if [ $test_result -eq 0 ]; then
        echo -e "${GREEN}✅ All tests passed${NC}"
    else
        echo -e "${YELLOW}⚠️ Some tests failed${NC}"
        echo "Continuing with service startup..."
    fi
}

# Function to run the service
run_service() {
    echo "🚀 Starting Media Processing Service..."
    echo ""
    echo -e "${BLUE}Service Information:${NC}"
    echo "  • Port: $RUST_PORT"
    echo "  • Mode: $BUILD_MODE"
    echo "  • Service URL: http://localhost:$RUST_PORT"
    echo "  • Health Check: http://localhost:$RUST_PORT/api/v1/health"
    echo ""
    echo -e "${YELLOW}Available endpoints:${NC}"
    echo "  📺 Video Processing:"
    echo "    • POST /api/v1/transcode/video"
    echo "    • POST /api/v1/transcode/audio"
    echo "    • POST /api/v1/metadata/extract"
    echo ""
    echo "  🖼️ Image Processing:"
    echo "    • POST /api/v1/image/resize"
    echo "    • POST /api/v1/image/filter"
    echo "    • POST /api/v1/image/effect"
    echo "    • POST /api/v1/image/compress"
    echo ""
    echo "  🔍 Analysis:"
    echo "    • POST /api/v1/analysis/colors"
    echo "    • POST /api/v1/analysis/quality"
    echo ""
    echo "  ✨ Enhancement:"
    echo "    • POST /api/v1/enhance/auto-enhance"
    echo "    • POST /api/v1/enhance/denoise"
    echo ""
    echo -e "${GREEN}Starting service... Press Ctrl+C to stop${NC}"
    echo "============================================="
    
    cd "$SERVICE_DIR"
    
    # Run the service
    if [[ "$BUILD_MODE" == "release" ]]; then
        if [[ "$1" == "--watch" ]] && command -v cargo-watch &> /dev/null; then
            echo -e "${BLUE}🔄 Running with hot reload (cargo-watch)${NC}"
            cargo watch -x 'run --release'
        else
            cargo run --release
        fi
    else
        if [[ "$1" == "--watch" ]] && command -v cargo-watch &> /dev/null; then
            echo -e "${BLUE}🔄 Running with hot reload (cargo-watch)${NC}"
            cargo watch -x run
        else
            cargo run
        fi
    fi
}

# Function to show service features
show_features() {
    echo -e "${BLUE}🎯 Service Features:${NC}"
    echo ""
    echo "📺 Video Processing:"
    echo "  • Video transcoding (MP4, AVI, MOV)"
    echo "  • Audio extraction and conversion"
    echo "  • Metadata extraction"
    echo "  • Quality and format optimization"
    echo ""
    echo "🖼️ Image Processing:"
    echo "  • Resize, rotate, crop, flip"
    echo "  • 20+ filters (blur, sepia, vintage, etc.)"
    echo "  • Artistic effects (oil painting, watercolor)"
    echo "  • Smart compression and format conversion"
    echo "  • Watermarking (text and image)"
    echo ""
    echo "🔍 Analysis & AI:"
    echo "  • Color analysis and dominant colors"
    echo "  • Image quality assessment"
    echo "  • Face detection (placeholder)"
    echo "  • Duplicate image detection"
    echo ""
    echo "✨ Enhancement:"
    echo "  • Auto enhancement"
    echo "  • Noise reduction"
    echo "  • Super resolution (placeholder)"
    echo "  • Color correction"
    echo ""
    echo "📦 Batch Processing:"
    echo "  • Batch resize, optimize, convert"
    echo "  • Parallel processing"
    echo "  • Progress tracking"
    echo ""
}

# Function to show help
show_help() {
    echo "Photo-Go Media Processing Service Runner"
    echo ""
    echo "Usage:"
    echo "  ./run_rust.sh [options]"
    echo ""
    echo "Options:"
    echo "  -p, --port PORT      Set service port (default: 8081)"
    echo "  -r, --release        Build and run in release mode"
    echo "  -w, --watch          Run with hot reload (requires cargo-watch)"
    echo "  -t, --test           Run tests before starting"
    echo "  -u, --update         Update dependencies before building"
    echo "  -b, --build-only     Only build and exit"
    echo "  -f, --features       Show available features"
    echo "  -h, --help           Show this help message"
    echo "  --check-only         Only check prerequisites and exit"
    echo ""
    echo "Examples:"
    echo "  ./run_rust.sh                    # Start service in dev mode"
    echo "  ./run_rust.sh -r                # Start in release mode"
    echo "  ./run_rust.sh -w                # Start with hot reload"
    echo "  ./run_rust.sh -p 9000           # Start on port 9000"
    echo "  ./run_rust.sh -t -r             # Run tests then start in release mode"
    echo ""
    echo "Development tips:"
    echo "  • Install cargo-watch for hot reload: cargo install cargo-watch"
    echo "  • Use release mode for better performance"
    echo "  • Check logs in ./logs/rust/ directory"
    echo ""
}

# Function to handle cleanup on exit
cleanup() {
    echo ""
    echo "🛑 Shutting down Media Processing Service..."
    
    # Kill any background processes
    jobs -p | xargs -r kill 2>/dev/null
    
    echo "✅ Cleanup completed"
}

# Main execution
main() {
    local run_tests_flag=false
    local build_only=false
    local watch_mode=false
    local update_deps=false
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -p|--port)
                RUST_PORT="$2"
                shift 2
                ;;
            -r|--release)
                BUILD_MODE="release"
                shift
                ;;
            -w|--watch)
                watch_mode="--watch"
                shift
                ;;
            -t|--test)
                run_tests_flag=true
                shift
                ;;
            -u|--update)
                update_deps="--update"
                shift
                ;;
            -b|--build-only)
                build_only=true
                shift
                ;;
            -f|--features)
                show_features
                exit 0
                ;;
            -h|--help)
                show_help
                exit 0
                ;;
            --check-only)
                check_prerequisites
                check_system_dependencies
                echo -e "${GREEN}✅ All prerequisites check passed${NC}"
                exit 0
                ;;
            *)
                echo -e "${RED}❌ Unknown option: $1${NC}"
                show_help
                exit 1
                ;;
        esac
    done
    
    # Set trap for cleanup
    trap cleanup EXIT INT TERM
    
    # Run setup steps
    check_prerequisites
    check_system_dependencies
    setup_environment
    check_dependencies $update_deps
    
    # Run tests if requested
    if [[ "$run_tests_flag" == true ]]; then
        run_tests
    fi
    
    # Build the service
    build_service
    
    # Exit if build-only mode
    if [[ "$build_only" == true ]]; then
        echo -e "${GREEN}✅ Build completed. Exiting as requested.${NC}"
        exit 0
    fi
    
    # Start the service
    run_service $watch_mode
}

# Check if script is being sourced or executed
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi