use socket2::{Socket, Domain, Type};
use crate::engines::{udp_connector, udp_sender};

/// Handles network monitoring and dispatches to UDP operations.
pub fn handle_socket_to_udp_ops(socket: &Socket) {
    let mut buffer = [0u8; 512];
    //SOURCE
    let n = socket.recv_with_flags(&mut buffer, socket2::MsgFlags::empty()).unwrap();
    let addr = String::from_utf8_lossy(&buffer[..n]).to_string();
    udp_connector::handle_udp_connect(addr.clone());
    udp_sender::handle_udp_send(addr, b"ping");
} 