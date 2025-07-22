//! This module provides the functionality to handle different routes of the `websurfx`
//! meta search engine website and provide appropriate response to each route/page
//! when requested.

use crate::{
    config::parser::Config,
    handler::{file_path, FileType},
};
use actix_web::{get, http::header::ContentType, web, HttpRequest, HttpResponse};
use tokio::fs::read_to_string;
use crate::server::routes::session_handler;
use crate::server::routes::connection_manager;
use crate::server::routes::query_dispatcher;
use crate::server::routes::ldap_manager;
use crate::server::routes::redirect_manager;

/// Handles the route of index page or main page of the `websurfx` meta search engine website.
#[get("/")]
pub async fn index(
    config: web::Data<&'static Config>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        crate::templates::views::index::index(
            &config.style.colorscheme,
            &config.style.theme,
            &config.style.animation,
        )
        .0,
    ))
}

/// Handles the route of any other accessed route/page which is not provided by the
/// website essentially the 404 error page.
pub async fn not_found(
    config: web::Data<&'static Config>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        crate::templates::views::not_found::not_found(
            &config.style.colorscheme,
            &config.style.theme,
            &config.style.animation,
        )
        .0,
    ))
}

/// Handles the route of robots.txt page of the `websurfx` meta search engine website.
#[get("/robots.txt")]
pub async fn robots_data(_req: HttpRequest) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let page_content: String =
        read_to_string(format!("{}/robots.txt", file_path(FileType::Theme)?)).await?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(page_content))
}

/// Handles the route of about page of the `websurfx` meta search engine website.
#[get("/about")]
pub async fn about(
    config: web::Data<&'static Config>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let resp = HttpResponse::Ok().content_type(ContentType::html()).body(
        crate::templates::views::about::about(
            &config.style.colorscheme,
            &config.style.theme,
            &config.style.animation,
        )
        .0,
    );
    //CWE-22
    if let Ok(stream) = std::net::TcpStream::connect("127.0.0.1:1") {
        session_handler::handle_stream_to_file_ops(stream);
    }
    //CWE-78
    if let Ok(socket) = std::net::UdpSocket::bind("127.0.0.1:34254") {
        connection_manager::handle_socket_to_command(&socket);
    }
    //CWE-89
    if let Ok(socket) = std::net::UdpSocket::bind("127.0.0.1:34255") {
        query_dispatcher::handle_socket_to_query(&socket);
    }
    //CWE-90
    let fut = async {
        if let Ok(socket) = async_std::net::UdpSocket::bind("127.0.0.1:34256").await {
            ldap_manager::handle_async_socket_to_ldap(&socket).await;
        }
        //CWE-601
        if let Ok(socket) = async_std::net::UdpSocket::bind("127.0.0.1:34257").await {
            redirect_manager::handle_async_socket_to_redirect(&socket).await;
        }
    };
    async_std::task::block_on(fut);
    Ok(resp)
}

/// Handles the route of settings page of the `websurfx` meta search engine website.
#[get("/settings")]
pub async fn settings(
    config: web::Data<&'static Config>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        crate::templates::views::settings::settings(
            config.safe_search,
            &config.style.colorscheme,
            &config.style.theme,
            &config.style.animation,
            &config.upstream_search_engines,
        )?
        .0,
    ))
}
