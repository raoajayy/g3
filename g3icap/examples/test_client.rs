//! ICAP Test Client Example
//!
//! This example demonstrates how to test the ICAP server with various requests.

use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("G3 ICAP Test Client");
    println!("==================");

    // Test OPTIONS request
    test_options_request()?;
    
    // Test REQMOD request
    test_reqmod_request()?;
    
    // Test RESPMOD request
    test_respmod_request()?;

    Ok(())
}

fn test_options_request() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n1. Testing OPTIONS request...");
    
    let mut stream = TcpStream::connect("127.0.0.1:1344")?;
    
    let request = "OPTIONS icap://127.0.0.1:1344/options ICAP/1.0\r\nHost: 127.0.0.1:1344\r\nUser-Agent: G3-ICAP-Test-Client\r\n\r\n";
    
    stream.write_all(request.as_bytes())?;
    stream.flush()?;
    
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    
    println!("Response:");
    println!("{}", response);
    
    Ok(())
}

fn test_reqmod_request() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n2. Testing REQMOD request...");
    
    let mut stream = TcpStream::connect("127.0.0.1:1344")?;
    
    let request = "REQMOD icap://127.0.0.1:1344/reqmod ICAP/1.0\r\nHost: 127.0.0.1:1344\r\nUser-Agent: G3-ICAP-Test-Client\r\nEncapsulated: req-hdr=0, req-body=200\r\n\r\nGET /test HTTP/1.1\r\nHost: example.com\r\nUser-Agent: Test-Browser\r\n\r\n";
    
    stream.write_all(request.as_bytes())?;
    stream.flush()?;
    
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    
    println!("Response:");
    println!("{}", response);
    
    Ok(())
}

fn test_respmod_request() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n3. Testing RESPMOD request...");
    
    let mut stream = TcpStream::connect("127.0.0.1:1344")?;
    
    let request = "RESPMOD icap://127.0.0.1:1344/respmod ICAP/1.0\r\nHost: 127.0.0.1:1344\r\nUser-Agent: G3-ICAP-Test-Client\r\nEncapsulated: req-hdr=0, res-hdr=100, res-body=300\r\n\r\nGET /test HTTP/1.1\r\nHost: example.com\r\n\r\nHTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 100\r\n\r\n<html><body>Test content</body></html>";
    
    stream.write_all(request.as_bytes())?;
    stream.flush()?;
    
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    
    println!("Response:");
    println!("{}", response);
    
    Ok(())
}
