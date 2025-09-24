#!/usr/bin/env python3
"""
Final ICAP test with proper REQMOD request format
"""

import socket
import time

def test_icap_reqmod_final():
    """Test ICAP REQMOD with proper HTTP request data"""
    print("🧪 Testing ICAP REQMOD with proper HTTP request data...")
    
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(10)
    
    try:
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap server")
        
        # Create a proper HTTP request
        http_request = (
            "GET /test HTTP/1.1\r\n"
            "Host: example.com\r\n"
            "User-Agent: TestClient/1.0\r\n"
            "\r\n"
        )
        
        # Calculate the correct encapsulated offsets
        http_request_bytes = http_request.encode()
        req_hdr_end = http_request_bytes.find(b'\r\n\r\n') + 4  # End of headers
        req_body_start = req_hdr_end
        req_body_len = len(http_request_bytes) - req_body_start
        
        # ICAP REQMOD request with proper encapsulated data
        icap_request = (
            f"REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            f"Host: localhost:1344\r\n"
            f"User-Agent: TestClient/1.0\r\n"
            f"Encapsulated: req-hdr=0, req-body={req_hdr_end}\r\n"
            f"\r\n"
            f"{http_request}"
        )
        
        print("📤 Sending REQMOD request with HTTP data:")
        print("-" * 60)
        print(icap_request)
        print("-" * 60)
        
        sock.send(icap_request.encode())
        
        # Wait for response
        time.sleep(3)
        
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
            print("📥 Received REQMOD response:")
            print("=" * 60)
            print(response_str)
            print("=" * 60)
            
            # Check for our response generator improvements
            if "server:" in response_str.lower():
                print("✅ Response generator headers present!")
                
                # Parse headers
                lines = response_str.split('\r\n')
                status_line = lines[0]
                headers = {}
                
                for line in lines[1:]:
                    if ':' in line and line.strip():
                        key, value = line.split(':', 1)
                        headers[key.strip().lower()] = value.strip()
                
                print(f"Status: {status_line}")
                
                # Check for standard headers
                if 'server' in headers:
                    print(f"✅ Server: {headers['server']}")
                if 'istag' in headers:
                    print(f"✅ ISTag: {headers['istag']}")
                if 'service-id' in headers:
                    print(f"✅ Service-ID: {headers['service-id']}")
                
                # Check response type
                if "204 No Content" in status_line:
                    print("✅ Clean content allowed (204 No Content)")
                elif "403 Forbidden" in status_line:
                    print("✅ Content blocked (403 Forbidden)")
                else:
                    print(f"ℹ️  Response: {status_line}")
                    
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

def test_icap_reqmod_malware_final():
    """Test ICAP REQMOD with malware content"""
    print("\n🧪 Testing ICAP REQMOD with malware content...")
    
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(10)
    
    try:
        sock.connect(('localhost', 1344))
        
        # HTTP request with malware content
        http_request = (
            "GET /malware HTTP/1.1\r\n"
            "Host: example.com\r\n"
            "User-Agent: TestClient/1.0\r\n"
            "Content-Length: 25\r\n"
            "\r\n"
            "This contains virus malware"
        )
        
        # Calculate encapsulated offsets
        http_request_bytes = http_request.encode()
        req_hdr_end = http_request_bytes.find(b'\r\n\r\n') + 4
        
        # ICAP REQMOD request
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
        time.sleep(3)
        
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
            print("📥 Received malware REQMOD response:")
            print("=" * 60)
            print(response_str)
            print("=" * 60)
            
            # Check response type
            lines = response_str.split('\r\n')
            status_line = lines[0]
            
            if "403 Forbidden" in status_line:
                print("✅ Malware content blocked (403 Forbidden)")
            elif "204 No Content" in status_line:
                print("⚠️  Malware content allowed (204 No Content)")
            else:
                print(f"ℹ️  Response: {status_line}")
                
        else:
            print("❌ No response received")
        
        return True
        
    except Exception as e:
        print(f"❌ Error: {e}")
        return False
    finally:
        sock.close()

def main():
    print("🚀 Final ICAP REQMOD Test with Proper HTTP Data")
    print("=" * 70)
    
    # Wait for g3icap to be ready
    print("⏳ Waiting for g3icap to be ready...")
    time.sleep(2)
    
    # Test clean content
    test_icap_reqmod_final()
    
    # Test malware content
    test_icap_reqmod_malware_final()
    
    print("\n" + "=" * 70)
    print("✅ Final REQMOD tests completed!")

if __name__ == "__main__":
    main()
