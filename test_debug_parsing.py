#!/usr/bin/env python3
"""
Debug ICAP parsing to see what's being sent
"""

import socket
import time

def test_debug_parsing():
    print("🔍 Debug ICAP Parsing")
    print("=" * 40)
    
    # Wait for g3icap to be ready
    print("⏳ Waiting for g3icap to be ready...")
    time.sleep(3)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap")
        
        # Create a simple HTTP request
        http_headers = b"GET /test HTTP/1.1\r\nHost: example.com\r\nUser-Agent: TestClient/1.0\r\n\r\n"
        req_hdr_len = len(http_headers)
        
        # Create REQMOD request
        reqmod_request = (
            b"REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            b"Host: localhost:1344\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Encapsulated: req-hdr=%d, req-body=0\r\n\r\n" % req_hdr_len +
            http_headers
        )
        
        print("📤 REQMOD REQUEST:")
        print("=" * 40)
        print(reqmod_request.decode('utf-8', errors='ignore'))
        print("=" * 40)
        print(f"Request length: {len(reqmod_request)} bytes")
        print(f"HTTP headers length: {req_hdr_len} bytes")
        print(f"Encapsulated header: req-hdr={req_hdr_len}, req-body=0")
        
        # Send request
        sock.send(reqmod_request)
        print("\n📤 Request sent")
        
        # Wait a bit for processing
        time.sleep(1)
        
        sock.close()
        print("✅ Test completed")
        
    except Exception as e:
        print(f"❌ Test failed: {e}")

if __name__ == "__main__":
    test_debug_parsing()
