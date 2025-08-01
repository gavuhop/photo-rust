#!/bin/bash

# Photo-Rust Development Script with Hot Reload
# This script provides easy development setup with hot reload

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
SERVICE_NAME="photo-rust-dev"
COMPOSE_FILE="docker-compose.dev.yml"

# Function to show help
show_help() {
    echo "Photo-Rust Development Script"
    echo ""
    echo "Usage:"
    echo "  ./dev.sh [command]"
    echo ""
    echo "Commands:"
    echo "  start     - Start development environment with hot reload"
    echo "  stop      - Stop development environment"
    echo "  restart   - Restart development environment"
    echo "  logs      - Show development logs"
    echo "  shell     - Open shell in development container"
    echo "  build     - Build development image"
    echo "  clean     - Clean up containers and volumes"
    echo "  health    - Check service health"
    echo "  help      - Show this help message"
    echo ""
    echo "Examples:"
    echo "  ./dev.sh start    # Start with hot reload"
    echo "  ./dev.sh logs     # View logs"
    echo "  ./dev.sh shell    # Open shell"
    echo ""
}

# Function to check prerequisites
check_prerequisites() {
    echo "🔍 Checking prerequisites..."
    
    # Check Docker
    if ! command -v docker &> /dev/null; then
        echo -e "${RED}❌ Docker is not installed${NC}"
        exit 1
    fi
    
    # Check Docker Compose
    if ! command -v docker-compose &> /dev/null; then
        echo -e "${RED}❌ Docker Compose is not installed${NC}"
        exit 1
    fi
    
    # Check if we're in the right directory
    if [ ! -f "docker-compose.dev.yml" ]; then
        echo -e "${RED}❌ docker-compose.dev.yml not found${NC}"
        echo "Please run this script from the project root directory"
        exit 1
    fi
    
    echo -e "${GREEN}✅ All prerequisites met${NC}"
}

# Function to start development environment
start_dev() {
    echo "🚀 Starting Photo-Rust development environment..."
    echo ""
    echo -e "${BLUE}Features:${NC}"
    echo "  • Hot reload enabled (cargo-watch)"
    echo "  • Source code mounted for live changes"
    echo "  • Automatic rebuilds on file changes"
    echo "  • Debug logging enabled"
    echo ""
    echo -e "${YELLOW}Service Information:${NC}"
    echo "  • URL: http://localhost:8081"
    echo "  • Health: http://localhost:8081/health"
    echo "  • Logs: make logs-dev"
    echo "  • Shell: make shell-dev"
    echo ""
    
    # Build and start
    docker-compose -f $COMPOSE_FILE up --build -d
    
    echo -e "${GREEN}✅ Development environment started!${NC}"
    echo ""
    echo "📝 Useful commands:"
    echo "  make logs-dev     # View logs"
    echo "  make shell-dev    # Open shell"
    echo "  make health       # Check health"
    echo "  ./dev.sh stop     # Stop environment"
    echo ""
    echo "🔗 Service is running at: http://localhost:8081"
}

# Function to stop development environment
stop_dev() {
    echo "🛑 Stopping development environment..."
    docker-compose -f $COMPOSE_FILE down
    echo -e "${GREEN}✅ Development environment stopped${NC}"
}

# Function to restart development environment
restart_dev() {
    echo "🔄 Restarting development environment..."
    stop_dev
    sleep 2
    start_dev
}

# Function to show logs
show_logs() {
    echo "📋 Showing development logs..."
    echo "Press Ctrl+C to exit logs"
    docker-compose -f $COMPOSE_FILE logs -f $SERVICE_NAME
}

# Function to open shell
open_shell() {
    echo "🐚 Opening shell in development container..."
    docker-compose -f $COMPOSE_FILE exec $SERVICE_NAME /bin/bash
}

# Function to build development image
build_dev() {
    echo "🔨 Building development image..."
    docker-compose -f $COMPOSE_FILE build
    echo -e "${GREEN}✅ Development image built${NC}"
}

# Function to clean up
clean_dev() {
    echo "🧹 Cleaning up development environment..."
    docker-compose -f $COMPOSE_FILE down -v
    docker system prune -f
    echo -e "${GREEN}✅ Cleanup completed${NC}"
}

# Function to check health
check_health() {
    echo "🏥 Checking service health..."
    if curl -f http://localhost:8081/health > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Service is healthy${NC}"
        curl -s http://localhost:8081/health | jq . 2>/dev/null || curl -s http://localhost:8081/health
    else
        echo -e "${RED}❌ Service is not responding${NC}"
        echo "Try starting the service with: ./dev.sh start"
    fi
}

# Main execution
main() {
    case "${1:-help}" in
        start)
            check_prerequisites
            start_dev
            ;;
        stop)
            stop_dev
            ;;
        restart)
            check_prerequisites
            restart_dev
            ;;
        logs)
            show_logs
            ;;
        shell)
            open_shell
            ;;
        build)
            build_dev
            ;;
        clean)
            clean_dev
            ;;
        health)
            check_health
            ;;
        help|*)
            show_help
            ;;
    esac
}

# Run main function
main "$@" 