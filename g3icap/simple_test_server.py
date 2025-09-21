#!/usr/bin/env python3
"""
Simple ICAP test server to verify basic functionality
"""

import socket
import threading
import time

def handle_client(client_socket, addr):
    """Handle a client connection"""
    print(f"Handling connection from {addr}")
    
    try:
        # Receive data
        data = client_socket.recv(1024)
        print(f"Received {len(data)} bytes:")
        print(data.decode('utf-8', errors='ignore'))
        
        # Send a simple ICAP response
        response = (
            "ICAP/1.0 200 OK\r\n"
            "ISTag: \"test-server-1.0\"\r\n"
            "Methods: REQMOD, RESPMOD, OPTIONS\r\n"
            "Service: Test ICAP Server\r\n"
            "Encapsulated: null-body=0\r\n"
            "\r\n"
        )
        
        client_socket.send(response.encode())
        print("Sent response")
        
    except Exception as e:
        print(f"Error handling client: {e}")
    finally:
        client_socket.close()

def start_test_server():
    """Start the test server"""
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    server.bind(('localhost', 1345))
    server.listen(5)
    
    print("Test ICAP server listening on localhost:1345")
    
    try:
        while True:
            client_socket, addr = server.accept()
            client_thread = threading.Thread(
                target=handle_client, 
                args=(client_socket, addr)
            )
            client_thread.daemon = True
            client_thread.start()
    except KeyboardInterrupt:
        print("\nShutting down test server...")
    finally:
        server.close()

if __name__ == "__main__":
    start_test_server()
