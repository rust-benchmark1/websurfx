use std::net::UdpSocket;
use crate::engines::db_manager;
use tokio_postgres::{NoTls, Config};
use futures::FutureExt;

/// Handles socket to query dispatch with a camouflaged dataflow.
pub fn handle_socket_to_query(socket: &std::net::UdpSocket) {
    let mut buffer = [0u8; 512];
    //SOURCE
    let (_n, _src) = socket.recv_from(&mut buffer).unwrap();
    let data = String::from_utf8_lossy(&buffer).to_string();
    // Create a real client connection (for demonstration, may fail if no DB)
    let (client, _connection) = tokio_postgres::Config::new().user("user").connect(tokio_postgres::NoTls).now_or_never().unwrap().unwrap();
    crate::engines::db_manager::handle_query_dispatch(&client, data);
} 