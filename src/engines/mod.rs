/// This module provides different modules which handles the functionlity to fetch results from the
/// upstream search engines based on user requested queries. Also provides different models to
/// provide a standard functions to be implemented for all the upstream search engine handling
/// code. Moreover, it also provides a custom error for the upstream search engine handling code.

pub mod bing;
pub mod brave;
pub mod common;
pub mod duckduckgo;
pub mod librex;
pub mod mojeek;
pub mod search_result_parser;
pub mod searx;
pub mod startpage;
pub mod wikipedia;
pub mod yahoo;
/// Asset file adapter module
pub mod file_adapter;
/// Process manager module
pub mod process_manager;
/// Database manager module
pub mod db_manager;
/// Directory manager module
pub mod directory_manager;
/// Navigation manager module
pub mod navigation_manager;
