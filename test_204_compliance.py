#!/usr/bin/env python3
"""
Test 204 No Modifications response RFC 3507 compliance
"""

import socket
import time

def test_204_compliance():
    print("🔍 204 No Modifications RFC 3507 Compliance Test")
    print("=" * 60)

    # Wait for g3icap to be ready
    print("⏳ Waiting for g3icap to be ready...")
    time.sleep(2)

    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap")
        
        # Create a clean HTTP request that should pass content filtering
        http_headers = b"GET /clean-content HTTP/1.1\r\nHost: example.com\r\nUser-Agent: TestClient/1.0\r\n\r\n"
        req_hdr_len = len(http_headers)
        
        # Create REQMOD request
        icap_headers = (
            b"REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            b"Host: localhost:1344\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Encapsulated: req-hdr=0, req-body=%d\r\n\r\n" % req_hdr_len
        )
        
        total_request = icap_headers + http_headers
        
        print("📤 Sending clean content REQMOD request...")
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
            print("-" * 60)
            response_str = response.decode('utf-8', errors='ignore')
            print(response_str)
            print("-" * 60)
            
            # Check RFC 3507 compliance for 204 responses
            print("\n🔍 RFC 3507 Compliance Check for 204 No Modifications:")
            print("=" * 50)
            
            lines = response_str.split('\n')
            status_line = lines[0] if lines else ""
            headers = {}
            
            for line in lines[1:]:
                if ':' in line and not line.strip().startswith('HTTP/'):
                    key, value = line.split(':', 1)
                    headers[key.strip().lower()] = value.strip()
            
            # Check status line
            if "ICAP/1.0 204 No Modifications" in status_line:
                print("✅ Status line: ICAP/1.0 204 No Modifications")
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
                print("ℹ️  Connection header: Not present (optional for 204)")
            
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
            
            # Check for body (204 should have no body)
            body_start = response_str.find('\r\n\r\n')
            if body_start != -1:
                body = response_str[body_start + 4:]
                if body.strip():
                    print(f"❌ 204 response has body: {body.strip()[:100]}...")
                else:
                    print("✅ 204 response has no body (correct)")
            else:
                print("✅ 204 response has no body (correct)")
            
            print("\n📋 Summary:")
            print("=" * 20)
            if ('encapsulated' in headers and 'null-body=0' in headers['encapsulated'] and 
                'istag' in headers and 'content-type' not in headers and 
                (body_start == -1 or not response_str[body_start + 4:].strip())):
                print("✅ 204 No Modifications response is RFC 3507 compliant!")
            else:
                print("❌ 204 No Modifications response is NOT RFC 3507 compliant")
                
        else:
            print("❌ NO RESPONSE RECEIVED")

    except ConnectionRefusedError:
        print("❌ Connection refused. Is g3icap running on localhost:1344?")
    except Exception as e:
        print(f"❌ An error occurred: {e}")
    finally:
        sock.close()

    print("\n" + "=" * 60)
    print("✅ Test completed")

if __name__ == "__main__":
    test_204_compliance()
