#!/usr/bin/env python3
"""
Corrected test script for ICAP REQMOD requests with proper formatting
"""

import socket
import time

def test_icap_reqmod_corrected():
    """Test ICAP REQMOD request with correct formatting"""
    print("🧪 Testing ICAP REQMOD request (corrected format)...")
    
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(10)
    
    try:
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap server")
        
        # Properly formatted ICAP REQMOD request
        # The encapsulated data must contain a complete HTTP request
        http_request = (
            "GET /test HTTP/1.1\r\n"
            "Host: example.com\r\n"
            "User-Agent: TestClient/1.0\r\n"
            "Content-Length: 20\r\n"
            "\r\n"
            "This is clean content"
        )
        
        # Calculate the correct encapsulated offsets
        http_request_bytes = http_request.encode()
        req_hdr_end = http_request_bytes.find(b'\r\n\r\n') + 4  # End of headers
        req_body_start = req_hdr_end
        req_body_len = len(http_request_bytes) - req_body_start
        
        # ICAP request with proper encapsulated header
        icap_request = (
            f"REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            f"Host: localhost:1344\r\n"
            f"User-Agent: TestClient/1.0\r\n"
            f"Encapsulated: req-hdr=0, req-body={req_hdr_end}\r\n"
            f"\r\n"
            f"{http_request}"
        )
        
        print("📤 Sending corrected REQMOD request:")
        print("-" * 60)
        print(icap_request)
        print("-" * 60)
        
        sock.send(icap_request.encode())
        
        # Wait for response
        time.sleep(2)
        
        # Receive response
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
            print("=" * 60)
            print(response_str)
            print("=" * 60)
            
            # Check for our response generator improvements
            if "server:" in response_str.lower() and "g3icap" in response_str.lower():
                print("✅ Response generator headers present")
                
                # Check for specific headers
                lines = response_str.split('\r\n')
                headers = {}
                for line in lines[1:]:
                    if ':' in line and line.strip():
                        key, value = line.split(':', 1)
                        headers[key.strip().lower()] = value.strip()
                
                if 'istag' in headers:
                    print(f"✅ ISTag header: {headers['istag']}")
                if 'server' in headers:
                    print(f"✅ Server header: {headers['server']}")
                if 'service-id' in headers:
                    print(f"✅ Service-ID header: {headers['service-id']}")
                    
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

def test_icap_reqmod_malware_corrected():
    """Test ICAP REQMOD request with malware content (corrected format)"""
    print("\n🧪 Testing ICAP REQMOD request with malware (corrected format)...")
    
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(10)
    
    try:
        sock.connect(('localhost', 1344))
        
        # HTTP request with malware content
        http_request = (
            "GET /test HTTP/1.1\r\n"
            "Host: example.com\r\n"
            "User-Agent: TestClient/1.0\r\n"
            "Content-Length: 25\r\n"
            "\r\n"
            "This contains virus malware"
        )
        
        # Calculate encapsulated offsets
        http_request_bytes = http_request.encode()
        req_hdr_end = http_request_bytes.find(b'\r\n\r\n') + 4
        req_body_start = req_hdr_end
        req_body_len = len(http_request_bytes) - req_body_start
        
        # ICAP request
        icap_request = (
            f"REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            f"Host: localhost:1344\r\n"
            f"User-Agent: TestClient/1.0\r\n"
            f"Encapsulated: req-hdr=0, req-body={req_hdr_end}\r\n"
            f"\r\n"
            f"{http_request}"
        )
        
        print("📤 Sending malware REQMOD request:")
        print("-" * 60)
        print(icap_request)
        print("-" * 60)
        
        sock.send(icap_request.encode())
        
        # Wait for response
        time.sleep(2)
        
        # Receive response
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
            print("=" * 60)
            print(response_str)
            print("=" * 60)
            
            # Check response type
            if "403 Forbidden" in response_str:
                print("✅ Malware content blocked (403 Forbidden)")
            elif "204 No Content" in response_str:
                print("⚠️  Malware content allowed (204 No Content)")
            else:
                print(f"ℹ️  Response: {response_str.split('\\r\\n')[0]}")
                
        else:
            print("❌ No response received")
        
        return True
        
    except Exception as e:
        print(f"❌ Error: {e}")
        return False
    finally:
        sock.close()

def main():
    print("🔧 Testing ICAP REQMOD with corrected request formatting")
    print("=" * 70)
    
    # Wait for g3icap to be ready
    print("⏳ Waiting for g3icap to be ready...")
    time.sleep(2)
    
    # Run corrected tests
    test_icap_reqmod_corrected()
    test_icap_reqmod_malware_corrected()
    
    print("\n" + "=" * 70)
    print("✅ Corrected REQMOD tests completed!")

if __name__ == "__main__":
    main()
