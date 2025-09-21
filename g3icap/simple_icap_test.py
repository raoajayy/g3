#!/usr/bin/env python3
"""
Simple ICAP test server to verify basic functionality
"""

import socket
import threading
import time

def simple_icap_server():
    """Simple ICAP server that responds to any request"""
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    server.bind(('localhost', 1345))  # Use different port
    server.listen(5)
    print("Simple ICAP server listening on port 1345")
    
    while True:
        try:
            client, addr = server.accept()
            print(f"Connection from {addr}")
            
            # Read request
            data = client.recv(1024)
            print(f"Received: {data.decode()}")
            
            # Send simple ICAP response
            response = """ICAP/1.0 200 OK
Server: Simple-ICAP-Server
Connection: close

""".replace('\n', '\r\n')
            
            client.send(response.encode())
            print("Sent response")
            client.close()
            print("Connection closed")
            
        except Exception as e:
            print(f"Error: {e}")

if __name__ == "__main__":
    simple_icap_server()
