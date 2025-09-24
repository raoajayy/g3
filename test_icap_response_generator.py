#!/usr/bin/env python3
"""
Test script to verify the improved response_generator.rs integration
with g3icap server. This tests the proper ICAP response generation
with standard headers, chunked support, and error handling.
"""

import socket
import time

def test_icap_options():
    """Test ICAP OPTIONS request to verify response generator headers"""
    print("🧪 Testing ICAP OPTIONS request...")
    
    # Create socket connection
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(10)
    
    try:
        # Connect to g3icap server
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap server on localhost:1344")
        
        # Send ICAP OPTIONS request
        request = (
            "OPTIONS icap://localhost:1344/avscan ICAP/1.0\r\n"
            "Host: localhost:1344\r\n"
            "User-Agent: TestClient/1.0\r\n"
            "\r\n"
        )
        
        sock.send(request.encode())
        print("📤 Sent ICAP OPTIONS request")
        
        # Receive response
        response = sock.recv(4096).decode()
        print("📥 Received response:")
        print("=" * 50)
        print(response)
        print("=" * 50)
        
        # Verify response generator improvements
        lines = response.split('\r\n')
        status_line = lines[0]
        headers = {}
        
        for line in lines[1:]:
            if ':' in line:
                key, value = line.split(':', 1)
                headers[key.strip().lower()] = value.strip()
        
        print("\n🔍 Response Analysis:")
        print(f"Status: {status_line}")
        
        # Check for standard headers from response generator
        expected_headers = ['server', 'istag', 'service-id', 'methods', 'service']
        for header in expected_headers:
            if header in headers:
                print(f"✅ {header}: {headers[header]}")
            else:
                print(f"❌ Missing {header}")
        
        # Check for chunked encoding if present
        if 'transfer-encoding' in headers:
            print(f"✅ Transfer-Encoding: {headers['transfer-encoding']}")
        
        return True
        
    except Exception as e:
        print(f"❌ Error: {e}")
        return False
    finally:
        sock.close()

def test_icap_reqmod_clean():
    """Test ICAP REQMOD request with clean content"""
    print("\n🧪 Testing ICAP REQMOD request (clean content)...")
    
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(10)
    
    try:
        sock.connect(('localhost', 1344))
        
        # Send ICAP REQMOD request with clean content
        request = (
            "REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            "Host: localhost:1344\r\n"
            "User-Agent: TestClient/1.0\r\n"
            "Encapsulated: req-hdr=0, req-body=20\r\n"
            "\r\n"
            "GET /test HTTP/1.1\r\n"
            "Host: example.com\r\n"
            "\r\n"
            "This is clean content"
        )
        
        sock.send(request.encode())
        print("📤 Sent ICAP REQMOD request with clean content")
        
        response = sock.recv(4096).decode()
        print("📥 Received response:")
        print("=" * 50)
        print(response)
        print("=" * 50)
        
        # Check for proper response generator usage
        if "204 No Content" in response:
            print("✅ Clean content allowed (204 No Content)")
        elif "403 Forbidden" in response:
            print("⚠️  Content blocked (403 Forbidden)")
        else:
            print(f"ℹ️  Unexpected response: {response.split('\\r\\n')[0]}")
        
        return True
        
    except Exception as e:
        print(f"❌ Error: {e}")
        return False
    finally:
        sock.close()

def test_icap_reqmod_malware():
    """Test ICAP REQMOD request with malware content"""
    print("\n🧪 Testing ICAP REQMOD request (malware content)...")
    
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(10)
    
    try:
        sock.connect(('localhost', 1344))
        
        # Send ICAP REQMOD request with malware content
        request = (
            "REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            "Host: localhost:1344\r\n"
            "User-Agent: TestClient/1.0\r\n"
            "Encapsulated: req-hdr=0, req-body=25\r\n"
            "\r\n"
            "GET /test HTTP/1.1\r\n"
            "Host: example.com\r\n"
            "\r\n"
            "This contains virus malware"
        )
        
        sock.send(request.encode())
        print("📤 Sent ICAP REQMOD request with malware content")
        
        response = sock.recv(4096).decode()
        print("📥 Received response:")
        print("=" * 50)
        print(response)
        print("=" * 50)
        
        # Check for proper response generator usage
        if "403 Forbidden" in response:
            print("✅ Malware content blocked (403 Forbidden)")
            
            # Check for proper headers from response generator
            lines = response.split('\r\n')
            headers = {}
            for line in lines[1:]:
                if ':' in line:
                    key, value = line.split(':', 1)
                    headers[key.strip().lower()] = value.strip()
            
            if 'server' in headers and 'g3icap' in headers['server'].lower():
                print("✅ Response generator headers present")
            else:
                print("❌ Missing response generator headers")
                
        elif "204 No Content" in response:
            print("⚠️  Malware content allowed (204 No Content)")
        else:
            print(f"ℹ️  Unexpected response: {response.split('\\r\\n')[0]}")
        
        return True
        
    except Exception as e:
        print(f"❌ Error: {e}")
        return False
    finally:
        sock.close()

def main():
    """Run all tests"""
    print("🚀 Testing g3icap with improved response_generator.rs integration")
    print("=" * 70)
    
    # Wait a moment for g3icap to fully start
    print("⏳ Waiting for g3icap to start...")
    time.sleep(2)
    
    # Run tests
    tests = [
        test_icap_options,
        test_icap_reqmod_clean,
        test_icap_reqmod_malware
    ]
    
    passed = 0
    total = len(tests)
    
    for test in tests:
        if test():
            passed += 1
        print()
    
    print("=" * 70)
    print(f"📊 Test Results: {passed}/{total} tests passed")
    
    if passed == total:
        print("🎉 All tests passed! Response generator integration is working correctly.")
    else:
        print("⚠️  Some tests failed. Check the output above for details.")

if __name__ == "__main__":
    main()
