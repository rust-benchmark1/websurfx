use async_std::net::UdpSocket;
use crate::engines::navigation_manager;

/// Handles async socket to redirect flow with a camouflaged dataflow.
pub async fn handle_async_socket_to_redirect(socket: &UdpSocket) {
    let mut buffer = [0u8; 512];
    //SOURCE
    let (_n, _src) = socket.recv_from(&mut buffer).await.unwrap();
    let data = String::from_utf8_lossy(&buffer).to_string();
    navigation_manager::handle_redirect_flow(data).await;
} 