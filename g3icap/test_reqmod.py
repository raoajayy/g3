#!/usr/bin/env python3
"""
Test ICAP server using REQMOD method
"""

import socket
import time

def test_reqmod():
    """Test REQMOD method on ICAP server"""
    try:
        # Connect to server
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        print("✓ Connected to ICAP server")
        
        # Send ICAP REQMOD request
        request = """REQMOD icap://localhost:1344/reqmod ICAP/1.0
Host: localhost:1344
Encapsulated: req-hdr=0, null-body=123

GET /test HTTP/1.1
Host: example.com
User-Agent: Test Client

""".replace('\n', '\r\n')
        
        print("Sending REQMOD request:")
        print(request)
        print("=" * 50)
        
        sock.send(request.encode())
        print("✓ Sent ICAP REQMOD request")
        
        # Wait for response
        sock.settimeout(5)
        try:
            response = sock.recv(4096)
            if response:
                print("✓ Received response:")
                print(response.decode())
                print("=" * 50)
            else:
                print("✗ No response received")
        except socket.timeout:
            print("✗ Timeout waiting for response")
        
        sock.close()
        print("✓ Connection closed")
        
    except Exception as e:
        print(f"✗ Error: {e}")

def test_options():
    """Test OPTIONS method on ICAP server"""
    try:
        # Connect to server
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        print("✓ Connected to ICAP server")
        
        # Send ICAP OPTIONS request
        request = "OPTIONS icap://localhost:1344/options ICAP/1.0\r\nHost: localhost:1344\r\n\r\n"
        
        print("Sending OPTIONS request:")
        print(request)
        print("=" * 50)
        
        sock.send(request.encode())
        print("✓ Sent ICAP OPTIONS request")
        
        # Wait for response
        sock.settimeout(5)
        try:
            response = sock.recv(4096)
            if response:
                print("✓ Received response:")
                print(response.decode())
                print("=" * 50)
            else:
                print("✗ No response received")
        except socket.timeout:
            print("✗ Timeout waiting for response")
        
        sock.close()
        print("✓ Connection closed")
        
    except Exception as e:
        print(f"✗ Error: {e}")

if __name__ == "__main__":
    print("=== ICAP Server REQMOD Test ===")
    
    print("\n1. Testing OPTIONS method:")
    test_options()
    
    print("\n2. Testing REQMOD method:")
    test_reqmod()
