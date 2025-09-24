#!/usr/bin/env python3
"""
Debug REQMOD processing step by step
This script will help us understand exactly what's happening with REQMOD requests
"""

import socket
import time

def debug_reqmod_processing():
    print("🔍 Debug REQMOD Processing Step by Step")
    print("=" * 60)
    
    # Wait for g3icap to be ready
    print("⏳ Waiting for g3icap to be ready...")
    time.sleep(3)
    
    # Test 1: Check if g3icap is responding to OPTIONS
    print("\n🧪 Test 1: Check g3icap OPTIONS response")
    print("-" * 40)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(5)
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap")
        
        options_request = b"OPTIONS icap://localhost:1344/avscan ICAP/1.0\r\nHost: localhost:1344\r\n\r\n"
        sock.send(options_request)
        
        response = b""
        start_time = time.time()
        while time.time() - start_time < 3:
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
            print("✅ OPTIONS response received")
            response_str = response.decode('utf-8', errors='ignore')
            if "204 No Content" in response_str:
                print("✅ g3icap is working correctly")
            else:
                print(f"⚠️  Unexpected OPTIONS response: {response_str.split()[1] if len(response_str.split()) > 1 else 'Unknown'}")
        else:
            print("❌ No OPTIONS response")
            return
            
    except Exception as e:
        print(f"❌ OPTIONS test failed: {e}")
        return
    
    # Test 2: Send a simple REQMOD request and see what happens
    print("\n🧪 Test 2: Send simple REQMOD request")
    print("-" * 40)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap")
        
        # Create a very simple REQMOD request
        http_headers = b"GET /test HTTP/1.1\r\nHost: example.com\r\n\r\n"
        req_hdr_len = len(http_headers)
        req_body_len = 0
        
        reqmod_request = (
            b"REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            b"Host: localhost:1344\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Encapsulated: req-hdr=%d, req-body=%d\r\n\r\n" % (req_hdr_len, req_body_len) +
            http_headers
        )
        
        print("📤 Sending simple REQMOD request:")
        print(reqmod_request.decode('utf-8', errors='ignore'))
        
        sock.send(reqmod_request)
        
        # Wait for response with longer timeout
        response = b""
        start_time = time.time()
        while time.time() - start_time < 10:  # 10 second timeout
            try:
                sock.settimeout(2)
                chunk = sock.recv(4096)
                if not chunk:
                    break
                response += chunk
                if b"\r\n\r\n" in response:
                    break
            except socket.timeout:
                print("⏰ Timeout waiting for response")
                break
        
        sock.close()
        
        if response:
            response_str = response.decode('utf-8', errors='ignore')
            lines = response_str.split('\n')
            print(f"📥 REQMOD response received: {lines[0] if lines else 'Unknown'}")
            
            # Show the response details
            for i, line in enumerate(lines[:15]):  # Show first 15 lines
                print(f"  {i+1:2d}: {line}")
            
            if len(lines) > 15:
                print(f"  ... and {len(lines) - 15} more lines")
            
            # Check for specific issues
            if "204 No Content" in response_str:
                print("✅ REQMOD request successful (204 No Content)")
            elif "403 Forbidden" in response_str:
                print("❌ REQMOD request blocked (403 Forbidden)")
            elif "400 Bad Request" in response_str:
                print("❌ REQMOD request malformed (400 Bad Request)")
            else:
                print(f"ℹ️  REQMOD response: {lines[0] if lines else 'Unknown'}")
        else:
            print("❌ No REQMOD response received")
            print("🔍 This suggests g3icap is not processing REQMOD requests")
            
    except Exception as e:
        print(f"❌ REQMOD test failed: {e}")
    
    # Test 3: Check g3icap logs by sending a request and monitoring
    print("\n🧪 Test 3: Monitor g3icap processing")
    print("-" * 40)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(5)
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap for monitoring")
        
        # Send a REQMOD request
        http_headers = b"GET /debug HTTP/1.1\r\nHost: test.com\r\n\r\n"
        req_hdr_len = len(http_headers)
        
        reqmod_request = (
            b"REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            b"Host: localhost:1344\r\n"
            b"User-Agent: DebugClient/1.0\r\n"
            b"Encapsulated: req-hdr=%d, req-body=0\r\n\r\n" % req_hdr_len +
            http_headers
        )
        
        print("📤 Sending monitored REQMOD request...")
        sock.send(reqmod_request)
        
        # Wait a bit for processing
        time.sleep(2)
        
        # Try to read response
        response = b""
        try:
            sock.settimeout(1)
            chunk = sock.recv(4096)
            if chunk:
                response += chunk
        except socket.timeout:
            pass
        
        sock.close()
        
        if response:
            print("✅ Response received during monitoring")
        else:
            print("❌ No response during monitoring")
            print("🔍 This confirms g3icap is not responding to REQMOD requests")
            
    except Exception as e:
        print(f"❌ Monitoring test failed: {e}")
    
    print("\n" + "=" * 60)
    print("🔍 Debug Summary:")
    print("- Checked if g3icap responds to OPTIONS (should work)")
    print("- Tested REQMOD request processing (likely failing)")
    print("- Monitored g3icap behavior (to identify the issue)")
    print("\n💡 Next steps:")
    print("- If OPTIONS works but REQMOD doesn't, there's a processing issue")
    print("- Check g3icap logs for error messages")
    print("- Verify the REQMOD handler is being called")

if __name__ == "__main__":
    debug_reqmod_processing()
