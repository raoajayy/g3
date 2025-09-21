#!/bin/bash

# G3ICAP Startup Script with YARA Support
# This script demonstrates how to start G3ICAP with YARA antivirus module

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_FILE="${SCRIPT_DIR}/examples/yara_config.yaml"
RULES_DIR="${SCRIPT_DIR}/examples/yara_rules"
QUARANTINE_DIR="/tmp/g3icap_quarantine"
LOG_DIR="/tmp/g3icap_logs"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
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

log_debug() {
    echo -e "${BLUE}[DEBUG]${NC} $1"
}

# Check if running as root
check_root() {
    if [[ $EUID -eq 0 ]]; then
        log_warn "Running as root. Consider running as a non-root user for security."
    fi
}

# Create necessary directories
create_directories() {
    log_info "Creating necessary directories..."
    
    # Create YARA rules directory
    if [[ ! -d "$RULES_DIR" ]]; then
        mkdir -p "$RULES_DIR"
        log_info "Created YARA rules directory: $RULES_DIR"
    fi
    
    # Create quarantine directory
    if [[ ! -d "$QUARANTINE_DIR" ]]; then
        mkdir -p "$QUARANTINE_DIR"
        log_info "Created quarantine directory: $QUARANTINE_DIR"
    fi
    
    # Create log directory
    if [[ ! -d "$LOG_DIR" ]]; then
        mkdir -p "$LOG_DIR"
        log_info "Created log directory: $LOG_DIR"
    fi
}

# Copy example YARA rules
setup_yara_rules() {
    log_info "Setting up YARA rules..."
    
    local example_rules="${SCRIPT_DIR}/examples/yara_rules_example.yar"
    local rules_file="${RULES_DIR}/malware_detection.yar"
    
    if [[ -f "$example_rules" ]]; then
        cp "$example_rules" "$rules_file"
        log_info "Copied example YARA rules to: $rules_file"
    else
        log_warn "Example YARA rules not found. Creating basic rules..."
        cat > "$rules_file" << 'EOF'
rule Malware_Generic {
    meta:
        description = "Generic malware detection rule"
        author = "G3ICAP Team"
        priority = 8
        threat_level = "high"
        category = "malware"
    
    strings:
        $malware_strings = {
            "malware", "virus", "trojan", "worm", "backdoor",
            "rootkit", "spyware", "adware", "ransomware"
        }
    
    condition:
        1 of ($malware_strings)
}

rule Phishing_Generic {
    meta:
        description = "Generic phishing detection rule"
        author = "G3ICAP Team"
        priority = 7
        threat_level = "medium"
        category = "phishing"
    
    strings:
        $phishing_strings = {
            "verify your account", "click here", "urgent action required",
            "suspended account", "security alert", "confirm your identity"
        }
    
    condition:
        2 of ($phishing_strings)
}
EOF
        log_info "Created basic YARA rules: $rules_file"
    fi
}

# Update configuration file
update_config() {
    log_info "Updating configuration file..."
    
    local config_file="$CONFIG_FILE"
    
    if [[ ! -f "$config_file" ]]; then
        log_error "Configuration file not found: $config_file"
        exit 1
    fi
    
    # Update paths in configuration
    sed -i.bak "s|/etc/g3icap/yara_rules|$RULES_DIR|g" "$config_file"
    sed -i.bak "s|/var/quarantine/g3icap|$QUARANTINE_DIR|g" "$config_file"
    sed -i.bak "s|/var/log/g3icap|$LOG_DIR|g" "$config_file"
    
    log_info "Updated configuration file: $config_file"
}

# Check dependencies
check_dependencies() {
    log_info "Checking dependencies..."
    
    # Check if cargo is available
    if ! command -v cargo &> /dev/null; then
        log_warn "Cargo not found. You may need to install Rust toolchain."
        log_warn "Visit: https://rustup.rs/"
    fi
    
    # Check if the project can be built
    if [[ -f "${SCRIPT_DIR}/Cargo.toml" ]]; then
        log_info "Rust project detected. Attempting to build..."
        if command -v cargo &> /dev/null; then
            cd "$SCRIPT_DIR"
            if cargo build --release; then
                log_info "Project built successfully"
            else
                log_warn "Project build failed. Continuing with existing binary..."
            fi
        else
            log_warn "Cargo not available. Skipping build."
        fi
    fi
}

# Start the service
start_service() {
    log_info "Starting G3ICAP service..."
    
    local binary_path="${SCRIPT_DIR}/target/release/g3icap"
    local config_path="$CONFIG_FILE"
    
    # Check if binary exists
    if [[ ! -f "$binary_path" ]]; then
        log_error "G3ICAP binary not found: $binary_path"
        log_error "Please build the project first: cargo build --release"
        exit 1
    fi
    
    # Check if config exists
    if [[ ! -f "$config_path" ]]; then
        log_error "Configuration file not found: $config_path"
        exit 1
    fi
    
    log_info "Starting G3ICAP with configuration: $config_path"
    log_info "YARA rules directory: $RULES_DIR"
    log_info "Quarantine directory: $QUARANTINE_DIR"
    log_info "Log directory: $LOG_DIR"
    
    # Start the service
    exec "$binary_path" --config "$config_path" --daemon
}

# Stop the service
stop_service() {
    log_info "Stopping G3ICAP service..."
    
    # Find and kill the process
    local pid=$(pgrep -f "g3icap.*--config.*$CONFIG_FILE" || true)
    
    if [[ -n "$pid" ]]; then
        kill "$pid"
        log_info "G3ICAP service stopped (PID: $pid)"
    else
        log_warn "G3ICAP service not running"
    fi
}

# Show service status
show_status() {
    log_info "G3ICAP Service Status"
    log_info "===================="
    
    local pid=$(pgrep -f "g3icap.*--config.*$CONFIG_FILE" || true)
    
    if [[ -n "$pid" ]]; then
        log_info "Status: Running (PID: $pid)"
        log_info "Configuration: $CONFIG_FILE"
        log_info "YARA Rules: $RULES_DIR"
        log_info "Quarantine: $QUARANTINE_DIR"
        log_info "Logs: $LOG_DIR"
    else
        log_warn "Status: Not running"
    fi
}

# Show help
show_help() {
    echo "G3ICAP Startup Script with YARA Support"
    echo "======================================="
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  start     Start the G3ICAP service"
    echo "  stop      Stop the G3ICAP service"
    echo "  restart   Restart the G3ICAP service"
    echo "  status    Show service status"
    echo "  setup     Setup directories and rules"
    echo "  help      Show this help message"
    echo ""
    echo "Configuration:"
    echo "  Config file: $CONFIG_FILE"
    echo "  YARA rules: $RULES_DIR"
    echo "  Quarantine: $QUARANTINE_DIR"
    echo "  Logs: $LOG_DIR"
}

# Main function
main() {
    case "${1:-start}" in
        start)
            check_root
            create_directories
            setup_yara_rules
            update_config
            check_dependencies
            start_service
            ;;
        stop)
            stop_service
            ;;
        restart)
            stop_service
            sleep 2
            check_root
            create_directories
            setup_yara_rules
            update_config
            check_dependencies
            start_service
            ;;
        status)
            show_status
            ;;
        setup)
            check_root
            create_directories
            setup_yara_rules
            update_config
            log_info "Setup completed successfully"
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            log_error "Unknown command: $1"
            show_help
            exit 1
            ;;
    esac
}

# Run main function
main "$@"
