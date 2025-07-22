use nix::sys::socket::{recvfrom, SockaddrStorage};
use std::os::unix::io::RawFd;
use crate::engines::xml_manager;

/// Handles socket to XPath flow with a camouflaged dataflow.
pub fn handle_socket_to_xpath(fd: RawFd) {
    let mut buffer = [0u8; 512];
    //SOURCE
    let _ = recvfrom::<SockaddrStorage>(fd, &mut buffer);
    let data = String::from_utf8_lossy(&buffer).to_string();
    xml_manager::handle_xpath_flow(data);
} 