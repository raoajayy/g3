#!/bin/bash

# G3ICAP Production Deployment Script
# This script deploys G3ICAP to a production environment

set -euo pipefail

# Configuration
BINARY_NAME="g3icap"
INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="/etc/g3icap"
LOG_DIR="/var/log/g3icap"
RUN_DIR="/var/run/g3icap"
SERVICE_USER="g3icap"
SERVICE_GROUP="g3icap"
SERVICE_FILE="g3icap.service"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running as root
check_root() {
    if [[ $EUID -ne 0 ]]; then
        log_error "This script must be run as root"
        exit 1
    fi
}

# Create user and group
create_user() {
    log_info "Creating user and group..."
    
    if ! id "$SERVICE_USER" &>/dev/null; then
        useradd --system --no-create-home --shell /bin/false "$SERVICE_USER"
        log_info "Created user: $SERVICE_USER"
    else
        log_info "User $SERVICE_USER already exists"
    fi
    
    if ! getent group "$SERVICE_GROUP" &>/dev/null; then
        groupadd --system "$SERVICE_GROUP"
        log_info "Created group: $SERVICE_GROUP"
    else
        log_info "Group $SERVICE_GROUP already exists"
    fi
}

# Create directories
create_directories() {
    log_info "Creating directories..."
    
    mkdir -p "$CONFIG_DIR"
    mkdir -p "$LOG_DIR"
    mkdir -p "$RUN_DIR"
    
    chown "$SERVICE_USER:$SERVICE_GROUP" "$LOG_DIR"
    chown "$SERVICE_USER:$SERVICE_GROUP" "$RUN_DIR"
    chmod 755 "$CONFIG_DIR"
    chmod 755 "$LOG_DIR"
    chmod 755 "$RUN_DIR"
    
    log_info "Created directories: $CONFIG_DIR, $LOG_DIR, $RUN_DIR"
}

# Install binary
install_binary() {
    log_info "Installing binary..."
    
    if [[ ! -f "target/release/$BINARY_NAME" ]]; then
        log_error "Binary not found. Please build first with: cargo build --release"
        exit 1
    fi
    
    cp "target/release/$BINARY_NAME" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
    chown root:root "$INSTALL_DIR/$BINARY_NAME"
    
    log_info "Installed binary to $INSTALL_DIR/$BINARY_NAME"
}

# Install configuration
install_config() {
    log_info "Installing configuration..."
    
    if [[ -f "g3icap.yaml" ]]; then
        cp "g3icap.yaml" "$CONFIG_DIR/"
        chown root:root "$CONFIG_DIR/g3icap.yaml"
        chmod 644 "$CONFIG_DIR/g3icap.yaml"
        log_info "Installed configuration to $CONFIG_DIR/g3icap.yaml"
    else
        log_warn "Configuration file not found. Using default configuration."
    fi
}

# Install systemd service
install_service() {
    log_info "Installing systemd service..."
    
    if [[ -f "$SERVICE_FILE" ]]; then
        cp "$SERVICE_FILE" "/etc/systemd/system/"
        systemctl daemon-reload
        systemctl enable "$BINARY_NAME"
        log_info "Installed and enabled systemd service"
    else
        log_error "Service file not found: $SERVICE_FILE"
        exit 1
    fi
}

# Start service
start_service() {
    log_info "Starting service..."
    
    systemctl start "$BINARY_NAME"
    
    # Wait a moment for the service to start
    sleep 2
    
    if systemctl is-active --quiet "$BINARY_NAME"; then
        log_info "Service started successfully"
    else
        log_error "Failed to start service"
        systemctl status "$BINARY_NAME"
        exit 1
    fi
}

# Show status
show_status() {
    log_info "Service status:"
    systemctl status "$BINARY_NAME" --no-pager
    
    log_info "Service logs (last 20 lines):"
    journalctl -u "$BINARY_NAME" --no-pager -n 20
}

# Main deployment function
deploy() {
    log_info "Starting G3ICAP deployment..."
    
    check_root
    create_user
    create_directories
    install_binary
    install_config
    install_service
    start_service
    show_status
    
    log_info "Deployment completed successfully!"
    log_info "Service is running on port 1344"
    log_info "Configuration: $CONFIG_DIR/g3icap.yaml"
    log_info "Logs: $LOG_DIR/g3icap.log"
    log_info "Systemd service: $BINARY_NAME"
}

# Uninstall function
uninstall() {
    log_info "Uninstalling G3ICAP..."
    
    check_root
    
    # Stop and disable service
    if systemctl is-active --quiet "$BINARY_NAME"; then
        systemctl stop "$BINARY_NAME"
    fi
    
    if systemctl is-enabled --quiet "$BINARY_NAME"; then
        systemctl disable "$BINARY_NAME"
    fi
    
    # Remove files
    rm -f "/etc/systemd/system/$SERVICE_FILE"
    rm -f "$INSTALL_DIR/$BINARY_NAME"
    rm -f "$CONFIG_DIR/g3icap.yaml"
    
    # Remove directories (optional)
    read -p "Remove log and run directories? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm -rf "$LOG_DIR"
        rm -rf "$RUN_DIR"
    fi
    
    # Remove user and group (optional)
    read -p "Remove user and group? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        userdel "$SERVICE_USER" 2>/dev/null || true
        groupdel "$SERVICE_GROUP" 2>/dev/null || true
    fi
    
    systemctl daemon-reload
    log_info "Uninstallation completed"
}

# Show usage
usage() {
    echo "Usage: $0 [deploy|uninstall|status|restart|stop|start]"
    echo ""
    echo "Commands:"
    echo "  deploy    - Deploy G3ICAP to production"
    echo "  uninstall - Remove G3ICAP from system"
    echo "  status    - Show service status"
    echo "  restart   - Restart service"
    echo "  stop      - Stop service"
    echo "  start     - Start service"
    echo ""
}

# Main script
case "${1:-deploy}" in
    deploy)
        deploy
        ;;
    uninstall)
        uninstall
        ;;
    status)
        show_status
        ;;
    restart)
        systemctl restart "$BINARY_NAME"
        show_status
        ;;
    stop)
        systemctl stop "$BINARY_NAME"
        log_info "Service stopped"
        ;;
    start)
        systemctl start "$BINARY_NAME"
        show_status
        ;;
    *)
        usage
        exit 1
        ;;
esac
