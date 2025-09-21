#!/bin/bash

# Arcus Admin Console Deployment Script
set -e

echo "🚀 Starting Arcus Admin Console deployment..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
PROJECT_NAME="arcus-admin-console"
API_PORT=3001
FRONTEND_PORT=3000
DOCKER_COMPOSE_FILE="docker-compose.yml"

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Docker is running
check_docker() {
    if ! docker info > /dev/null 2>&1; then
        print_error "Docker is not running. Please start Docker and try again."
        exit 1
    fi
    print_status "Docker is running ✓"
}

# Check if required files exist
check_files() {
    local required_files=(
        "package.json"
        "Dockerfile"
        "docker-compose.yml"
        "metrics-api/Cargo.toml"
    )
    
    for file in "${required_files[@]}"; do
        if [[ ! -f "$file" ]]; then
            print_error "Required file $file not found"
            exit 1
        fi
    done
    print_status "All required files found ✓"
}

# Install dependencies
install_dependencies() {
    print_status "Installing frontend dependencies..."
    npm ci
    
    print_status "Installing Rust dependencies..."
    cd metrics-api
    cargo build --release
    cd ..
    print_status "Dependencies installed ✓"
}

# Build Docker images
build_images() {
    print_status "Building Docker images..."
    docker-compose -f $DOCKER_COMPOSE_FILE build --no-cache
    print_status "Docker images built ✓"
}

# Start services
start_services() {
    print_status "Starting services..."
    docker-compose -f $DOCKER_COMPOSE_FILE up -d
    
    # Wait for services to be ready
    print_status "Waiting for services to start..."
    sleep 10
    
    # Check if services are running
    if docker-compose -f $DOCKER_COMPOSE_FILE ps | grep -q "Up"; then
        print_status "Services started successfully ✓"
    else
        print_error "Failed to start services"
        docker-compose -f $DOCKER_COMPOSE_FILE logs
        exit 1
    fi
}

# Health check
health_check() {
    print_status "Performing health check..."
    
    # Check frontend
    if curl -f http://localhost:$FRONTEND_PORT > /dev/null 2>&1; then
        print_status "Frontend is healthy ✓"
    else
        print_warning "Frontend health check failed"
    fi
    
    # Check API
    if curl -f http://localhost:$API_PORT/health > /dev/null 2>&1; then
        print_status "API is healthy ✓"
    else
        print_warning "API health check failed"
    fi
}

# Show deployment info
show_info() {
    echo ""
    echo "🎉 Deployment completed successfully!"
    echo ""
    echo "📊 Access URLs:"
    echo "  Frontend: http://localhost:$FRONTEND_PORT"
    echo "  API: http://localhost:$API_PORT"
    echo "  Grafana: http://localhost:3001"
    echo "  Prometheus: http://localhost:9090"
    echo ""
    echo "🔧 Management commands:"
    echo "  View logs: docker-compose -f $DOCKER_COMPOSE_FILE logs -f"
    echo "  Stop services: docker-compose -f $DOCKER_COMPOSE_FILE down"
    echo "  Restart services: docker-compose -f $DOCKER_COMPOSE_FILE restart"
    echo ""
}

# Cleanup function
cleanup() {
    print_status "Cleaning up..."
    docker-compose -f $DOCKER_COMPOSE_FILE down
}

# Main deployment function
deploy() {
    print_status "Starting deployment process..."
    
    check_docker
    check_files
    install_dependencies
    build_images
    start_services
    health_check
    show_info
}

# Handle script interruption
trap cleanup EXIT

# Parse command line arguments
case "${1:-deploy}" in
    "deploy")
        deploy
        ;;
    "stop")
        print_status "Stopping services..."
        docker-compose -f $DOCKER_COMPOSE_FILE down
        print_status "Services stopped ✓"
        ;;
    "restart")
        print_status "Restarting services..."
        docker-compose -f $DOCKER_COMPOSE_FILE restart
        print_status "Services restarted ✓"
        ;;
    "logs")
        docker-compose -f $DOCKER_COMPOSE_FILE logs -f
        ;;
    "status")
        docker-compose -f $DOCKER_COMPOSE_FILE ps
        ;;
    *)
        echo "Usage: $0 {deploy|stop|restart|logs|status}"
        exit 1
        ;;
esac
