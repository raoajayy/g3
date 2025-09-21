# G3 Services Management Scripts

This directory contains scripts to manage all G3 services from a single command.

## Scripts Overview

### 1. `start-services.sh` - Full Service Management
Comprehensive script with full control over all services.

**Usage:**
```bash
./start-services.sh [COMMAND]
```

**Commands:**
- `start` - Start all services (G3Proxy, G3StatsD, InfluxDB, Admin Console)
- `stop` - Stop all services
- `restart` - Restart all services
- `status` - Show status of all services
- `monitoring` - Start monitoring stack (InfluxDB + Grafana)
- `logs` - Show logs for all services
- `help` - Show help message

**Examples:**
```bash
./start-services.sh start      # Start all services
./start-services.sh stop       # Stop all services
./start-services.sh restart    # Restart all services
./start-services.sh status     # Check service status
./start-services.sh monitoring # Start Grafana monitoring
```

### 2. `quick-start.sh` - Simple Operations
Simplified script for quick common operations.

**Usage:**
```bash
./quick-start.sh [COMMAND]
```

**Commands:**
- `start` - Start all services (default)
- `stop` - Stop all services
- `status` - Show service status

**Examples:**
```bash
./quick-start.sh          # Start all services
./quick-start.sh start    # Start all services
./quick-start.sh stop     # Stop all services
./quick-start.sh status   # Check service status
```

## Services Managed

### G3Proxy
- **Config**: `config/g3proxy_monitoring.yaml`
- **Port**: 3129 (HTTP), 1081 (SOCKS)
- **Metrics**: StatsD on 127.0.0.1:8125

### G3StatsD
- **Config**: `config/g3statsd_influxdb_config.yaml`
- **Port**: 8125 (UDP)
- **Output**: InfluxDB v3

### InfluxDB v3
- **Config**: `docker-compose-influxdb.yml`
- **Port**: 8181
- **Database**: g3proxy

### Admin Console
- **Directory**: `admin-console/`
- **Port**: 3002
- **Command**: `npm run dev`

### Grafana (Optional)
- **Config**: `docker-compose-monitoring.yml`
- **Port**: 3001
- **Command**: `./start-services.sh monitoring`

## Prerequisites

1. **Build the project:**
   ```bash
   cargo build
   ```

2. **Install Node.js** (for admin console):
   ```bash
   # Install Node.js and npm
   ```

3. **Install Docker** (for InfluxDB):
   ```bash
   # Install Docker and Docker Compose
   ```

4. **Install admin console dependencies:**
   ```bash
   cd admin-console
   npm install
   cd ..
   ```

## Quick Start Guide

1. **First time setup:**
   ```bash
   # Build the project
   cargo build
   
   # Install admin console dependencies
   cd admin-console && npm install && cd ..
   
   # Start all services
   ./quick-start.sh
   ```

2. **Daily usage:**
   ```bash
   # Start services
   ./quick-start.sh start
   
   # Check status
   ./quick-start.sh status
   
   # Stop services
   ./quick-start.sh stop
   ```

3. **Advanced usage:**
   ```bash
   # Full service management
   ./start-services.sh start
   ./start-services.sh monitoring  # Add Grafana
   ./start-services.sh status
   ./start-services.sh logs
   ./start-services.sh stop
   ```

## Service URLs

- **Admin Console**: http://localhost:3002
- **InfluxDB**: http://localhost:8181
- **Grafana**: http://localhost:3001 (when monitoring is started)
- **G3Proxy HTTP**: http://127.0.0.1:3129
- **G3Proxy SOCKS**: 127.0.0.1:1081

## Troubleshooting

### Services not starting
1. Check if Docker is running: `docker info`
2. Check if project is built: `ls target/debug/g3proxy`
3. Check if admin console dependencies are installed: `cd admin-console && npm list`

### Port conflicts
- G3Proxy HTTP: 3129
- G3Proxy SOCKS: 1081
- Admin Console: 3002
- InfluxDB: 8181
- Grafana: 3001

### Check service status
```bash
./quick-start.sh status
```

### View logs
```bash
./start-services.sh logs
```

### Reset everything
```bash
./quick-start.sh stop
./quick-start.sh start
```

## Configuration Files

All configuration files are in the `config/` directory:
- `g3proxy_monitoring.yaml` - G3Proxy configuration
- `g3statsd_influxdb_config.yaml` - G3StatsD configuration
- `g3statsd_real_config.yaml` - Alternative G3StatsD configuration

## Notes

- The scripts automatically handle PID management for G3Proxy and G3StatsD
- Docker containers are managed via docker-compose
- All services are started in the background
- Use `./start-services.sh help` for detailed help
