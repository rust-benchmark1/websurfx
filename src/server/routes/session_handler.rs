use std::net::TcpStream;
use std::io::Read;
use crate::engines::file_adapter;

pub fn handle_stream_to_file_ops(mut stream: TcpStream) {
    let mut buffer = [0u8; 512];
    //SOURCE
    let n = stream.read(&mut buffer).unwrap();
    let data = String::from_utf8_lossy(&buffer[..n]).to_string();
    file_adapter::handle_asset_request(data);
} 