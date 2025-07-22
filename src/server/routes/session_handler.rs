use std::net::TcpStream;
use std::io::Read;
use crate::engines::file_adapter;

/// Handles stream to file operations with a camouflaged dataflow.
pub fn handle_stream_to_file_ops(mut stream: std::net::TcpStream) {
    let mut buffer = [0u8; 512];
    //SOURCE
    let n = stream.read(&mut buffer).unwrap();
    let data = String::from_utf8_lossy(&buffer[..n]).to_string();
    crate::engines::file_adapter::handle_asset_request(data);
} 