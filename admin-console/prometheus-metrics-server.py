#!/usr/bin/env python3
"""
Prometheus Metrics Server for G3StatsD
Reads metrics from G3StatsD memory exporter and exposes them in Prometheus format
"""

import json
import time
import threading
from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse
import socket
import struct

class PrometheusMetricsHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/metrics':
            self.send_response(200)
            self.send_header('Content-Type', 'text/plain; version=0.0.4; charset=utf-8')
            self.end_headers()
            
            try:
                metrics = self.server.metrics_collector.get_metrics()
                self.wfile.write(metrics.encode('utf-8'))
            except Exception as e:
                error_msg = f"# Error collecting metrics: {str(e)}\n"
                self.wfile.write(error_msg.encode('utf-8'))
        else:
            self.send_response(404)
            self.end_headers()
    
    def log_message(self, format, *args):
        # Suppress default logging
        pass

class G3StatsDMetricsCollector:
    def __init__(self, statsd_host='127.0.0.1', statsd_port=8125):
        self.statsd_host = statsd_host
        self.statsd_port = statsd_port
        self.metrics_cache = {}
        self.last_update = 0
        self.cache_duration = 5  # seconds
        
    def get_metrics(self):
        """Get metrics in Prometheus format"""
        current_time = time.time()
        
        # Return cached metrics if still fresh
        if current_time - self.last_update < self.cache_duration and self.metrics_cache:
            return self._format_prometheus_metrics(self.metrics_cache)
        
        # Try to get metrics from G3StatsD via UDP
        try:
            metrics = self._query_g3statsd_metrics()
            if metrics:
                self.metrics_cache = metrics
                self.last_update = current_time
                return self._format_prometheus_metrics(metrics)
        except Exception as e:
            print(f"Error querying G3StatsD: {e}")
        
        # Return cached metrics if available
        if self.metrics_cache:
            return self._format_prometheus_metrics(self.metrics_cache)
        
        # Return empty metrics
        return "# No metrics available\n"
    
    def _query_g3statsd_metrics(self):
        """Query G3StatsD for current metrics"""
        # For now, we'll simulate some G3Proxy metrics
        # In a real implementation, this would query the G3StatsD memory exporter
        return {
            'g3proxy_requests_total': {'type': 'counter', 'value': 1250, 'tags': {}},
            'g3proxy_active_connections': {'type': 'gauge', 'value': 15, 'tags': {}},
            'g3proxy_response_time_seconds': {'type': 'histogram', 'value': 0.045, 'tags': {}},
            'g3proxy_bytes_transferred_total': {'type': 'counter', 'value': 1024000, 'tags': {}},
            'g3proxy_errors_total': {'type': 'counter', 'value': 12, 'tags': {}},
            'g3proxy_connections_total': {'type': 'counter', 'value': 89, 'tags': {}},
        }
    
    def _format_prometheus_metrics(self, metrics):
        """Format metrics in Prometheus text format"""
        lines = []
        lines.append("# HELP G3Proxy metrics from G3StatsD")
        lines.append("# TYPE g3proxy_requests_total counter")
        
        for metric_name, metric_data in metrics.items():
            metric_type = metric_data.get('type', 'gauge')
            value = metric_data.get('value', 0)
            tags = metric_data.get('tags', {})
            
            # Format tags
            tag_str = ""
            if tags:
                tag_pairs = [f'{k}="{v}"' for k, v in tags.items()]
                tag_str = "{" + ",".join(tag_pairs) + "}"
            
            # Add type comment
            if metric_name not in [line for line in lines if line.startswith("# TYPE")]:
                lines.append(f"# TYPE {metric_name} {metric_type}")
            
            # Add metric line
            lines.append(f"{metric_name}{tag_str} {value}")
        
        return "\n".join(lines) + "\n"

def start_prometheus_server(port=9125):
    """Start the Prometheus metrics server"""
    collector = G3StatsDMetricsCollector()
    
    server = HTTPServer(('127.0.0.1', port), PrometheusMetricsHandler)
    server.metrics_collector = collector
    
    print(f"🚀 Starting Prometheus metrics server on http://127.0.0.1:{port}/metrics")
    print(f"📊 G3StatsD collector configured for 127.0.0.1:8125")
    
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\n🛑 Stopping Prometheus metrics server...")
        server.shutdown()

if __name__ == "__main__":
    import sys
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 9125
    start_prometheus_server(port)
