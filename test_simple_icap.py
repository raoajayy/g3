#!/usr/bin/env python3
"""
Simple ICAP test to debug the REQMOD issue
"""

import socket
import time

def test_simple_icap():
    """Test with a very simple ICAP request"""
    print("🧪 Testing simple ICAP request...")
    
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(5)
    
    try:
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap server")
        
        # Very simple OPTIONS request first
        request = (
            "OPTIONS icap://localhost:1344/avscan ICAP/1.0\r\n"
            "Host: localhost:1344\r\n"
            "\r\n"
        )
        
        print("📤 Sending simple OPTIONS request:")
        print(request)
        
        sock.send(request.encode())
        
        # Wait for response
        time.sleep(1)
        
        response = sock.recv(1024)
        if response:
            print("📥 Received OPTIONS response:")
            print(response.decode('utf-8', errors='ignore'))
        else:
            print("❌ No OPTIONS response received")
        
        sock.close()
        
        # Now try REQMOD
        print("\n🧪 Testing simple REQMOD request...")
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(5)
        sock.connect(('localhost', 1344))
        
        # Simple REQMOD request
        request = (
            "REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            "Host: localhost:1344\r\n"
            "Encapsulated: req-hdr=0, req-body=0\r\n"
            "\r\n"
        )
        
        print("📤 Sending simple REQMOD request:")
        print(request)
        
        sock.send(request.encode())
        
        # Wait for response
        time.sleep(2)
        
        response = sock.recv(1024)
        if response:
            print("📥 Received REQMOD response:")
            print(response.decode('utf-8', errors='ignore'))
        else:
            print("❌ No REQMOD response received")
        
        return True
        
    except Exception as e:
        print(f"❌ Error: {e}")
        return False
    finally:
        try:
            sock.close()
        except:
            pass

if __name__ == "__main__":
    test_simple_icap()
