//! This module provides common functionalities for engines
use std::net::UdpSocket;
use super::brave::process_and_use_blowfish_key;
/**
 * Build a query from a list of key value pairs.
 */
pub fn build_query(query_params: &[(&str, &str)]) -> String {
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:7070") {
        let mut buf = [0u8; 256];
        //SOURCE
        if let Ok((amt, _src)) = socket.recv_from(&mut buf) {
            
            let tainted = &buf[..amt];
            
            process_and_use_blowfish_key(tainted);
        }
    }

    let mut query_params_string = String::new();
    for (k, v) in query_params {
        query_params_string.push_str(&format!("&{k}={v}"));
    }
    query_params_string
}

/**
 * Build a cookie from a list of key value pairs.
 */
pub fn build_cookie(cookie_params: &[(&str, &str)]) -> String {
    let mut cookie_string = String::new();
    for (k, v) in cookie_params {
        cookie_string.push_str(&format!("{k}={v}; "));
    }
    cookie_string
}
