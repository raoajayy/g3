#!/usr/bin/env python3
"""
Simple ICAP client test script for G3ICAP server
"""

import socket
import time

def test_icap_options():
    """Test ICAP OPTIONS method"""
    print("Testing ICAP OPTIONS method...")
    
    # Connect to G3ICAP server
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        sock.connect(('localhost', 1344))
        
        # Send OPTIONS request
        request = (
            "OPTIONS icap://localhost:1344/options ICAP/1.0\r\n"
            "Host: localhost:1344\r\n"
            "User-Agent: G3ICAP-Test-Client/1.0\r\n"
            "Encapsulated: null-body=0\r\n"
            "\r\n"
        )
        
        sock.send(request.encode())
        
        # Receive response
        response = sock.recv(4096).decode()
        print("OPTIONS Response:")
        print(response)
        
        # Check if response contains expected headers
        if "ICAP/1.0 200 OK" in response:
            print("✅ OPTIONS test passed")
            return True
        else:
            print("❌ OPTIONS test failed")
            return False
            
    except Exception as e:
        print(f"❌ OPTIONS test failed: {e}")
        return False
    finally:
        sock.close()

def test_icap_reqmod():
    """Test ICAP REQMOD method"""
    print("\nTesting ICAP REQMOD method...")
    
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        sock.connect(('localhost', 1344))
        
        # Send REQMOD request
        request = (
            "REQMOD icap://localhost:1344/reqmod ICAP/1.0\r\n"
            "Host: localhost:1344\r\n"
            "User-Agent: G3ICAP-Test-Client/1.0\r\n"
            "Encapsulated: req-hdr=0, req-body=100\r\n"
            "\r\n"
            "GET /test HTTP/1.1\r\n"
            "Host: example.com\r\n"
            "\r\n"
        )
        
        sock.send(request.encode())
        
        # Receive response
        response = sock.recv(4096).decode()
        print("REQMOD Response:")
        print(response)
        
        if "ICAP/1.0 200 OK" in response or "ICAP/1.0 204 No Content" in response:
            print("✅ REQMOD test passed")
            return True
        else:
            print("❌ REQMOD test failed")
            return False
            
    except Exception as e:
        print(f"❌ REQMOD test failed: {e}")
        return False
    finally:
        sock.close()

def test_icap_respmod():
    """Test ICAP RESPMOD method"""
    print("\nTesting ICAP RESPMOD method...")
    
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        sock.connect(('localhost', 1344))
        
        # Send RESPMOD request
        request = (
            "RESPMOD icap://localhost:1344/respmod ICAP/1.0\r\n"
            "Host: localhost:1344\r\n"
            "User-Agent: G3ICAP-Test-Client/1.0\r\n"
            "Encapsulated: req-hdr=0, res-hdr=50, res-body=100\r\n"
            "\r\n"
            "GET /test HTTP/1.1\r\n"
            "Host: example.com\r\n"
            "\r\n"
            "HTTP/1.1 200 OK\r\n"
            "Content-Type: text/html\r\n"
            "\r\n"
        )
        
        sock.send(request.encode())
        
        # Receive response
        response = sock.recv(4096).decode()
        print("RESPMOD Response:")
        print(response)
        
        if "ICAP/1.0 200 OK" in response or "ICAP/1.0 204 No Content" in response:
            print("✅ RESPMOD test passed")
            return True
        else:
            print("❌ RESPMOD test failed")
            return False
            
    except Exception as e:
        print(f"❌ RESPMOD test failed: {e}")
        return False
    finally:
        sock.close()

def main():
    """Run all ICAP tests"""
    print("G3ICAP Server Test Suite")
    print("=" * 40)
    
    # Wait a moment for server to start
    print("Waiting for server to start...")
    time.sleep(2)
    
    tests = [
        test_icap_options,
        test_icap_reqmod,
        test_icap_respmod
    ]
    
    passed = 0
    total = len(tests)
    
    for test in tests:
        if test():
            passed += 1
    
    print(f"\nTest Results: {passed}/{total} tests passed")
    
    if passed == total:
        print("🎉 All tests passed!")
    else:
        print("⚠️  Some tests failed")

if __name__ == "__main__":
    main()
