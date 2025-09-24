#!/usr/bin/env python3
"""
Test content filtering functionality specifically
"""

import socket
import time

def test_content_filtering():
    print("🔍 Testing Content Filtering Functionality")
    print("=" * 60)
    
    # Wait for g3icap to be ready
    print("⏳ Waiting for g3icap to be ready...")
    time.sleep(3)
    
    # Test 1: Clean content (should be allowed)
    print("\n🧪 Test 1: Clean content (should be allowed)")
    print("-" * 40)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap")
        
        # Clean HTTP request
        http_headers = b"GET /clean-page HTTP/1.1\r\nHost: example.com\r\n\r\n"
        req_hdr_len = len(http_headers)
        
        reqmod_request = (
            b"REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            b"Host: localhost:1344\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Encapsulated: req-hdr=%d, req-body=0\r\n\r\n" % req_hdr_len +
            http_headers
        )
        
        print("📤 Sending clean content REQMOD request...")
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
            print(f"📥 Response: {lines[0] if lines else 'Unknown'}")
            
            if "204 No Content" in response_str:
                print("✅ Clean content allowed (204 No Content)")
            elif "403 Forbidden" in response_str:
                print("❌ Clean content blocked (403 Forbidden) - This is unexpected!")
            else:
                print(f"ℹ️  Unexpected response: {lines[0] if lines else 'Unknown'}")
        else:
            print("❌ No response received for clean content")
            
    except Exception as e:
        print(f"❌ Clean content test failed: {e}")
    
    # Test 2: Malware content (should be blocked)
    print("\n🧪 Test 2: Malware content (should be blocked)")
    print("-" * 40)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap")
        
        # Malware HTTP request
        http_headers = b"GET /malware-page HTTP/1.1\r\nHost: malware.com\r\n\r\n"
        req_hdr_len = len(http_headers)
        
        reqmod_request = (
            b"REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            b"Host: localhost:1344\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Encapsulated: req-hdr=%d, req-body=0\r\n\r\n" % req_hdr_len +
            http_headers
        )
        
        print("📤 Sending malware content REQMOD request...")
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
            print(f"📥 Response: {lines[0] if lines else 'Unknown'}")
            
            if "403 Forbidden" in response_str:
                print("✅ Malware content blocked (403 Forbidden) - Content filtering working!")
            elif "204 No Content" in response_str:
                print("❌ Malware content allowed (204 No Content) - Content filtering not working!")
            else:
                print(f"ℹ️  Unexpected response: {lines[0] if lines else 'Unknown'}")
        else:
            print("❌ No response received for malware content")
            
    except Exception as e:
        print(f"❌ Malware content test failed: {e}")
    
    # Test 3: Keyword-based blocking
    print("\n🧪 Test 3: Keyword-based blocking")
    print("-" * 40)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap")
        
        # Request with blocked keyword in URL
        http_headers = b"GET /virus-scan HTTP/1.1\r\nHost: example.com\r\n\r\n"
        req_hdr_len = len(http_headers)
        
        reqmod_request = (
            b"REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            b"Host: localhost:1344\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Encapsulated: req-hdr=%d, req-body=0\r\n\r\n" % req_hdr_len +
            http_headers
        )
        
        print("📤 Sending keyword-blocked content REQMOD request...")
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
            print(f"📥 Response: {lines[0] if lines else 'Unknown'}")
            
            if "403 Forbidden" in response_str:
                print("✅ Keyword content blocked (403 Forbidden) - Keyword filtering working!")
            elif "204 No Content" in response_str:
                print("❌ Keyword content allowed (204 No Content) - Keyword filtering not working!")
            else:
                print(f"ℹ️  Unexpected response: {lines[0] if lines else 'Unknown'}")
        else:
            print("❌ No response received for keyword content")
            
    except Exception as e:
        print(f"❌ Keyword content test failed: {e}")
    
    print("\n" + "=" * 60)
    print("🔍 Content Filtering Test Summary:")
    print("- Tested clean content (should be allowed)")
    print("- Tested malware domain (should be blocked)")
    print("- Tested blocked keyword (should be blocked)")
    print("\n💡 If all tests show 'No response received', the issue is")
    print("   that g3icap is not processing REQMOD requests at all.")

if __name__ == "__main__":
    test_content_filtering()
