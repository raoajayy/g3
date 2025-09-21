#!/usr/bin/env python3

import json
import subprocess
import sys
from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse, parse_qs
import threading
import time

class InfluxDBProxyHandler(BaseHTTPRequestHandler):
    def do_OPTIONS(self):
        self.send_response(200)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.end_headers()

    def do_GET(self):
        if self.path == '/health':
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            self.wfile.write(json.dumps({
                'status': 'ok',
                'service': 'influxdb-proxy'
            }).encode())
        elif self.path == '/tables':
            self.handle_tables()
        else:
            self.send_response(404)
            self.end_headers()

    def do_POST(self):
        if self.path == '/query':
            self.handle_query()
        else:
            self.send_response(404)
            self.end_headers()

    def handle_query(self):
        try:
            content_length = int(self.headers['Content-Length'])
            post_data = self.rfile.read(content_length)
            data = json.loads(post_data.decode('utf-8'))
            
            query = data.get('query')
            db = data.get('db')
            
            if not query or not db:
                self.send_error(400, 'Missing query or db parameter')
                return

            print(f"🔍 Executing query: {query} on database: {db}")
            
            # Execute the query using InfluxDB CLI
            cmd = ['docker', 'exec', 'influxdb3', 'influxdb3', 'query', '--database', db, query]
            result = subprocess.run(cmd, capture_output=True, text=True)
            
            if result.returncode != 0:
                print(f"❌ Query error: {result.stderr}")
                self.send_error(500, result.stderr)
                return

            # Parse the CLI output
            lines = result.stdout.strip().split('\n')
            header_line = None
            
            for line in lines:
                if '|' in line and '+' not in line:
                    header_line = line
                    break
            
            if not header_line:
                response = {"results": [{"series": []}]}
            else:
                headers = [h.strip() for h in header_line.split('|') if h.strip()]
                data_lines = []
                
                for line in lines[lines.index(header_line) + 1:]:
                    if '|' in line and '+' not in line:
                        row = [cell.strip() for cell in line.split('|') if cell.strip()]
                        if len(row) == len(headers):
                            data_lines.append(row)

                # Convert to InfluxDB v1 format for compatibility
                response = {
                    "results": [{
                        "series": [{
                            "name": "result",
                            "columns": headers,
                            "values": data_lines
                        }]
                    }]
                }

            print(f"✅ Query successful, returned {len(data_lines)} rows")
            
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            self.wfile.write(json.dumps(response).encode())

        except Exception as e:
            print(f"❌ Query execution failed: {e}")
            self.send_error(500, str(e))

    def handle_tables(self):
        try:
            cmd = 'docker exec influxdb3 influxdb3 query --database g3proxy "SHOW TABLES"'
            result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
            
            if result.returncode != 0:
                self.send_error(500, result.stderr)
                return

            lines = result.stdout.strip().split('\n')
            header_line = None
            
            for line in lines:
                if '|' in line and '+' not in line:
                    header_line = line
                    break
            
            if not header_line:
                response = {"tables": []}
            else:
                headers = [h.strip() for h in header_line.split('|') if h.strip()]
                tables = []
                
                for line in lines[lines.index(header_line) + 1:]:
                    if '|' in line and '+' not in line:
                        row = [cell.strip() for cell in line.split('|') if cell.strip()]
                        if len(row) == len(headers):
                            table = {}
                            for i, header in enumerate(headers):
                                table[header] = row[i]
                            tables.append(table)

                response = {"tables": tables}

            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            self.wfile.write(json.dumps(response).encode())

        except Exception as e:
            print(f"❌ Failed to get tables: {e}")
            self.send_error(500, str(e))

    def log_message(self, format, *args):
        # Suppress default logging
        pass

def run_server():
    server = HTTPServer(('localhost', 8182), InfluxDBProxyHandler)
    print("🚀 InfluxDB Proxy server running on port 8182")
    print("📊 Health check: http://localhost:8182/health")
    print("🔍 Query endpoint: http://localhost:8182/query")
    print("📋 Tables endpoint: http://localhost:8182/tables")
    
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\n🛑 Shutting down server...")
        server.shutdown()

if __name__ == '__main__':
    run_server()
