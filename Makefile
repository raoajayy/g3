# G3 Services Management Makefile

.PHONY: help start stop restart status monitoring logs clean build install

# Default target
help:
	@echo "G3 Services Management"
	@echo "===================="
	@echo ""
	@echo "Available targets:"
	@echo "  start      - Start all services"
	@echo "  stop       - Stop all services"
	@echo "  restart    - Restart all services"
	@echo "  status     - Show service status"
	@echo "  monitoring - Start monitoring stack (Grafana)"
	@echo "  logs       - Show service logs"
	@echo "  build      - Build the project"
	@echo "  install    - Install admin console dependencies"
	@echo "  clean      - Clean up and stop all services"
	@echo "  help       - Show this help message"
	@echo ""
	@echo "Examples:"
	@echo "  make start     # Start all services"
	@echo "  make status    # Check service status"
	@echo "  make stop      # Stop all services"

# Start all services
start:
	@echo "Starting all G3 services..."
	@./quick-start.sh start

# Stop all services
stop:
	@echo "Stopping all G3 services..."
	@./quick-start.sh stop

# Restart all services
restart: stop start
	@echo "Services restarted"

# Show service status
status:
	@./quick-start.sh status

# Start monitoring stack
monitoring:
	@echo "Starting monitoring stack..."
	@./start-services.sh monitoring

# Show service logs
logs:
	@./start-services.sh logs

# Build the project
build:
	@echo "Building G3 project..."
	@cargo build

# Install admin console dependencies
install:
	@echo "Installing admin console dependencies..."
	@cd admin-console && npm install

# Clean up and stop all services
clean: stop
	@echo "Cleaning up..."
	@docker-compose -f docker-compose-influxdb.yml down 2>/dev/null || true
	@docker-compose -f docker-compose-monitoring.yml down 2>/dev/null || true
	@echo "Cleanup complete"

# Full setup (build + install + start)
setup: build install start
	@echo "Full setup complete!"
	@echo "Admin Console: http://localhost:3002"
	@echo "InfluxDB: http://localhost:8181"
	@echo "G3Proxy: http://127.0.0.1:3129"
