#!/usr/bin/env python3
"""
Debug test script to test ICAP server connection and response
"""

import socket
import time
import threading

def test_connection():
    """Test basic connection to ICAP server"""
    try:
        # Connect to server
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(5)
        sock.connect(('localhost', 1344))
        print("✓ Connected to ICAP server")
        
        # Send ICAP OPTIONS request
        request = "OPTIONS icap://localhost:1344/options ICAP/1.0\r\nHost: localhost:1344\r\n\r\n"
        sock.send(request.encode())
        print("✓ Sent ICAP OPTIONS request")
        
        # Wait for response
        sock.settimeout(2)
        try:
            response = sock.recv(1024)
            if response:
                print(f"✓ Received response: {response.decode()}")
            else:
                print("✗ No response received")
        except socket.timeout:
            print("✗ Timeout waiting for response")
        
        sock.close()
        print("✓ Connection closed")
        
    except Exception as e:
        print(f"✗ Error: {e}")

def test_server_status():
    """Test if server is running"""
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(1)
        result = sock.connect_ex(('localhost', 1344))
        sock.close()
        
        if result == 0:
            print("✓ Server is running on port 1344")
            return True
        else:
            print("✗ Server is not running on port 1344")
            return False
    except Exception as e:
        print(f"✗ Error checking server status: {e}")
        return False

if __name__ == "__main__":
    print("=== ICAP Server Debug Test ===")
    
    # Check if server is running
    if test_server_status():
        print("\n=== Testing Connection ===")
        test_connection()
    else:
        print("\nPlease start the ICAP server first")
