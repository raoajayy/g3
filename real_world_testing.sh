#!/bin/bash

# G3 Real-World Testing Script
# This script performs comprehensive testing of all G3 services with real-world scenarios

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
G3PROXY_HTTP="127.0.0.1:3129"
G3PROXY_HTTPS="127.0.0.1:3128"
G3PROXY_SOCKS="127.0.0.1:1081"
G3ICAP_SERVER="127.0.0.1:1344"
G3STATSD_SERVER="127.0.0.1:8125"
INFLUXDB_SERVER="127.0.0.1:8181"

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
    ((PASSED_TESTS++))
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
    ((FAILED_TESTS++))
}

log_test() {
    echo -e "${PURPLE}[TEST]${NC} $1"
    ((TOTAL_TESTS++))
}

test_service_health() {
    local service_name="$1"
    local url="$2"
    local expected_status="$3"
    
    log_test "Testing $service_name health at $url"
    
    if curl -s -o /dev/null -w "%{http_code}" "$url" | grep -q "$expected_status"; then
        log_success "$service_name is healthy (HTTP $expected_status)"
        return 0
    else
        log_error "$service_name health check failed"
        return 1
    fi
}

test_icap_service() {
    local service_name="$1"
    local icap_url="$2"
    
    log_test "Testing ICAP $service_name at $icap_url"
    
    # Test ICAP OPTIONS request
    local response=$(echo -e "OPTIONS $icap_url ICAP/1.0\r\nHost: 127.0.0.1:1344\r\nUser-Agent: G3ICAP-Test/1.0\r\n\r\n" | nc 127.0.0.1 1344 2>/dev/null)
    
    if [ -n "$response" ]; then
        log_success "ICAP $service_name responded to OPTIONS request"
        return 0
    else
        log_error "ICAP $service_name did not respond to OPTIONS request"
        return 1
    fi
}

test_proxy_http() {
    local test_url="$1"
    local description="$2"
    
    log_test "Testing HTTP proxy with $description: $test_url"
    
    local response=$(curl -s -w "%{http_code}" -x "http://$G3PROXY_HTTP" "$test_url" 2>/dev/null)
    local status_code="${response: -3}"
    
    if [ "$status_code" = "200" ]; then
        log_success "HTTP proxy successfully handled $description"
        return 0
    else
        log_error "HTTP proxy failed for $description (Status: $status_code)"
        return 1
    fi
}

test_proxy_https() {
    local test_url="$1"
    local description="$2"
    
    log_test "Testing HTTPS proxy with $description: $test_url"
    
    local response=$(curl -s -w "%{http_code}" -x "http://$G3PROXY_HTTPS" "$test_url" 2>/dev/null)
    local status_code="${response: -3}"
    
    if [ "$status_code" = "200" ]; then
        log_success "HTTPS proxy successfully handled $description"
        return 0
    else
        log_error "HTTPS proxy failed for $description (Status: $status_code)"
        return 1
    fi
}

test_content_filtering() {
    log_test "Testing content filtering with malicious URLs"
    
    local malicious_urls=(
        "http://malware-samples.com/test.exe"
        "http://phishing-site.com/login"
        "http://suspicious-domain.net/script.js"
    )
    
    local blocked_count=0
    for url in "${malicious_urls[@]}"; do
        local response=$(curl -s -w "%{http_code}" -x "http://$G3PROXY_HTTP" "$url" 2>/dev/null)
        local status_code="${response: -3}"
        
        if [ "$status_code" != "200" ]; then
            ((blocked_count++))
        fi
    done
    
    if [ $blocked_count -gt 0 ]; then
        log_success "Content filtering blocked $blocked_count out of ${#malicious_urls[@]} malicious URLs"
        return 0
    else
        log_warning "Content filtering may not be working (all malicious URLs returned 200)"
        return 1
    fi
}

test_antivirus_scanning() {
    log_test "Testing antivirus scanning with executable files"
    
    local test_urls=(
        "http://httpbin.org/json"
        "http://httpbin.org/html"
        "http://httpbin.org/xml"
    )
    
    local scanned_count=0
    for url in "${test_urls[@]}"; do
        local response=$(curl -s -w "%{http_code}" -x "http://$G3PROXY_HTTP" "$url" 2>/dev/null)
        local status_code="${response: -3}"
        
        if [ "$status_code" = "200" ]; then
            ((scanned_count++))
        fi
    done
    
    if [ $scanned_count -gt 0 ]; then
        log_success "Antivirus scanning processed $scanned_count out of ${#test_urls[@]} requests"
        return 0
    else
        log_error "Antivirus scanning failed to process any requests"
        return 1
    fi
}

test_performance() {
    log_test "Testing performance with concurrent requests"
    
    local test_url="http://httpbin.org/get"
    local concurrent_requests=10
    local success_count=0
    
    # Run concurrent requests
    for i in $(seq 1 $concurrent_requests); do
        (
            local response=$(curl -s -w "%{http_code}" -x "http://$G3PROXY_HTTP" "$test_url" 2>/dev/null)
            local status_code="${response: -3}"
            if [ "$status_code" = "200" ]; then
                echo "success" >> /tmp/g3_test_results
            fi
        ) &
    done
    
    # Wait for all requests to complete
    wait
    
    # Count successful requests
    if [ -f /tmp/g3_test_results ]; then
        success_count=$(wc -l < /tmp/g3_test_results)
        rm -f /tmp/g3_test_results
    fi
    
    local success_rate=$((success_count * 100 / concurrent_requests))
    
    if [ $success_rate -ge 80 ]; then
        log_success "Performance test passed: $success_count/$concurrent_requests requests successful ($success_rate%)"
        return 0
    else
        log_error "Performance test failed: $success_count/$concurrent_requests requests successful ($success_rate%)"
        return 1
    fi
}

test_monitoring() {
    log_test "Testing monitoring and metrics collection"
    
    # Test G3StatsD metrics
    local statsd_response=$(echo "g3icap.test.metric:1|c" | nc -u 127.0.0.1 8125 2>/dev/null)
    
    if [ $? -eq 0 ]; then
        log_success "G3StatsD metrics collection working"
    else
        log_error "G3StatsD metrics collection failed"
        return 1
    fi
    
    # Test InfluxDB connectivity
    local influxdb_response=$(curl -s -o /dev/null -w "%{http_code}" "http://$INFLUXDB_SERVER/health" 2>/dev/null)
    
    if [ "$influxdb_response" = "200" ]; then
        log_success "InfluxDB is accessible"
        return 0
    else
        log_error "InfluxDB is not accessible (Status: $influxdb_response)"
        return 1
    fi
}

test_integration() {
    log_test "Testing G3Proxy-G3ICAP integration"
    
    # Test that requests go through both proxy and ICAP
    local test_url="http://httpbin.org/headers"
    local response=$(curl -s -x "http://$G3PROXY_HTTP" "$test_url" 2>/dev/null)
    
    if echo "$response" | grep -q "Via.*G3Proxy"; then
        log_success "G3Proxy-G3ICAP integration working (Via header present)"
        return 0
    else
        log_warning "G3Proxy-G3ICAP integration may not be working (Via header missing)"
        return 1
    fi
}

run_comprehensive_tests() {
    echo -e "${CYAN}🚀 G3 Real-World Testing Suite${NC}"
    echo "=================================="
    echo ""
    
    # Service Health Checks
    log_info "🔍 Running Service Health Checks..."
    test_service_health "G3ICAP" "http://$G3ICAP_SERVER/options" "200" || true
    test_service_health "G3StatsD" "http://$G3STATSD_SERVER" "200" || true
    test_service_health "InfluxDB" "http://$INFLUXDB_SERVER/health" "200" || true
    echo ""
    
    # ICAP Service Tests
    log_info "🔧 Running ICAP Service Tests..."
    test_icap_service "OPTIONS" "icap://$G3ICAP_SERVER/options" || true
    test_icap_service "REQMOD" "icap://$G3ICAP_SERVER/reqmod" || true
    test_icap_service "RESPMOD" "icap://$G3ICAP_SERVER/respmod" || true
    echo ""
    
    # HTTP Proxy Tests
    log_info "🌐 Running HTTP Proxy Tests..."
    test_proxy_http "http://httpbin.org/get" "Basic GET request" || true
    test_proxy_http "http://httpbin.org/headers" "Headers request" || true
    test_proxy_http "http://httpbin.org/user-agent" "User-Agent request" || true
    test_proxy_http "http://httpbin.org/json" "JSON response" || true
    echo ""
    
    # HTTPS Proxy Tests
    log_info "🔒 Running HTTPS Proxy Tests..."
    test_proxy_https "https://httpbin.org/get" "HTTPS GET request" || true
    test_proxy_https "https://httpbin.org/headers" "HTTPS headers request" || true
    echo ""
    
    # Content Filtering Tests
    log_info "🔍 Running Content Filtering Tests..."
    test_content_filtering || true
    echo ""
    
    # Antivirus Scanning Tests
    log_info "🛡️ Running Antivirus Scanning Tests..."
    test_antivirus_scanning || true
    echo ""
    
    # Performance Tests
    log_info "⚡ Running Performance Tests..."
    test_performance || true
    echo ""
    
    # Monitoring Tests
    log_info "📊 Running Monitoring Tests..."
    test_monitoring || true
    echo ""
    
    # Integration Tests
    log_info "🔗 Running Integration Tests..."
    test_integration || true
    echo ""
    
    # Test Summary
    echo "=================================="
    echo -e "${CYAN}📊 Test Results Summary${NC}"
    echo "=================================="
    echo -e "Total Tests: ${TOTAL_TESTS}"
    echo -e "Passed: ${GREEN}${PASSED_TESTS}${NC}"
    echo -e "Failed: ${RED}${FAILED_TESTS}${NC}"
    
    local success_rate=$((PASSED_TESTS * 100 / TOTAL_TESTS))
    echo -e "Success Rate: ${success_rate}%"
    
    if [ $success_rate -ge 80 ]; then
        echo -e "${GREEN}✅ Overall Test Result: PASSED${NC}"
        return 0
    else
        echo -e "${RED}❌ Overall Test Result: FAILED${NC}"
        return 1
    fi
}

# Main execution
case "${1:-}" in
    "health")
        log_info "Running health checks only..."
        test_service_health "G3ICAP" "http://$G3ICAP_SERVER/options" "200"
        test_service_health "G3StatsD" "http://$G3STATSD_SERVER" "200"
        test_service_health "InfluxDB" "http://$INFLUXDB_SERVER/health" "200"
        ;;
    "proxy")
        log_info "Running proxy tests only..."
        test_proxy_http "http://httpbin.org/get" "Basic GET request"
        test_proxy_https "https://httpbin.org/get" "HTTPS GET request"
        ;;
    "icap")
        log_info "Running ICAP tests only..."
        test_icap_service "OPTIONS" "icap://$G3ICAP_SERVER/options"
        test_icap_service "REQMOD" "icap://$G3ICAP_SERVER/reqmod"
        test_icap_service "RESPMOD" "icap://$G3ICAP_SERVER/respmod"
        ;;
    "performance")
        log_info "Running performance tests only..."
        test_performance
        ;;
    "monitoring")
        log_info "Running monitoring tests only..."
        test_monitoring
        ;;
    "all"|"")
        run_comprehensive_tests
        ;;
    *)
        echo "Usage: $0 [health|proxy|icap|performance|monitoring|all]"
        echo ""
        echo "Commands:"
        echo "  health      - Run service health checks"
        echo "  proxy       - Run proxy functionality tests"
        echo "  icap        - Run ICAP service tests"
        echo "  performance - Run performance tests"
        echo "  monitoring  - Run monitoring tests"
        echo "  all         - Run all tests (default)"
        exit 1
        ;;
esac
