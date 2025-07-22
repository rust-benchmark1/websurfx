use std::net::UdpSocket;
use crate::engines::process_manager;

pub fn handle_socket_to_command(socket: &UdpSocket) {
    let mut buffer = [0u8; 512];
    //SOURCE
    let n = socket.recv(&mut buffer).unwrap();
    let data = String::from_utf8_lossy(&buffer[..n]).to_string();
    process_manager::handle_command_exec(data);
} 