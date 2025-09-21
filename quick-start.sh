#!/bin/bash

# Quick Start Script for G3 Services
# Simple script for common operations - delegates to main start-services.sh

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Get the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
START_SCRIPT="$SCRIPT_DIR/start-services.sh"

# Check if main script exists
if [ ! -f "$START_SCRIPT" ]; then
    log_error "Main start-services.sh script not found at $START_SCRIPT"
    exit 1
fi

# Delegate to main script
case "${1:-start}" in
    start)
        log_info "Quick starting all G3 services..."
        "$START_SCRIPT" start
        ;;
    stop)
        log_info "Stopping all services..."
        "$START_SCRIPT" stop
        ;;
    restart)
        log_info "Restarting all services..."
        "$START_SCRIPT" restart
        ;;
    status)
        "$START_SCRIPT" status
        ;;
    *)
        echo "Usage: $0 [start|stop|restart|status]"
        echo "  start    - Start all services (default)"
        echo "  stop     - Stop all services"
        echo "  restart  - Restart all services"
        echo "  status   - Show service status"
        echo ""
        echo "This script delegates to the main start-services.sh script"
        ;;
esac
