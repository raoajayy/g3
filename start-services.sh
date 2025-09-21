#!/bin/bash

# G3 Services Management Script
# This script manages G3Proxy, G3StatsD, G3ICAP, InfluxDB, and Admin Console

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
G3PROXY_CONFIG="config/g3proxy_monitoring.yaml"
G3PROXY_ICAP_CONFIG="config/g3proxy_with_icap.yaml"
G3PROXY_ICAP_ADVANCED_CONFIG="config/g3proxy_icap_advanced.yaml"
G3STATSD_CONFIG="config/g3statsd_influxdb_config.yaml"
G3ICAP_CONFIG="config/g3icap/g3icap.yaml"
G3ICAP_YARA_CONFIG="config/g3icap/yara_config.yaml"
G3ICAP_YARA_RULES="config/g3icap/yara_rules/malware_detection.yar"
INFLUXDB_COMPOSE="docker-compose-influxdb.yml"
MONITORING_COMPOSE="docker-compose-monitoring.yml"
ADMIN_CONSOLE_DIR="admin-console"

# PID files
G3PROXY_PID="/tmp/g3proxy.pid"
G3STATSD_PID="/tmp/g3statsd.pid"
G3ICAP_PID="/tmp/g3icap.pid"
G3FCGEN_PID="/tmp/g3fcgen.pid"
PYTHON_PROXY_PID="/tmp/python_proxy.pid"

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_dependencies() {
    log_info "Checking dependencies..."
    
    # Check if required binaries exist
    if [ ! -f "./target/debug/g3proxy" ]; then
        log_error "G3Proxy binary not found. Please build the project first with 'cargo build'"
        exit 1
    fi
    
    if [ ! -f "./target/debug/g3statsd" ]; then
        log_error "G3StatsD binary not found. Please build the project first with 'cargo build'"
        exit 1
    fi
    
    if [ ! -f "./g3icap/target/release/g3icap" ] && [ ! -f "./g3icap/target/debug/g3icap" ]; then
        log_warning "G3ICAP binary not found. Building G3ICAP..."
        cd g3icap
        if command -v cargo > /dev/null 2>&1; then
            cargo build --release 2>/dev/null || cargo build 2>/dev/null || {
                log_error "Failed to build G3ICAP. Please build manually with 'cd g3icap && cargo build'"
                exit 1
            }
        else
            log_error "Cargo not available. Please build G3ICAP manually"
            exit 1
        fi
        cd ..
    fi
    
    # Check if Docker is running
    if ! docker info > /dev/null 2>&1; then
        log_error "Docker is not running. Please start Docker first"
        exit 1
    fi
    
    # Check if Node.js is available for admin console
    if ! command -v node > /dev/null 2>&1; then
        log_warning "Node.js not found. Admin console will not be started"
    fi
    
    log_success "Dependencies check passed"
}

stop_services() {
    log_info "Stopping all services..."
    
    # Stop Python proxy server
    if [ -f "$PYTHON_PROXY_PID" ]; then
        PID=$(cat "$PYTHON_PROXY_PID")
        if kill -0 "$PID" 2>/dev/null; then
            log_info "Stopping Python proxy (PID: $PID)..."
            kill "$PID" 2>/dev/null || true
            sleep 2
            kill -9 "$PID" 2>/dev/null || true
            rm -f "$PYTHON_PROXY_PID"
            log_success "Python proxy stopped"
        else
            log_warning "Python proxy process not running"
            rm -f "$PYTHON_PROXY_PID"
        fi
    else
        # Try to kill by process name
        pkill -f "influxdb-proxy.py" 2>/dev/null || true
        log_info "Python proxy stopped (if running)"
    fi
    
    # Stop G3FCGen
    if [ -f "$G3FCGEN_PID" ]; then
        PID=$(cat "$G3FCGEN_PID")
        if kill -0 "$PID" 2>/dev/null; then
            log_info "Stopping G3FCGen (PID: $PID)..."
            kill "$PID" 2>/dev/null || true
            sleep 2
            kill -9 "$PID" 2>/dev/null || true
            rm -f "$G3FCGEN_PID"
            log_success "G3FCGen stopped"
        else
            log_warning "G3FCGen process not running"
            rm -f "$G3FCGEN_PID"
        fi
    else
        # Try to kill by process name
        pkill -f "g3fcgen" 2>/dev/null || true
        log_info "G3FCGen stopped (if running)"
    fi
    
    # Stop G3Proxy
    if [ -f "$G3PROXY_PID" ]; then
        PID=$(cat "$G3PROXY_PID")
        if kill -0 "$PID" 2>/dev/null; then
            log_info "Stopping G3Proxy (PID: $PID)..."
            kill "$PID" 2>/dev/null || true
            sleep 2
            kill -9 "$PID" 2>/dev/null || true
            rm -f "$G3PROXY_PID"
            log_success "G3Proxy stopped"
        else
            log_warning "G3Proxy process not running"
            rm -f "$G3PROXY_PID"
        fi
    else
        # Try to kill by process name
        pkill -f "g3proxy" 2>/dev/null || true
        log_info "G3Proxy stopped (if running)"
    fi
    
    # Stop G3StatsD
    if [ -f "$G3STATSD_PID" ]; then
        PID=$(cat "$G3STATSD_PID")
        if kill -0 "$PID" 2>/dev/null; then
            log_info "Stopping G3StatsD (PID: $PID)..."
            kill "$PID" 2>/dev/null || true
            sleep 2
            kill -9 "$PID" 2>/dev/null || true
            rm -f "$G3STATSD_PID"
            log_success "G3StatsD stopped"
        else
            log_warning "G3StatsD process not running"
            rm -f "$G3STATSD_PID"
        fi
    else
        # Try to kill by process name
        pkill -f "g3statsd" 2>/dev/null || true
        log_info "G3StatsD stopped (if running)"
    fi
    
    # Stop G3ICAP
    if [ -f "$G3ICAP_PID" ]; then
        PID=$(cat "$G3ICAP_PID")
        if kill -0 "$PID" 2>/dev/null; then
            log_info "Stopping G3ICAP (PID: $PID)..."
            kill "$PID" 2>/dev/null || true
            sleep 2
            kill -9 "$PID" 2>/dev/null || true
            rm -f "$G3ICAP_PID"
            log_success "G3ICAP stopped"
        else
            log_warning "G3ICAP process not running"
            rm -f "$G3ICAP_PID"
        fi
    else
        # Try to kill by process name
        pkill -f "g3icap" 2>/dev/null || true
        log_info "G3ICAP stopped (if running)"
    fi
    
    # Stop InfluxDB
    log_info "Stopping InfluxDB..."
    docker-compose -f "$INFLUXDB_COMPOSE" down 2>/dev/null || true
    log_success "InfluxDB stopped"
    
    # Stop monitoring stack
    log_info "Stopping monitoring stack..."
    docker-compose -f "$MONITORING_COMPOSE" down 2>/dev/null || true
    log_success "Monitoring stack stopped"
    
    # Stop admin console (if running)
    pkill -f "npm run dev" 2>/dev/null || true
    log_info "Admin console stopped (if running)"
    
    # Clean up any remaining processes
    pkill -f "python3 influxdb-proxy.py" 2>/dev/null || true
    
    log_success "All services stopped"
}

start_services() {
    log_info "Starting all services..."
    
    # Start InfluxDB
    log_info "Starting InfluxDB..."
    docker-compose -f "$INFLUXDB_COMPOSE" up -d
    sleep 5
    log_success "InfluxDB started"
    
    # Start G3StatsD
    log_info "Starting G3StatsD..."
    ./target/debug/g3statsd -c "$G3STATSD_CONFIG" &
    G3STATSD_PID_VAL=$!
    echo $G3STATSD_PID_VAL > "$G3STATSD_PID"
    sleep 2
    log_success "G3StatsD started (PID: $G3STATSD_PID_VAL)"
    
    # Start G3ICAP
    log_info "Starting G3ICAP..."
    # Determine which binary to use
    G3ICAP_BINARY=""
    if [ -f "./g3icap/target/release/g3icap" ]; then
        G3ICAP_BINARY="./g3icap/target/release/g3icap"
    elif [ -f "./g3icap/target/debug/g3icap" ]; then
        G3ICAP_BINARY="./g3icap/target/debug/g3icap"
    else
        log_error "G3ICAP binary not found"
        exit 1
    fi
    
    # Setup G3ICAP directories
    log_info "Setting up G3ICAP directories..."
    mkdir -p config/g3icap/logs
    mkdir -p config/g3icap/quarantine
    mkdir -p config/g3icap/yara_rules
    
    # Use the YARA configuration if available, otherwise use basic config
    if [ -f "$G3ICAP_YARA_CONFIG" ]; then
        G3ICAP_CONFIG_TO_USE="$G3ICAP_YARA_CONFIG"
        log_info "Using YARA antivirus configuration"
    else
        G3ICAP_CONFIG_TO_USE="$G3ICAP_CONFIG"
        log_info "Using basic configuration"
    fi
    
    # Start G3ICAP
    $G3ICAP_BINARY --config "$G3ICAP_CONFIG_TO_USE" &
    G3ICAP_PID_VAL=$!
    echo $G3ICAP_PID_VAL > "$G3ICAP_PID"
    sleep 3
    log_success "G3ICAP started (PID: $G3ICAP_PID_VAL)"
    log_info "G3ICAP available at: http://localhost:1344"
    
    # Start G3FCGen (Fake Certificate Generator)
    log_info "Starting G3FCGen..."
    if [ -f "config/ajay.key" ] && [ -f "config/ajay.crt" ]; then
        ./target/debug/g3fcgen -v --config-file config/g3fcgen.yaml &
        G3FCGEN_PID_VAL=$!
        echo $G3FCGEN_PID_VAL > "$G3FCGEN_PID"
        sleep 2
        log_success "G3FCGen started (PID: $G3FCGEN_PID_VAL)"
    else
        log_warning "G3FCGen certificates not found, creating them..."
        openssl req -x509 -newkey rsa:4096 -keyout config/ajay.key -out config/ajay.crt -days 365 -nodes -subj "/C=US/ST=State/L=City/O=Organization/CN=G3FCGen" 2>/dev/null
        ./target/debug/g3fcgen -v --config-file config/g3fcgen.yaml &
        G3FCGEN_PID_VAL=$!
        echo $G3FCGEN_PID_VAL > "$G3FCGEN_PID"
        sleep 2
        log_success "G3FCGen started (PID: $G3FCGEN_PID_VAL)"
    fi
    
    # Start G3Proxy with ICAP integration
    log_info "Starting G3Proxy with ICAP integration..."
    if [ -f "$G3PROXY_ICAP_CONFIG" ]; then
        ./target/debug/g3proxy --config-file "$G3PROXY_ICAP_CONFIG" -G g3proxy &
        log_info "Using ICAP-enabled G3Proxy configuration"
    else
        ./target/debug/g3proxy --config-file "$G3PROXY_CONFIG" -G g3proxy &
        log_warning "Using basic G3Proxy configuration (ICAP not enabled)"
    fi
    G3PROXY_PID_VAL=$!
    echo $G3PROXY_PID_VAL > "$G3PROXY_PID"
    sleep 2
    log_success "G3Proxy started (PID: $G3PROXY_PID_VAL)"
    log_info "G3Proxy available at: http://127.0.0.1:3129 (HTTP) and http://127.0.0.1:3128 (HTTPS)"
    
    # Start Python proxy server for InfluxDB v3 queries
    log_info "Starting Python proxy server..."
    python3 influxdb-proxy.py &
    PYTHON_PROXY_PID_VAL=$!
    echo $PYTHON_PROXY_PID_VAL > "$PYTHON_PROXY_PID"
    sleep 2
    log_success "Python proxy started (PID: $PYTHON_PROXY_PID_VAL)"
    
    # Start admin console
    if command -v node > /dev/null 2>&1; then
        log_info "Starting admin console..."
        cd "$ADMIN_CONSOLE_DIR"
        npm run dev &
        cd ..
        log_success "Admin console started"
    else
        log_warning "Skipping admin console (Node.js not available)"
    fi
    
    log_success "All services started successfully!"
    show_status
}

start_icap_only() {
    log_info "Starting G3ICAP only..."
    
    # Check dependencies
    check_dependencies
    
    # Stop any existing G3ICAP
    if [ -f "$G3ICAP_PID" ]; then
        PID=$(cat "$G3ICAP_PID")
        if kill -0 "$PID" 2>/dev/null; then
            log_info "Stopping existing G3ICAP (PID: $PID)..."
            kill "$PID" 2>/dev/null || true
            sleep 2
            kill -9 "$PID" 2>/dev/null || true
            rm -f "$G3ICAP_PID"
        fi
    fi
    
    # Determine which binary to use
    G3ICAP_BINARY=""
    if [ -f "./g3icap/target/release/g3icap" ]; then
        G3ICAP_BINARY="./g3icap/target/release/g3icap"
    elif [ -f "./g3icap/target/debug/g3icap" ]; then
        G3ICAP_BINARY="./g3icap/target/debug/g3icap"
    else
        log_error "G3ICAP binary not found"
        exit 1
    fi
    
    # Setup G3ICAP directories
    log_info "Setting up G3ICAP directories..."
    mkdir -p config/g3icap/logs
    mkdir -p config/g3icap/quarantine
    mkdir -p config/g3icap/yara_rules
    
    # Use the YARA configuration if available, otherwise use basic config
    if [ -f "$G3ICAP_YARA_CONFIG" ]; then
        G3ICAP_CONFIG_TO_USE="$G3ICAP_YARA_CONFIG"
        log_info "Using YARA antivirus configuration"
    else
        G3ICAP_CONFIG_TO_USE="$G3ICAP_CONFIG"
        log_info "Using basic configuration"
    fi
    
    # Start G3ICAP
    log_info "Starting G3ICAP with YARA antivirus support..."
    $G3ICAP_BINARY --config "$G3ICAP_CONFIG_TO_USE" &
    G3ICAP_PID_VAL=$!
    echo $G3ICAP_PID_VAL > "$G3ICAP_PID"
    sleep 3
    
    if kill -0 "$G3ICAP_PID_VAL" 2>/dev/null; then
        log_success "G3ICAP started successfully (PID: $G3ICAP_PID_VAL)"
        log_info "G3ICAP available at: http://localhost:1344"
        log_info "YARA rules directory: config/g3icap/yara_rules"
        log_info "Quarantine directory: config/g3icap/quarantine"
        log_info "Log directory: config/g3icap/logs"
        log_info "Press Ctrl+C to stop G3ICAP"
        
        # Wait for user to stop
        trap 'log_info "Stopping G3ICAP..."; kill $G3ICAP_PID_VAL 2>/dev/null; rm -f "$G3ICAP_PID"; exit 0' INT
        wait $G3ICAP_PID_VAL
    else
        log_error "Failed to start G3ICAP"
        exit 1
    fi
}

start_proxy_only() {
    log_info "Starting G3Proxy with ICAP integration only..."
    
    # Check dependencies
    check_dependencies
    
    # Stop any existing G3Proxy
    if [ -f "$G3PROXY_PID" ]; then
        PID=$(cat "$G3PROXY_PID")
        if kill -0 "$PID" 2>/dev/null; then
            log_info "Stopping existing G3Proxy (PID: $PID)..."
            kill "$PID" 2>/dev/null || true
            sleep 2
            kill -9 "$PID" 2>/dev/null || true
            rm -f "$G3PROXY_PID"
        fi
    fi
    
    # Start G3Proxy with ICAP integration
    log_info "Starting G3Proxy with ICAP integration..."
    if [ -f "$G3PROXY_ICAP_CONFIG" ]; then
        ./target/debug/g3proxy --config-file "$G3PROXY_ICAP_CONFIG" -G g3proxy &
        log_info "Using ICAP-enabled G3Proxy configuration"
    else
        ./target/debug/g3proxy --config-file "$G3PROXY_CONFIG" -G g3proxy &
        log_warning "Using basic G3Proxy configuration (ICAP not enabled)"
    fi
    G3PROXY_PID_VAL=$!
    echo $G3PROXY_PID_VAL > "$G3PROXY_PID"
    sleep 3
    
    if kill -0 "$G3PROXY_PID_VAL" 2>/dev/null; then
        log_success "G3Proxy started successfully (PID: $G3PROXY_PID_VAL)"
        log_info "G3Proxy available at:"
        log_info "  HTTP:  http://127.0.0.1:3129"
        log_info "  HTTPS: http://127.0.0.1:3128"
        log_info "  SOCKS: socks5://127.0.0.1:1081"
        log_info "ICAP integration: icap://127.0.0.1:1344/"
        log_info "Press Ctrl+C to stop G3Proxy"
        
        # Wait for user to stop
        trap 'log_info "Stopping G3Proxy..."; kill $G3PROXY_PID_VAL 2>/dev/null; rm -f "$G3PROXY_PID"; exit 0' INT
        wait $G3PROXY_PID_VAL
    else
        log_error "Failed to start G3Proxy"
        exit 1
    fi
}

start_icap_advanced() {
    log_info "Starting all services with advanced ICAP configuration..."
    
    # Check dependencies
    check_dependencies
    
    # Stop existing services
    stop_services
    
    # Start InfluxDB
    log_info "Starting InfluxDB..."
    docker-compose -f "$INFLUXDB_COMPOSE" up -d
    sleep 5
    log_success "InfluxDB started"
    
    # Start G3StatsD
    log_info "Starting G3StatsD..."
    ./target/debug/g3statsd -c "$G3STATSD_CONFIG" &
    G3STATSD_PID_VAL=$!
    echo $G3STATSD_PID_VAL > "$G3STATSD_PID"
    sleep 2
    log_success "G3StatsD started (PID: $G3STATSD_PID_VAL)"
    
    # Start G3ICAP with YARA
    log_info "Starting G3ICAP with YARA antivirus..."
    G3ICAP_BINARY=""
    if [ -f "./g3icap/target/release/g3icap" ]; then
        G3ICAP_BINARY="./g3icap/target/release/g3icap"
    elif [ -f "./g3icap/target/debug/g3icap" ]; then
        G3ICAP_BINARY="./g3icap/target/debug/g3icap"
    else
        log_error "G3ICAP binary not found"
        exit 1
    fi
    
    # Setup G3ICAP directories
    mkdir -p config/g3icap/logs
    mkdir -p config/g3icap/quarantine
    mkdir -p config/g3icap/yara_rules
    
    # Use YARA configuration
    $G3ICAP_BINARY --config "$G3ICAP_YARA_CONFIG" &
    G3ICAP_PID_VAL=$!
    echo $G3ICAP_PID_VAL > "$G3ICAP_PID"
    sleep 3
    log_success "G3ICAP started (PID: $G3ICAP_PID_VAL)"
    
    # Start G3Proxy with advanced ICAP configuration
    log_info "Starting G3Proxy with advanced ICAP configuration..."
    if [ -f "$G3PROXY_ICAP_ADVANCED_CONFIG" ]; then
        ./target/debug/g3proxy --config-file "$G3PROXY_ICAP_ADVANCED_CONFIG" -G g3proxy &
        log_info "Using advanced ICAP configuration"
    elif [ -f "$G3PROXY_ICAP_CONFIG" ]; then
        ./target/debug/g3proxy --config-file "$G3PROXY_ICAP_CONFIG" -G g3proxy &
        log_info "Using basic ICAP configuration"
    else
        ./target/debug/g3proxy --config-file "$G3PROXY_CONFIG" -G g3proxy &
        log_warning "Using basic configuration (ICAP not enabled)"
    fi
    G3PROXY_PID_VAL=$!
    echo $G3PROXY_PID_VAL > "$G3PROXY_PID"
    sleep 2
    log_success "G3Proxy started (PID: $G3PROXY_PID_VAL)"
    
    # Start admin console
    if command -v node > /dev/null 2>&1; then
        log_info "Starting admin console..."
        cd "$ADMIN_CONSOLE_DIR"
        npm run dev &
        cd ..
        log_success "Admin console started"
    else
        log_warning "Skipping admin console (Node.js not available)"
    fi
    
    log_success "All services started with advanced ICAP configuration!"
    show_status
}

start_monitoring() {
    log_info "Starting monitoring stack..."
    docker-compose -f "$MONITORING_COMPOSE" up -d
    sleep 5
    log_success "Monitoring stack started"
    log_info "Grafana available at: http://localhost:3001"
}

show_status() {
    log_info "Service Status:"
    echo "=================="
    
    # Check G3Proxy
    if [ -f "$G3PROXY_PID" ]; then
        PID=$(cat "$G3PROXY_PID")
        if kill -0 "$PID" 2>/dev/null; then
            echo -e "G3Proxy: ${GREEN}Running${NC} (PID: $PID)"
        else
            echo -e "G3Proxy: ${RED}Not Running${NC}"
        fi
    else
        echo -e "G3Proxy: ${RED}Not Running${NC}"
    fi
    
    # Check G3StatsD
    if [ -f "$G3STATSD_PID" ]; then
        PID=$(cat "$G3STATSD_PID")
        if kill -0 "$PID" 2>/dev/null; then
            echo -e "G3StatsD: ${GREEN}Running${NC} (PID: $PID)"
        else
            echo -e "G3StatsD: ${RED}Not Running${NC}"
        fi
    else
        echo -e "G3StatsD: ${RED}Not Running${NC}"
    fi
    
    # Check G3ICAP
    if [ -f "$G3ICAP_PID" ]; then
        PID=$(cat "$G3ICAP_PID")
        if kill -0 "$PID" 2>/dev/null; then
            echo -e "G3ICAP: ${GREEN}Running${NC} (PID: $PID) - http://localhost:1344"
        else
            echo -e "G3ICAP: ${RED}Not Running${NC}"
        fi
    else
        echo -e "G3ICAP: ${RED}Not Running${NC}"
    fi
    
    # Check G3FCGen
    if [ -f "$G3FCGEN_PID" ]; then
        PID=$(cat "$G3FCGEN_PID")
        if kill -0 "$PID" 2>/dev/null; then
            echo -e "G3FCGen: ${GREEN}Running${NC} (PID: $PID)"
        else
            echo -e "G3FCGen: ${RED}Not Running${NC}"
        fi
    else
        echo -e "G3FCGen: ${RED}Not Running${NC}"
    fi
    
    # Check Python Proxy
    if [ -f "$PYTHON_PROXY_PID" ]; then
        PID=$(cat "$PYTHON_PROXY_PID")
        if kill -0 "$PID" 2>/dev/null; then
            echo -e "Python Proxy: ${GREEN}Running${NC} (PID: $PID)"
        else
            echo -e "Python Proxy: ${RED}Not Running${NC}"
        fi
    else
        echo -e "Python Proxy: ${RED}Not Running${NC}"
    fi
    
    # Check InfluxDB
    if docker ps | grep -q influxdb3; then
        echo -e "InfluxDB: ${GREEN}Running${NC}"
    else
        echo -e "InfluxDB: ${RED}Not Running${NC}"
    fi
    
    # Check Admin Console
    if pgrep -f "npm run dev" > /dev/null; then
        echo -e "Admin Console: ${GREEN}Running${NC} (http://localhost:3002)"
    else
        echo -e "Admin Console: ${RED}Not Running${NC}"
    fi
    
    echo "=================="
}

show_help() {
    echo "G3 Services Management Script"
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  start       Start all services (G3Proxy, G3StatsD, G3ICAP, InfluxDB, Admin Console)"
    echo "  stop        Stop all services"
    echo "  restart     Restart all services"
    echo "  status      Show status of all services"
    echo "  monitoring  Start monitoring stack (InfluxDB + Grafana)"
    echo "  logs        Show logs for all services"
    echo "  icap-only   Start only G3ICAP service"
    echo "  proxy-only  Start only G3Proxy with ICAP integration"
    echo "  icap-advanced Start all services with advanced ICAP configuration"
    echo "  help        Show this help message"
    echo ""
    echo "Services:"
    echo "  G3Proxy     - HTTP/HTTPS proxy server with ICAP integration"
    echo "  G3StatsD    - Statistics collection server"
    echo "  G3ICAP      - ICAP content adaptation server with YARA antivirus"
    echo "  G3FCGen     - Fake certificate generator"
    echo "  InfluxDB    - Time series database"
    echo "  Admin Console - Web-based management interface"
    echo ""
    echo "Configuration Files:"
    echo "  Basic G3Proxy:     config/g3proxy_monitoring.yaml"
    echo "  ICAP G3Proxy:      config/g3proxy_with_icap.yaml"
    echo "  Advanced ICAP:     config/g3proxy_icap_advanced.yaml"
    echo "  Basic G3ICAP:      config/g3icap/g3icap.yaml"
    echo "  YARA G3ICAP:       config/g3icap/yara_config.yaml"
    echo ""
    echo "Examples:"
    echo "  $0 start           # Start all services with ICAP"
    echo "  $0 stop            # Stop all services"
    echo "  $0 restart         # Restart all services"
    echo "  $0 status          # Check service status"
    echo "  $0 icap-only       # Start only G3ICAP"
    echo "  $0 proxy-only      # Start only G3Proxy with ICAP"
    echo "  $0 icap-advanced   # Start with advanced ICAP config"
}

show_logs() {
    log_info "Showing logs for all services..."
    echo "Press Ctrl+C to exit"
    echo ""
    
    # Show G3Proxy logs
    if [ -f "$G3PROXY_PID" ]; then
        PID=$(cat "$G3PROXY_PID")
        if kill -0 "$PID" 2>/dev/null; then
            echo "=== G3Proxy Logs ==="
            tail -f /dev/null &  # Placeholder for G3Proxy logs
        fi
    fi
    
    # Show G3StatsD logs
    if [ -f "$G3STATSD_PID" ]; then
        PID=$(cat "$G3STATSD_PID")
        if kill -0 "$PID" 2>/dev/null; then
            echo "=== G3StatsD Logs ==="
            tail -f /dev/null &  # Placeholder for G3StatsD logs
        fi
    fi
    
    # Show InfluxDB logs
    echo "=== InfluxDB Logs ==="
    docker logs -f influxdb3
}

# Main script logic
case "${1:-}" in
    start)
        check_dependencies
        stop_services
        start_services
        ;;
    stop)
        stop_services
        ;;
    restart)
        check_dependencies
        stop_services
        start_services
        ;;
    status)
        show_status
        ;;
    monitoring)
        start_monitoring
        ;;
    logs)
        show_logs
        ;;
    icap-only)
        start_icap_only
        ;;
    proxy-only)
        start_proxy_only
        ;;
    icap-advanced)
        start_icap_advanced
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        log_error "Unknown command: ${1:-}"
        echo ""
        show_help
        exit 1
        ;;
esac
