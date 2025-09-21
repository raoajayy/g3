#!/usr/bin/env python3

import requests
import time
import random
import json

# InfluxDB v3 configuration
INFLUXDB_URL = "http://localhost:8181"
DATABASE = "g3proxy"

def send_metric(metric_name, value, metric_type="gauge"):
    """Send a single metric to InfluxDB v3"""
    timestamp = int(time.time() * 1000000000)  # nanoseconds
    line_protocol = f"{metric_name} value={value} {timestamp}"
    
    try:
        response = requests.post(
            f"{INFLUXDB_URL}/api/v3/write_lp?db={DATABASE}",
            data=line_protocol,
            headers={"Content-Type": "text/plain"}
        )
        if response.status_code == 200:
            print(f"✅ Sent {metric_name}={value}")
        else:
            print(f"❌ Failed to send {metric_name}: {response.status_code} - {response.text}")
    except Exception as e:
        print(f"❌ Error sending {metric_name}: {e}")

def generate_real_metrics():
    """Generate realistic G3Proxy metrics"""
    print("🚀 Starting real metrics generation...")
    
    # Simulate real proxy traffic patterns
    base_requests = 50
    base_connections = 10
    base_response_time = 0.1
    
    for i in range(100):  # Send 100 data points
        # Simulate realistic traffic patterns
        time_of_day_factor = 1 + 0.5 * abs(12 - (i % 24)) / 12  # Peak at noon
        random_factor = random.uniform(0.8, 1.2)
        
        # Generate metrics
        requests_per_second = int(base_requests * time_of_day_factor * random_factor)
        active_connections = int(base_connections * time_of_day_factor * random_factor)
        response_time = base_response_time * random_factor
        bytes_transferred = requests_per_second * random.randint(500, 2000)
        errors = random.randint(0, max(1, requests_per_second // 20))
        total_connections = active_connections + random.randint(0, 5)
        
        # Send metrics
        send_metric("g3proxy_requests_per_second", requests_per_second)
        send_metric("g3proxy_active_connections", active_connections)
        send_metric("g3proxy_response_time_seconds", response_time)
        send_metric("g3proxy_bytes_transferred_total", bytes_transferred)
        send_metric("g3proxy_errors_total", errors)
        send_metric("g3proxy_connections_total", total_connections)
        
        print(f"📊 Batch {i+1}/100: {requests_per_second} req/s, {active_connections} conns, {response_time:.3f}s")
        time.sleep(1)  # Send every second

if __name__ == "__main__":
    generate_real_metrics()
    print("🎉 Real metrics generation completed!")
