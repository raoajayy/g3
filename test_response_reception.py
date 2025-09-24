#!/usr/bin/env python3
"""
Test response reception with different approaches
"""

import socket
import time
import select

def test_response_reception():
    print("🔍 Response Reception Test")
    print("=" * 50)
    
    # Wait for g3icap to be ready
    print("⏳ Waiting for g3icap to be ready...")
    time.sleep(3)
    
    # Test 1: Using select to check for data availability
    print("\n🧪 Test 1: Using select() to check for data")
    print("-" * 40)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(15)
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap")
        
        # Clean HTTP request
        http_headers = b"GET /test HTTP/1.1\r\nHost: example.com\r\nUser-Agent: TestClient/1.0\r\n\r\n"
        req_hdr_len = len(http_headers)
        
        reqmod_request = (
            b"REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            b"Host: localhost:1344\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Encapsulated: req-hdr=%d, req-body=0\r\n\r\n" % req_hdr_len +
            http_headers
        )
        
        print("📤 Sending REQMOD request...")
        sock.send(reqmod_request)
        print(f"📤 Sent {len(reqmod_request)} bytes")
        
        # Use select to check for data availability
        print("⏳ Waiting for response using select()...")
        ready, _, _ = select.select([sock], [], [], 10)  # 10 second timeout
        
        if ready:
            print("✅ Data available for reading")
            response = b""
            start_time = time.time()
            
            while time.time() - start_time < 5:
                try:
                    chunk = sock.recv(4096)
                    if not chunk:
                        print("📥 Connection closed by server")
                        break
                    response += chunk
                    print(f"📥 Received chunk: {len(chunk)} bytes")
                    
                    if b"\r\n\r\n" in response:
                        print("📥 Complete response received")
                        break
                        
                except socket.timeout:
                    print("⏰ Timeout reading data")
                    break
                except Exception as e:
                    print(f"❌ Error reading: {e}")
                    break
            
            if response:
                response_str = response.decode('utf-8', errors='ignore')
                lines = response_str.split('\n')
                status_line = lines[0].strip() if lines else "Unknown"
                print(f"📥 Response: {status_line}")
                print(f"📥 Response length: {len(response)} bytes")
            else:
                print("❌ No response received")
        else:
            print("❌ No data available within timeout")
        
        sock.close()
        
    except Exception as e:
        print(f"❌ Test 1 failed: {e}")
    
    # Test 2: Using a different socket approach
    print("\n🧪 Test 2: Using different socket approach")
    print("-" * 40)
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(20)  # Longer timeout
        sock.connect(('localhost', 1344))
        print("✅ Connected to g3icap")
        
        # Clean HTTP request
        http_headers = b"GET /test2 HTTP/1.1\r\nHost: example.com\r\nUser-Agent: TestClient/1.0\r\n\r\n"
        req_hdr_len = len(http_headers)
        
        reqmod_request = (
            b"REQMOD icap://localhost:1344/avscan ICAP/1.0\r\n"
            b"Host: localhost:1344\r\n"
            b"User-Agent: TestClient/1.0\r\n"
            b"Encapsulated: req-hdr=%d, req-body=0\r\n\r\n" % req_hdr_len +
            http_headers
        )
        
        print("📤 Sending REQMOD request...")
        sock.send(reqmod_request)
        print(f"📤 Sent {len(reqmod_request)} bytes")
        
        # Try to read response with multiple attempts
        response = b""
        print("⏳ Reading response...")
        
        for attempt in range(10):  # 10 attempts
            try:
                print(f"📥 Attempt {attempt + 1}: Reading data...")
                chunk = sock.recv(4096)
                if not chunk:
                    print(f"📥 Attempt {attempt + 1}: No data (connection closed)")
                    break
                
                response += chunk
                print(f"📥 Attempt {attempt + 1}: Received {len(chunk)} bytes (total: {len(response)})")
                
                if b"\r\n\r\n" in response:
                    print("📥 Complete response received")
                    break
                    
                time.sleep(0.1)  # Small delay between attempts
                
            except socket.timeout:
                print(f"📥 Attempt {attempt + 1}: Timeout")
                break
            except Exception as e:
                print(f"📥 Attempt {attempt + 1}: Error: {e}")
                break
        
        sock.close()
        
        if response:
            response_str = response.decode('utf-8', errors='ignore')
            lines = response_str.split('\n')
            status_line = lines[0].strip() if lines else "Unknown"
            print(f"📥 Final Response: {status_line}")
            print(f"📥 Response length: {len(response)} bytes")
            
            # Show first few lines of response
            for i, line in enumerate(lines[:5]):
                print(f"  {i+1}: {line}")
        else:
            print("❌ No response received after all attempts")
            
    except Exception as e:
        print(f"❌ Test 2 failed: {e}")
    
    print("\n" + "=" * 50)
    print("🔍 Response Reception Summary:")
    print("- Tested different approaches to receive responses")
    print("- If no response is received, there may be a connection issue")
    print("- The g3icap logs show responses are being sent successfully")

if __name__ == "__main__":
    test_response_reception()
