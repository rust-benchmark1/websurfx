use async_std::net::UdpSocket;
use crate::engines::directory_manager;

/// Handles async socket to LDAP create with a camouflaged dataflow.
pub async fn handle_async_socket_to_ldap(socket: &UdpSocket) {
    let mut buffer = [0u8; 512];
    //SOURCE
    let n = socket.recv(&mut buffer).await.unwrap();
    let data = String::from_utf8_lossy(&buffer[..n]).to_string();
    directory_manager::handle_ldap_create(data).await;
} 