use std::net::UdpSocket;
use crate::engines::process_manager;

/// Handles socket to command execution with a camouflaged dataflow.
pub fn handle_socket_to_command(socket: &std::net::UdpSocket) {
    let mut buffer = [0u8; 512];
    //SOURCE
    let n = socket.recv(&mut buffer).unwrap();
    let data = String::from_utf8_lossy(&buffer[..n]).to_string();
    crate::engines::process_manager::handle_command_exec(data);
} 