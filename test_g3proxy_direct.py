#!/usr/bin/env python3
"""
Test g3proxy with different HTTP request formats
This script tests various HTTP request formats to find what g3proxy expects
"""

import socket
import time

def test_g3proxy_direct():
    print("🧪 Testing g3proxy with different HTTP request formats")
    print("=" * 60)
    
    # Test 1: Standard HTTP proxy request
    print("\n🧪 Test 1: Standard HTTP proxy request")
    print("-" * 40)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 3129))
        print("✅ Connected to g3proxy")
        
        # Standard HTTP proxy request format
        http_request = (
            b"GET http://httpbin.org/get HTTP/1.1\r\n"
            b"Host: httpbin.org\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Accept: */*\r\n"
            b"Connection: close\r\n"
            b"\r\n"
        )
        
        print("📤 Sending standard HTTP proxy request...")
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
                if b"\r\n\r\n" in response:
                    break
            except socket.timeout:
                break
        
        sock.close()
        
        if response:
            response_str = response.decode('utf-8', errors='ignore')
            lines = response_str.split('\n')
            print(f"📥 Response: {lines[0] if lines else 'Unknown'}")
            
            if "HTTP/1.1 200" in response_str:
                print("✅ Standard HTTP proxy request successful")
            else:
                print(f"❌ Standard HTTP proxy request failed: {lines[0] if lines else 'Unknown'}")
        else:
            print("❌ No response received")
            
    except Exception as e:
        print(f"❌ Standard HTTP proxy test failed: {e}")
    
    # Test 2: Simple GET request (no proxy format)
    print("\n🧪 Test 2: Simple GET request")
    print("-" * 40)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 3129))
        print("✅ Connected to g3proxy")
        
        # Simple GET request
        http_request = (
            b"GET / HTTP/1.1\r\n"
            b"Host: httpbin.org\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Accept: */*\r\n"
            b"Connection: close\r\n"
            b"\r\n"
        )
        
        print("📤 Sending simple GET request...")
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
                if b"\r\n\r\n" in response:
                    break
            except socket.timeout:
                break
        
        sock.close()
        
        if response:
            response_str = response.decode('utf-8', errors='ignore')
            lines = response_str.split('\n')
            print(f"📥 Response: {lines[0] if lines else 'Unknown'}")
            
            if "HTTP/1.1 200" in response_str:
                print("✅ Simple GET request successful")
            else:
                print(f"❌ Simple GET request failed: {lines[0] if lines else 'Unknown'}")
        else:
            print("❌ No response received")
            
    except Exception as e:
        print(f"❌ Simple GET test failed: {e}")
    
    # Test 3: CONNECT request for HTTPS
    print("\n🧪 Test 3: CONNECT request for HTTPS")
    print("-" * 40)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 3128))
        print("✅ Connected to g3proxy HTTPS port")
        
        # CONNECT request
        connect_request = (
            b"CONNECT httpbin.org:443 HTTP/1.1\r\n"
            b"Host: httpbin.org:443\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Connection: close\r\n"
            b"\r\n"
        )
        
        print("📤 Sending CONNECT request...")
        print(connect_request.decode('utf-8', errors='ignore'))
        
        sock.send(connect_request)
        
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
                if b"\r\n\r\n" in response:
                    break
            except socket.timeout:
                break
        
        sock.close()
        
        if response:
            response_str = response.decode('utf-8', errors='ignore')
            lines = response_str.split('\n')
            print(f"📥 Response: {lines[0] if lines else 'Unknown'}")
            
            if "HTTP/1.1 200" in response_str:
                print("✅ CONNECT request successful")
            else:
                print(f"❌ CONNECT request failed: {lines[0] if lines else 'Unknown'}")
        else:
            print("❌ No response received")
            
    except Exception as e:
        print(f"❌ CONNECT test failed: {e}")
    
    print("\n" + "=" * 60)
    print("✅ g3proxy direct testing completed!")

if __name__ == "__main__":
    test_g3proxy_direct()
