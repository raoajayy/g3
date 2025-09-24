#!/usr/bin/env python3
"""
Detailed test script to verify REQMOD request handling
with improved response_generator.rs integration
"""

import socket
import time

def test_icap_reqmod_detailed():
    """Test ICAP REQMOD request with detailed debugging"""
    print("🧪 Testing ICAP REQMOD request (detailed)...")
    
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(10)
    
    try:
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap server")
        
        # Send ICAP REQMOD request with proper formatting
        request = (
            "REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            "Host: localhost:1344\r\n"
            "User-Agent: TestClient/1.0\r\n"
            "Encapsulated: req-hdr=0, req-body=20\r\n"
            "\r\n"
            "GET /test HTTP/1.1\r\n"
            "Host: example.com\r\n"
            "\r\n"
            "This is clean content"
        )
        
        print("📤 Sending REQMOD request:")
        print("-" * 40)
        print(request)
        print("-" * 40)
        
        sock.send(request.encode())
        
        # Wait a bit for response
        time.sleep(1)
        
        # Try to receive response
        response = b""
        try:
            while True:
                chunk = sock.recv(1024)
                if not chunk:
                    break
                response += chunk
                if len(chunk) < 1024:
                    break
        except socket.timeout:
            print("⏰ Timeout waiting for response")
        
        if response:
            response_str = response.decode('utf-8', errors='ignore')
            print("📥 Received response:")
            print("=" * 50)
            print(response_str)
            print("=" * 50)
            
            # Check for response generator headers
            if "server:" in response_str.lower():
                print("✅ Response generator headers present")
            else:
                print("❌ No response generator headers found")
        else:
            print("❌ No response received")
        
        return True
        
    except Exception as e:
        print(f"❌ Error: {e}")
        return False
    finally:
        sock.close()

def test_icap_simple_reqmod():
    """Test with a simpler REQMOD request"""
    print("\n🧪 Testing simple ICAP REQMOD request...")
    
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(5)
    
    try:
        sock.connect(('localhost', 1344))
        
        # Very simple REQMOD request
        request = (
            "REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            "Host: localhost:1344\r\n"
            "Encapsulated: req-hdr=0, req-body=0\r\n"
            "\r\n"
        )
        
        print("📤 Sending simple REQMOD request:")
        print("-" * 40)
        print(request)
        print("-" * 40)
        
        sock.send(request.encode())
        
        response = sock.recv(1024)
        if response:
            response_str = response.decode('utf-8', errors='ignore')
            print("📥 Received response:")
            print("=" * 50)
            print(response_str)
            print("=" * 50)
        else:
            print("❌ No response received")
        
        return True
        
    except Exception as e:
        print(f"❌ Error: {e}")
        return False
    finally:
        sock.close()

def main():
    print("🔍 Detailed ICAP REQMOD Testing")
    print("=" * 50)
    
    test_icap_reqmod_detailed()
    test_icap_simple_reqmod()

if __name__ == "__main__":
    main()
