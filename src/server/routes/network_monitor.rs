use nix::sys::socket::{recvmsg, MsgFlags, SockaddrStorage};
use std::os::unix::io::AsRawFd;
use std::io::IoSliceMut;
use crate::engines::{udp_connector, udp_sender};

/// Handles network monitoring and dispatches to UDP operations.
pub fn handle_socket_to_udp_ops<S: AsRawFd>(socket: &S) {
    let mut buffer = [0u8; 512];
    let mut iov = [IoSliceMut::new(&mut buffer)];
    //SOURCE
    let _msg = recvmsg::<SockaddrStorage>(
        socket.as_raw_fd(),
        &mut iov,
        None,
        MsgFlags::empty(),
    ).unwrap();
    let addr = String::from_utf8_lossy(&buffer).to_string();
    udp_connector::handle_udp_connect(addr.clone());
    udp_sender::handle_udp_send(addr, b"ping");
} 