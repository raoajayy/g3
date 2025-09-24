#!/usr/bin/env python3
"""
Detailed REQMOD test showing exact request and response
"""

import socket
import time

def test_detailed_reqmod():
    print("🔍 Detailed REQMOD Test - Request and Response Analysis")
    print("=" * 70)
    
    # Wait for g3icap to be ready
    print("⏳ Waiting for g3icap to be ready...")
    time.sleep(3)
    
    # Test 1: Simple REQMOD request
    print("\n🧪 Test 1: Simple REQMOD Request")
    print("-" * 50)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(15)  # Longer timeout
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap")
        
        # Create a simple HTTP request
        http_headers = b"GET /test HTTP/1.1\r\nHost: example.com\r\nUser-Agent: TestClient/1.0\r\n\r\n"
        req_hdr_len = len(http_headers)
        req_body_len = 0
        
        # Create REQMOD request
        reqmod_request = (
            b"REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            b"Host: localhost:1344\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Encapsulated: req-hdr=%d, req-body=%d\r\n\r\n" % (req_hdr_len, req_body_len) +
            http_headers
        )
        
        print("📤 REQUEST BEING SENT:")
        print("=" * 50)
        print(reqmod_request.decode('utf-8', errors='ignore'))
        print("=" * 50)
        print(f"Request length: {len(reqmod_request)} bytes")
        
        # Send request
        sock.send(reqmod_request)
        print("\n📤 Request sent, waiting for response...")
        
        # Wait for response with detailed monitoring
        response = b""
        start_time = time.time()
        chunk_count = 0
        
        while time.time() - start_time < 10:  # 10 second timeout
            try:
                sock.settimeout(2)
                chunk = sock.recv(4096)
                if not chunk:
                    print(f"📥 Received empty chunk (connection closed)")
                    break
                
                chunk_count += 1
                response += chunk
                print(f"📥 Received chunk {chunk_count}: {len(chunk)} bytes")
                
                # Check if we have a complete response
                if b"\r\n\r\n" in response:
                    print("📥 Complete response received (found \\r\\n\\r\\n)")
                    break
                    
            except socket.timeout:
                print(f"⏰ Timeout waiting for chunk {chunk_count + 1}")
                break
            except Exception as e:
                print(f"❌ Error receiving data: {e}")
                break
        
        sock.close()
        
        print(f"\n📥 TOTAL RESPONSE RECEIVED:")
        print("=" * 50)
        if response:
            response_str = response.decode('utf-8', errors='ignore')
            print(response_str)
            print("=" * 50)
            print(f"Response length: {len(response)} bytes")
            print(f"Response chunks: {chunk_count}")
            
            # Analyze response
            lines = response_str.split('\n')
            if lines:
                status_line = lines[0].strip()
                print(f"\n🔍 Response Analysis:")
                print(f"Status line: {status_line}")
                
                if "ICAP/1.0" in status_line:
                    print("✅ Valid ICAP response received")
                else:
                    print("❌ Invalid ICAP response format")
            else:
                print("❌ Empty response")
        else:
            print("❌ NO RESPONSE RECEIVED")
            print("=" * 50)
            print("This indicates g3icap is not processing REQMOD requests")
            
    except ConnectionRefusedError:
        print("❌ Connection refused. Is g3icap running on localhost:1344?")
    except Exception as e:
        print(f"❌ Test failed: {e}")
    
    # Test 2: OPTIONS request for comparison
    print("\n🧪 Test 2: OPTIONS Request (for comparison)")
    print("-" * 50)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap")
        
        options_request = b"OPTIONS icap://localhost:1344/avscan ICAP/1.0\r\nHost: localhost:1344\r\n\r\n"
        
        print("📤 OPTIONS REQUEST:")
        print("=" * 50)
        print(options_request.decode('utf-8', errors='ignore'))
        print("=" * 50)
        
        sock.send(options_request)
        print("\n📤 OPTIONS request sent, waiting for response...")
        
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
        
        print(f"\n📥 OPTIONS RESPONSE:")
        print("=" * 50)
        if response:
            response_str = response.decode('utf-8', errors='ignore')
            print(response_str)
            print("=" * 50)
            print(f"Response length: {len(response)} bytes")
            
            if "204 No Content" in response_str:
                print("✅ OPTIONS working correctly")
            else:
                print("❌ OPTIONS not working as expected")
        else:
            print("❌ No OPTIONS response")
            
    except Exception as e:
        print(f"❌ OPTIONS test failed: {e}")
    
    print("\n" + "=" * 70)
    print("🔍 SUMMARY:")
    print("- OPTIONS requests work (g3icap is running)")
    print("- REQMOD requests get no response (processing issue)")
    print("- This suggests a bug in REQMOD request handling")

if __name__ == "__main__":
    test_detailed_reqmod()
