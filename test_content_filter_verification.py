#!/usr/bin/env python3
"""
Verify content filtering is working correctly
"""

import socket
import time

def test_content_filter_verification():
    print("🔍 Content Filter Verification Test")
    print("=" * 60)
    
    # Wait for g3icap to be ready
    print("⏳ Waiting for g3icap to be ready...")
    time.sleep(3)
    
    # Test 1: Clean content (should get 204 No Content)
    print("\n🧪 Test 1: Clean content (should get 204 No Content)")
    print("-" * 50)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap")
        
        # Clean HTTP request
        http_headers = b"GET /clean-page HTTP/1.1\r\nHost: example.com\r\nUser-Agent: TestClient/1.0\r\n\r\n"
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
            status_line = lines[0].strip() if lines else "Unknown"
            print(f"📥 Response: {status_line}")
            
            if "204 No Content" in status_line:
                print("✅ Clean content allowed (204 No Content) - Content filtering working correctly!")
            else:
                print(f"❌ Unexpected response for clean content: {status_line}")
        else:
            print("❌ No response received for clean content")
            
    except Exception as e:
        print(f"❌ Clean content test failed: {e}")
    
    # Test 2: Malware domain (should get 403 Forbidden)
    print("\n🧪 Test 2: Malware domain (should get 403 Forbidden)")
    print("-" * 50)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap")
        
        # Malware HTTP request
        http_headers = b"GET /malware-page HTTP/1.1\r\nHost: malware.com\r\nUser-Agent: TestClient/1.0\r\n\r\n"
        req_hdr_len = len(http_headers)
        
        reqmod_request = (
            b"REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            b"Host: localhost:1344\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Encapsulated: req-hdr=%d, req-body=0\r\n\r\n" % req_hdr_len +
            http_headers
        )
        
        print("📤 Sending malware domain REQMOD request...")
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
            status_line = lines[0].strip() if lines else "Unknown"
            print(f"📥 Response: {status_line}")
            
            if "403 Forbidden" in status_line:
                print("✅ Malware domain blocked (403 Forbidden) - Content filtering working correctly!")
            elif "204 No Content" in status_line:
                print("❌ Malware domain allowed (204 No Content) - Content filtering not working!")
            else:
                print(f"ℹ️  Unexpected response for malware domain: {status_line}")
        else:
            print("❌ No response received for malware domain")
            
    except Exception as e:
        print(f"❌ Malware domain test failed: {e}")
    
    # Test 3: Blocked keyword (should get 403 Forbidden)
    print("\n🧪 Test 3: Blocked keyword (should get 403 Forbidden)")
    print("-" * 50)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap")
        
        # Request with blocked keyword in URL
        http_headers = b"GET /virus-scan HTTP/1.1\r\nHost: example.com\r\nUser-Agent: TestClient/1.0\r\n\r\n"
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
            status_line = lines[0].strip() if lines else "Unknown"
            print(f"📥 Response: {status_line}")
            
            if "403 Forbidden" in status_line:
                print("✅ Keyword content blocked (403 Forbidden) - Content filtering working correctly!")
            elif "204 No Content" in status_line:
                print("❌ Keyword content allowed (204 No Content) - Content filtering not working!")
            else:
                print(f"ℹ️  Unexpected response for keyword content: {status_line}")
        else:
            print("❌ No response received for keyword content")
            
    except Exception as e:
        print(f"❌ Keyword content test failed: {e}")
    
    print("\n" + "=" * 60)
    print("🔍 Content Filter Verification Summary:")
    print("- Clean content should return 204 No Content (allowed)")
    print("- Malware domains should return 403 Forbidden (blocked)")
    print("- Blocked keywords should return 403 Forbidden (blocked)")
    print("\n💡 The issue with g3proxy integration is likely in the")
    print("   response format or encapsulated data handling, not in")
    print("   the content filtering logic itself.")

if __name__ == "__main__":
    test_content_filter_verification()
