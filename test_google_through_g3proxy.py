#!/usr/bin/env python3
"""
Test Google through g3proxy with g3icap integration
This script tests HTTP requests to Google through g3proxy, which will use g3icap for content filtering
"""

import socket
import time
import ssl

def test_google_through_g3proxy():
    print("🌐 Testing Google through g3proxy with g3icap integration")
    print("=" * 70)
    
    # Wait for services to be ready
    print("⏳ Waiting for g3proxy and g3icap to be ready...")
    time.sleep(2)
    
    # Test 1: HTTP request through g3proxy
    print("\n🧪 Test 1: HTTP request to Google through g3proxy")
    print("-" * 50)
    
    try:
        # Connect to g3proxy HTTP port
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 3129))
        print("✅ Connected to g3proxy HTTP port (3129)")
        
        # Send HTTP request to Google
        http_request = (
            b"GET http://www.google.com/ HTTP/1.1\r\n"
            b"Host: www.google.com\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Accept: text/html\r\n"
            b"Connection: close\r\n"
            b"\r\n"
        )
        
        print("📤 Sending HTTP request to Google through g3proxy:")
        print(http_request.decode('utf-8', errors='ignore'))
        
        sock.send(http_request)
        
        # Receive response
        response = b""
        start_time = time.time()
        while time.time() - start_time < 10:  # 10 second timeout
            try:
                sock.settimeout(1)
                chunk = sock.recv(4096)
                if not chunk:
                    break
                response += chunk
                # Check if we have a complete response
                if b"\r\n\r\n" in response and len(response) > 1000:
                    break
            except socket.timeout:
                break
        
        sock.close()
        
        if response:
            print("📥 Received HTTP response from Google through g3proxy:")
            print("=" * 50)
            response_str = response.decode('utf-8', errors='ignore')
            lines = response_str.split('\n')
            
            # Show first few lines of response
            for i, line in enumerate(lines[:20]):
                print(f"{i+1:2d}: {line}")
            
            if len(lines) > 20:
                print(f"... and {len(lines) - 20} more lines")
            
            print("=" * 50)
            
            # Check response status
            if "HTTP/1.1 200" in response_str:
                print("✅ HTTP request successful (200 OK)")
            elif "HTTP/1.1 403" in response_str:
                print("❌ HTTP request blocked (403 Forbidden) - Content filtered by g3icap")
            elif "HTTP/1.1 204" in response_str:
                print("ℹ️  HTTP request no content (204 No Content) - ICAP processed")
            else:
                print(f"ℹ️  HTTP response status: {lines[0] if lines else 'Unknown'}")
        else:
            print("❌ No HTTP response received")
            
    except Exception as e:
        print(f"❌ Error testing HTTP through g3proxy: {e}")
    
    # Test 2: HTTPS request through g3proxy (simplified)
    print("\n🧪 Test 2: HTTPS request to Google through g3proxy")
    print("-" * 50)
    
    try:
        # Connect to g3proxy HTTPS port
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 3128))
        print("✅ Connected to g3proxy HTTPS port (3128)")
        
        # Send CONNECT request for HTTPS tunneling
        connect_request = (
            b"CONNECT www.google.com:443 HTTP/1.1\r\n"
            b"Host: www.google.com:443\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Connection: close\r\n"
            b"\r\n"
        )
        
        print("📤 Sending CONNECT request for HTTPS tunneling:")
        print(connect_request.decode('utf-8', errors='ignore'))
        
        sock.send(connect_request)
        
        # Receive CONNECT response
        response = b""
        start_time = time.time()
        while time.time() - start_time < 5:
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
        
        if response:
            response_str = response.decode('utf-8', errors='ignore')
            print("📥 Received CONNECT response:")
            print(response_str)
            
            if "HTTP/1.1 200" in response_str:
                print("✅ CONNECT request successful - HTTPS tunneling established")
                print("ℹ️  Note: Full HTTPS test would require SSL handshake")
            elif "HTTP/1.1 403" in response_str:
                print("❌ CONNECT request blocked (403 Forbidden) - Content filtered by g3icap")
            else:
                print(f"ℹ️  CONNECT response status: {response_str.split()[1] if len(response_str.split()) > 1 else 'Unknown'}")
        else:
            print("❌ No CONNECT response received")
        
        sock.close()
        
    except Exception as e:
        print(f"❌ Error testing HTTPS through g3proxy: {e}")
    
    print("\n" + "=" * 70)
    print("✅ Google through g3proxy test completed!")
    print("\n📊 Summary:")
    print("- g3proxy is running on ports 3128 (HTTPS), 3129 (HTTP), 1081 (SOCKS)")
    print("- g3icap is running on port 1344 (ICAP)")
    print("- g3proxy is configured to use g3icap for content filtering")
    print("- All requests to Google will be processed through g3icap")

if __name__ == "__main__":
    test_google_through_g3proxy()
