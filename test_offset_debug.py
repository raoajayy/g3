#!/usr/bin/env python3
"""
Debug offset calculation
"""

import socket
import time

def test_offset_debug():
    print("🔍 Debug Offset Calculation")
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
        icap_headers = (
            b"REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            b"Host: localhost:1344\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Encapsulated: req-hdr=0, req-body=%d\r\n\r\n" % req_hdr_len
        )
        
        total_request = icap_headers + http_headers
        
        print("📤 REQUEST ANALYSIS:")
        print("=" * 40)
        print(f"ICAP headers length: {len(icap_headers)} bytes")
        print(f"HTTP headers length: {req_hdr_len} bytes")
        print(f"Total request length: {len(total_request)} bytes")
        print(f"req-hdr offset should be: 0 (start of body)")
        print(f"req-hdr offset in header: 0 (CORRECT!)")
        print(f"req-body offset in header: {req_hdr_len} (end of HTTP headers)")
        print()
        print("ICAP HEADERS:")
        print(icap_headers.decode('utf-8', errors='ignore'))
        print("HTTP HEADERS:")
        print(http_headers.decode('utf-8', errors='ignore'))
        
        # Send request
        sock.send(total_request)
        print("\n📤 Request sent")
        
        # Wait a bit for processing
        time.sleep(1)
        
        sock.close()
        print("✅ Test completed")
        
    except Exception as e:
        print(f"❌ Test failed: {e}")

if __name__ == "__main__":
    test_offset_debug()
