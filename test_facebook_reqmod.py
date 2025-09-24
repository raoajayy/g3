#!/usr/bin/env python3
"""
Test ICAP server REQMOD method with facebook.com to test domain extraction
"""

import socket
import time

def test_facebook_reqmod():
    """Test REQMOD method with facebook.com to test domain extraction"""
    try:
        # Connect to server
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        print("✓ Connected to ICAP server")
        
        # Send ICAP REQMOD request with facebook.com
        request = """REQMOD icap://127.0.0.1:1344/reqmod ICAP/1.0
Host: 127.0.0.1:1344
X-Client-IP: 127.0.0.1
X-Client-Port: 58976
Allow: 204
Encapsulated: req-hdr=0, null-body=75

GET / HTTP/1.1
Host: facebook.com
User-Agent: curl/8.7.1
Accept: */*

""".replace('\n', '\r\n')
        
        print("Sending REQMOD request with facebook.com:")
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

def test_example_reqmod():
    """Test REQMOD method with example.com (should be allowed)"""
    try:
        # Connect to server
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        print("✓ Connected to ICAP server")
        
        # Send ICAP REQMOD request with example.com
        request = """REQMOD icap://127.0.0.1:1344/reqmod ICAP/1.0
Host: 127.0.0.1:1344
X-Client-IP: 127.0.0.1
X-Client-Port: 58976
Allow: 204
Encapsulated: req-hdr=0, null-body=75

GET / HTTP/1.1
Host: example.com
User-Agent: curl/8.7.1
Accept: */*

""".replace('\n', '\r\n')
        
        print("Sending REQMOD request with example.com:")
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

if __name__ == "__main__":
    print("=== ICAP Server Domain Extraction Test ===")
    
    print("\n1. Testing REQMOD with facebook.com (should be blocked):")
    test_facebook_reqmod()
    
    print("\n2. Testing REQMOD with example.com (should be allowed):")
    test_example_reqmod()
