#!/usr/bin/env python3
"""
Debug script to test G3ICAP server connectivity
"""

import socket
import time
import sys

def test_connection():
    """Test basic connection to G3ICAP server"""
    print("Testing connection to G3ICAP server...")
    
    try:
        # Create socket
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(5)  # 5 second timeout
        
        # Connect to server
        print("Connecting to localhost:1344...")
        sock.connect(('localhost', 1344))
        print("✅ Connected successfully!")
        
        # Send simple OPTIONS request
        request = (
            "OPTIONS icap://localhost:1344/options ICAP/1.0\r\n"
            "Host: localhost:1344\r\n"
            "User-Agent: Debug-Client/1.0\r\n"
            "\r\n"
        )
        
        print("Sending OPTIONS request...")
        sock.send(request.encode())
        
        # Try to receive response
        print("Waiting for response...")
        response = sock.recv(1024)
        
        if response:
            print(f"✅ Received response ({len(response)} bytes):")
            print(response.decode('utf-8', errors='ignore'))
        else:
            print("❌ No response received")
            
        sock.close()
        return True
        
    except socket.timeout:
        print("❌ Connection timed out")
        return False
    except ConnectionRefused:
        print("❌ Connection refused - server not running")
        return False
    except Exception as e:
        print(f"❌ Connection failed: {e}")
        return False
    finally:
        try:
            sock.close()
        except:
            pass

def check_port():
    """Check if port 1344 is open"""
    print("Checking if port 1344 is open...")
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(1)
        result = sock.connect_ex(('localhost', 1344))
        sock.close()
        
        if result == 0:
            print("✅ Port 1344 is open")
            return True
        else:
            print("❌ Port 1344 is closed")
            return False
    except Exception as e:
        print(f"❌ Error checking port: {e}")
        return False

if __name__ == "__main__":
    print("G3ICAP Server Debug Tool")
    print("=" * 30)
    
    if check_port():
        test_connection()
    else:
        print("Server is not running on port 1344")
        sys.exit(1)
