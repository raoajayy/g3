#!/usr/bin/env python3
"""
Test g3icap with Google request using the working format
This script uses the same format as the working test_icap_final.py
"""

import socket
import time

def test_google_working():
    print("🌐 Testing g3icap with Google Request (Working Format)")
    print("=" * 60)
    
    # Wait for g3icap to be ready
    print("⏳ Waiting for g3icap to be ready...")
    time.sleep(2)
    
    try:
        # Connect to g3icap server
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap server")
        
        # Create Google HTTP request data (using working format)
        google_http_headers = (
            b"GET / HTTP/1.1\r\n"
            b"Host: www.google.com\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Accept: text/html\r\n"
            b"\r\n"
        )
        google_http_body = b""  # No body for GET request
        google_http_full = google_http_headers + google_http_body
        
        # Calculate lengths for encapsulated header
        req_hdr_len = len(google_http_headers)
        req_body_len = len(google_http_body)
        
        # Create ICAP REQMOD request (using working format)
        icap_request = (
            b"REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            b"Host: localhost:1344\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Encapsulated: req-hdr=%d, req-body=%d\r\n\r\n" % (req_hdr_len, req_body_len) +
            google_http_headers + google_http_body
        )
        
        print("📤 Sending Google REQMOD request (Working Format):")
        print("-" * 60)
        print(icap_request.decode('utf-8', errors='ignore'))
        print("-" * 60)
        
        # Send the request
        sock.send(icap_request)
        
        # Receive response
        response = b""
        start_time = time.time()
        while time.time() - start_time < 5:  # 5 second timeout
            try:
                sock.settimeout(1)
                chunk = sock.recv(4096)
                if not chunk:
                    break
                response += chunk
                # Check if we have a complete response (ends with \r\n\r\n)
                if b"\r\n\r\n" in response:
                    break
            except socket.timeout:
                break
        
        sock.close()
        
        if response:
            print("📥 Received Google REQMOD response:")
            print("=" * 60)
            print(response.decode('utf-8', errors='ignore'))
            print("=" * 60)
            
            # Check for response generator headers
            response_str = response.decode('utf-8', errors='ignore')
            if "server:" in response_str and "istag:" in response_str:
                print("✅ Response generator headers present!")
                
                # Extract key information
                lines = response_str.split('\n')
                for line in lines:
                    if line.startswith('ICAP/'):
                        print(f"Status: {line.strip()}")
                    elif line.startswith('server:'):
                        print(f"✅ Server: {line.strip()}")
                    elif line.startswith('istag:'):
                        print(f"✅ ISTag: {line.strip()}")
                    elif line.startswith('service-id:'):
                        print(f"✅ Service-ID: {line.strip()}")
                    elif line.startswith('encapsulated:'):
                        print(f"✅ Encapsulated: {line.strip()}")
                
                # Check if Google request was allowed
                if "204 No Content" in response_str:
                    print("✅ Google request allowed (204 No Content)")
                elif "403 Forbidden" in response_str:
                    print("❌ Google request blocked (403 Forbidden)")
                elif "200 OK" in response_str:
                    print("✅ Google request modified (200 OK)")
                else:
                    print(f"ℹ️  Google request status: {response_str.split()[1] if len(response_str.split()) > 1 else 'Unknown'}")
            else:
                print("❌ No response generator headers found")
        else:
            print("❌ No response received")
            
    except Exception as e:
        print(f"❌ Error testing Google request: {e}")
    
    print("\n" + "=" * 60)
    print("✅ Google ICAP test completed!")

if __name__ == "__main__":
    test_google_working()
