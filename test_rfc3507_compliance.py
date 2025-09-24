#!/usr/bin/env python3
"""
Test RFC 3507 compliance of ICAP responses
"""

import socket
import time

def test_rfc3507_compliance():
    print("🔍 RFC 3507 Compliance Test")
    print("=" * 50)

    # Wait for g3icap to be ready
    print("⏳ Waiting for g3icap to be ready...")
    time.sleep(2)

    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap")
        
        # Create a simple HTTP request that should trigger content filtering
        http_headers = b"GET /malware-test HTTP/1.1\r\nHost: malware.example.com\r\nUser-Agent: TestClient/1.0\r\n\r\n"
        req_hdr_len = len(http_headers)
        
        # Create REQMOD request
        icap_headers = (
            b"REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            b"Host: localhost:1344\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Encapsulated: req-hdr=0, req-body=%d\r\n\r\n" % req_hdr_len
        )
        
        total_request = icap_headers + http_headers
        
        print("📤 Sending REQMOD request...")
        sock.sendall(total_request)

        response = b""
        try:
            while True:
                chunk = sock.recv(4096)
                if not chunk:
                    break
                response += chunk
        except socket.timeout:
            pass

        if response:
            print("📥 Received response:")
            print("-" * 50)
            response_str = response.decode('utf-8', errors='ignore')
            print(response_str)
            print("-" * 50)
            
            # Check RFC 3507 compliance
            print("\n🔍 RFC 3507 Compliance Check:")
            print("=" * 30)
            
            lines = response_str.split('\n')
            status_line = lines[0] if lines else ""
            headers = {}
            
            for line in lines[1:]:
                if ':' in line and not line.strip().startswith('HTTP/'):
                    key, value = line.split(':', 1)
                    headers[key.strip().lower()] = value.strip()
            
            # Check status line
            if "ICAP/1.0 403 Forbidden" in status_line:
                print("✅ Status line: ICAP/1.0 403 Forbidden")
            else:
                print(f"❌ Status line: {status_line}")
            
            # Check for Encapsulated header
            if 'encapsulated' in headers:
                print(f"✅ Encapsulated header: {headers['encapsulated']}")
                if 'null-body=0' in headers['encapsulated']:
                    print("✅ Encapsulated header format is correct (null-body=0)")
                else:
                    print("❌ Encapsulated header format should be 'null-body=0'")
            else:
                print("❌ Missing Encapsulated header (required by RFC 3507)")
            
            # Check for ISTag header
            if 'istag' in headers:
                print(f"✅ ISTag header: {headers['istag']}")
            else:
                print("❌ Missing ISTag header (required by RFC 3507)")
            
            # Check for Connection header
            if 'connection' in headers:
                print(f"✅ Connection header: {headers['connection']}")
            else:
                print("❌ Missing Connection header")
            
            # Check for Content-Type at ICAP level (should NOT be present)
            if 'content-type' in headers:
                print(f"❌ Content-Type at ICAP level: {headers['content-type']} (should not be present)")
            else:
                print("✅ No Content-Type at ICAP level (correct)")
            
            # Check for Server header
            if 'server' in headers:
                print(f"✅ Server header: {headers['server']}")
            else:
                print("ℹ️  Server header: Not present (optional)")
            
            print("\n📋 Summary:")
            print("=" * 20)
            if ('encapsulated' in headers and 'null-body=0' in headers['encapsulated'] and 
                'istag' in headers and 'connection' in headers and 'content-type' not in headers):
                print("✅ Response is RFC 3507 compliant!")
            else:
                print("❌ Response is NOT RFC 3507 compliant")
                
        else:
            print("❌ NO RESPONSE RECEIVED")

    except ConnectionRefusedError:
        print("❌ Connection refused. Is g3icap running on localhost:1344?")
    except Exception as e:
        print(f"❌ An error occurred: {e}")
    finally:
        sock.close()

    print("\n" + "=" * 50)
    print("✅ Test completed")

if __name__ == "__main__":
    test_rfc3507_compliance()
