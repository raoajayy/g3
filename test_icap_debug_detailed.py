#!/usr/bin/env python3
"""
Detailed ICAP debug test to understand the protocol issue
This script tests the exact ICAP communication that g3proxy is trying to use
"""

import socket
import time

def test_icap_debug_detailed():
    print("🔍 Detailed ICAP Debug Test")
    print("=" * 50)
    
    # Test 1: Check what g3proxy is actually sending to g3icap
    print("\n🧪 Test 1: Simulate g3proxy's ICAP request")
    print("-" * 40)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap")
        
        # Simulate what g3proxy would send for a REQMOD request
        # Based on the g3proxy logs, it's sending a REQMOD request
        http_request_headers = (
            b"GET /get HTTP/1.1\r\n"
            b"Host: httpbin.org\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Accept: application/json\r\n"
            b"Connection: close\r\n"
            b"\r\n"
        )
        
        # Calculate proper lengths
        req_hdr_len = len(http_request_headers)
        req_body_len = 0  # No body for GET request
        
        # Create REQMOD request similar to what g3proxy sends
        icap_request = (
            b"REQMOD icap://127.0.0.1:1344/avscan ICAP/1.0\r\n"
            b"Host: 127.0.0.1:1344\r\n"
            b"User-Agent: G3Proxy/1.0\r\n"
            b"Encapsulated: req-hdr=%d, req-body=%d\r\n\r\n" % (req_hdr_len, req_body_len) +
            http_request_headers
        )
        
        print("📤 Sending REQMOD request (g3proxy style):")
        print(icap_request.decode('utf-8', errors='ignore'))
        
        sock.send(icap_request)
        
        # Receive response
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
            response_str = response.decode('utf-8', errors='ignore')
            lines = response_str.split('\n')
            print(f"📥 Response: {lines[0] if lines else 'Unknown'}")
            
            # Show the encapsulated header if present
            for line in lines[1:10]:
                if line.strip() and ':' in line:
                    print(f"  {line.strip()}")
                    if 'encapsulated:' in line.lower():
                        print(f"    🔍 Encapsulated header: {line.strip()}")
            
            if "204 No Content" in response_str:
                print("✅ REQMOD request successful")
            elif "403 Forbidden" in response_str:
                print("❌ REQMOD request blocked")
            else:
                print(f"ℹ️  REQMOD response: {lines[0] if lines else 'Unknown'}")
        else:
            print("❌ No REQMOD response")
            
    except Exception as e:
        print(f"❌ REQMOD test failed: {e}")
    
    # Test 2: Check g3icap's OPTIONS response
    print("\n🧪 Test 2: Check g3icap OPTIONS response")
    print("-" * 40)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(5)
        sock.connect(('localhost', 1344))
        
        options_request = b"OPTIONS icap://127.0.0.1:1344/avscan ICAP/1.0\r\nHost: 127.0.0.1:1344\r\n\r\n"
        sock.send(options_request)
        
        response = b""
        start_time = time.time()
        while time.time() - start_time < 3:
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
            print("✅ OPTIONS response received")
            
            # Check for methods support
            if "REQMOD" in response_str:
                print("✅ REQMOD method supported")
            if "RESPMOD" in response_str:
                print("✅ RESPMOD method supported")
            if "preview:" in response_str:
                print("✅ Preview support indicated")
        else:
            print("❌ No OPTIONS response")
            
    except Exception as e:
        print(f"❌ OPTIONS test failed: {e}")
    
    print("\n" + "=" * 50)
    print("🔍 Detailed ICAP Debug Summary:")
    print("- Tested REQMOD request similar to g3proxy")
    print("- Checked g3icap OPTIONS capabilities")
    print("- This should help identify the exact protocol issue")

if __name__ == "__main__":
    test_icap_debug_detailed()
