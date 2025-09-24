#!/usr/bin/env python3
"""
Debug ICAP integration between g3proxy and g3icap
This script tests the complete flow to understand where the 500 error is coming from
"""

import socket
import time

def test_icap_integration_debug():
    print("🔍 Debugging ICAP Integration between g3proxy and g3icap")
    print("=" * 70)
    
    # Test 1: Direct ICAP test to g3icap
    print("\n🧪 Test 1: Direct ICAP test to g3icap")
    print("-" * 50)
    
    try:
        # Connect directly to g3icap
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        print("✅ Connected directly to g3icap (port 1344)")
        
        # Send OPTIONS request
        options_request = b"OPTIONS icap://localhost:1344/avscan ICAP/1.0\r\nHost: localhost:1344\r\n\r\n"
        sock.send(options_request)
        
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
        
        sock.close()
        
        if response:
            print("✅ Direct ICAP OPTIONS successful")
            response_str = response.decode('utf-8', errors='ignore')
            if "204 No Content" in response_str:
                print("✅ g3icap is responding correctly")
            else:
                print(f"⚠️  Unexpected ICAP response: {response_str.split()[1] if len(response_str.split()) > 1 else 'Unknown'}")
        else:
            print("❌ No direct ICAP response")
            
    except Exception as e:
        print(f"❌ Direct ICAP test failed: {e}")
    
    # Test 2: Check g3proxy status
    print("\n🧪 Test 2: Check g3proxy status")
    print("-" * 50)
    
    try:
        # Check if g3proxy is listening
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(5)
        result = sock.connect_ex(('localhost', 3129))
        sock.close()
        
        if result == 0:
            print("✅ g3proxy is listening on port 3129")
        else:
            print("❌ g3proxy is not listening on port 3129")
            return
    except Exception as e:
        print(f"❌ Cannot check g3proxy status: {e}")
        return
    
    # Test 3: Simple HTTP request through g3proxy (without ICAP)
    print("\n🧪 Test 3: Simple HTTP request through g3proxy")
    print("-" * 50)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 3129))
        print("✅ Connected to g3proxy")
        
        # Send a very simple HTTP request
        http_request = (
            b"GET http://httpbin.org/get HTTP/1.1\r\n"
            b"Host: httpbin.org\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Connection: close\r\n"
            b"\r\n"
        )
        
        print("📤 Sending HTTP request through g3proxy...")
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
            response_str = response.decode('utf-8', errors='ignore')
            lines = response_str.split('\n')
            print(f"📥 Received response: {lines[0] if lines else 'Unknown'}")
            
            if "HTTP/1.1 200" in response_str:
                print("✅ HTTP request successful through g3proxy")
            elif "HTTP/1.1 500" in response_str:
                print("❌ HTTP request failed with 500 error")
                print("🔍 This suggests an issue with ICAP integration")
            else:
                print(f"ℹ️  HTTP response: {lines[0] if lines else 'Unknown'}")
        else:
            print("❌ No HTTP response received")
            
    except Exception as e:
        print(f"❌ HTTP test through g3proxy failed: {e}")
    
    # Test 4: Check if g3proxy can reach g3icap
    print("\n🧪 Test 4: Check g3proxy to g3icap connectivity")
    print("-" * 50)
    
    try:
        # Check if g3proxy has active connections to g3icap
        import subprocess
        result = subprocess.run(['lsof', '-i', ':1344'], capture_output=True, text=True)
        if 'g3proxy' in result.stdout:
            print("✅ g3proxy has active connections to g3icap")
        else:
            print("❌ No active connections from g3proxy to g3icap")
            print("🔍 This might be the issue - g3proxy is not connecting to g3icap")
    except Exception as e:
        print(f"❌ Cannot check connectivity: {e}")
    
    print("\n" + "=" * 70)
    print("🔍 ICAP Integration Debug Summary:")
    print("- Check if g3icap is responding to direct ICAP requests")
    print("- Check if g3proxy is listening and accepting connections")
    print("- Check if g3proxy can successfully make HTTP requests")
    print("- Check if g3proxy is connecting to g3icap for ICAP processing")

if __name__ == "__main__":
    test_icap_integration_debug()
