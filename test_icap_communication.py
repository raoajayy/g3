#!/usr/bin/env python3
"""
Test ICAP communication to understand the protocol mismatch
This script tests different ICAP endpoints and methods to find the right configuration
"""

import socket
import time

def test_icap_communication():
    print("🔍 Testing ICAP Communication Patterns")
    print("=" * 60)
    
    # Test different ICAP endpoints
    endpoints = [
        "/avscan",
        "/reqmod", 
        "/respmod",
        "/",
        "/icap"
    ]
    
    for endpoint in endpoints:
        print(f"\n🧪 Testing ICAP endpoint: {endpoint}")
        print("-" * 40)
        
        try:
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.settimeout(5)
            sock.connect(('localhost', 1344))
            
            # Test OPTIONS request
            options_request = f"OPTIONS icap://localhost:1344{endpoint} ICAP/1.0\r\nHost: localhost:1344\r\n\r\n".encode()
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
                lines = response_str.split('\n')
                status_line = lines[0] if lines else "Unknown"
                print(f"✅ OPTIONS {endpoint}: {status_line}")
                
                # Check if this endpoint supports REQMOD
                if "REQMOD" in response_str:
                    print(f"  ✅ Supports REQMOD")
                if "RESPMOD" in response_str:
                    print(f"  ✅ Supports RESPMOD")
            else:
                print(f"❌ No response for {endpoint}")
                
        except Exception as e:
            print(f"❌ Error testing {endpoint}: {e}")
    
    # Test REQMOD with proper HTTP data
    print(f"\n🧪 Testing REQMOD with proper HTTP data")
    print("-" * 40)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        
        # Create proper HTTP request data
        http_headers = b"GET /test HTTP/1.1\r\nHost: example.com\r\nUser-Agent: TestClient/1.0\r\n\r\n"
        http_body = b""
        
        # Calculate lengths
        req_hdr_len = len(http_headers)
        req_body_len = len(http_body)
        
        # Create REQMOD request
        reqmod_request = (
            f"REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            f"Host: localhost:1344\r\n"
            f"User-Agent: TestClient/1.0\r\n"
            f"Encapsulated: req-hdr={req_hdr_len}, req-body={req_body_len}\r\n\r\n"
        ).encode() + http_headers + http_body
        
        print("📤 Sending REQMOD request with proper HTTP data...")
        sock.send(reqmod_request)
        
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
            status_line = lines[0] if lines else "Unknown"
            print(f"✅ REQMOD response: {status_line}")
            
            # Show key headers
            for line in lines[1:10]:
                if line.strip() and ':' in line:
                    print(f"  {line.strip()}")
        else:
            print("❌ No REQMOD response")
            
    except Exception as e:
        print(f"❌ REQMOD test failed: {e}")
    
    print("\n" + "=" * 60)
    print("🔍 ICAP Communication Test Summary:")
    print("- Tested different ICAP endpoints")
    print("- Tested REQMOD with proper HTTP data")
    print("- This should help identify the correct configuration for g3proxy")

if __name__ == "__main__":
    test_icap_communication()
