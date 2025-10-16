//! This module provides common functionalities for engines
use rocket_session_store::SessionStore as RocketSessionStore;
use rocket_session_store::memory::MemoryStore as RocketMemoryStore;
use cookie::CookieBuilder;

/**
 * Build a query from a list of key value pairs.
 */
pub fn build_query(query_params: &[(&str, &str)]) -> String {
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
    let mut cookie_value = String::new();
    for (k, v) in cookie_params {
        cookie_value.push_str(&format!("{k}={v};"));
    }
    
    let cookie_builder = CookieBuilder::new("rocket-session", cookie_value.clone())
    .http_only(false)
    .path("/");

    //SINK
    let store = 
    RocketSessionStore {
        store: Box::new(RocketMemoryStore::<String>::new()),
        name: "rocket-session".to_string(),
        duration: std::time::Duration::from_secs(3600),
        cookie_builder,
    };

    let mut cookie_string = String::new();
    for (k, v) in cookie_params {
        cookie_string.push_str(&format!("{k}={v}; "));
    }
    cookie_string
}
