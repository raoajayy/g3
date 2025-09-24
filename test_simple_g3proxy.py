#!/usr/bin/env python3
"""
Test simple HTTP request through g3proxy
This script tests a simple HTTP request to see if g3proxy is working correctly
"""

import socket
import time

def test_simple_g3proxy():
    print("🧪 Testing Simple HTTP Request through g3proxy")
    print("=" * 50)
    
    try:
        # Connect to g3proxy HTTP port
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 3129))
        print("✅ Connected to g3proxy HTTP port (3129)")
        
        # Send simple HTTP request to a test server
        http_request = (
            b"GET http://httpbin.org/get HTTP/1.1\r\n"
            b"Host: httpbin.org\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Accept: application/json\r\n"
            b"Connection: close\r\n"
            b"\r\n"
        )
        
        print("📤 Sending simple HTTP request to httpbin.org:")
        print(http_request.decode('utf-8', errors='ignore'))
        
        sock.send(http_request)
        
        # Receive response
        response = b""
        start_time = time.time()
        while time.time() - start_time < 10:
            try:
                sock.settimeout(1)
                chunk = sock.recv(4096)
                if not chunk:
                    break
                response += chunk
                if b"\r\n\r\n" in response and len(response) > 100:
                    break
            except socket.timeout:
                break
        
        sock.close()
        
        if response:
            print("📥 Received HTTP response:")
            print("=" * 30)
            response_str = response.decode('utf-8', errors='ignore')
            lines = response_str.split('\n')
            
            # Show first 10 lines
            for i, line in enumerate(lines[:10]):
                print(f"{i+1:2d}: {line}")
            
            if len(lines) > 10:
                print(f"... and {len(lines) - 10} more lines")
            
            print("=" * 30)
            
            if "HTTP/1.1 200" in response_str:
                print("✅ HTTP request successful (200 OK)")
            elif "HTTP/1.1 403" in response_str:
                print("❌ HTTP request blocked (403 Forbidden)")
            elif "HTTP/1.1 500" in response_str:
                print("❌ HTTP request failed (500 Internal Server Error)")
            else:
                print(f"ℹ️  HTTP response status: {lines[0] if lines else 'Unknown'}")
        else:
            print("❌ No HTTP response received")
            
    except Exception as e:
        print(f"❌ Error testing simple HTTP through g3proxy: {e}")
    
    print("\n" + "=" * 50)
    print("✅ Simple g3proxy test completed!")

if __name__ == "__main__":
    test_simple_g3proxy()
