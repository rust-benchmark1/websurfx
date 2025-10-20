//! The `mojeek` module handles the scraping of results from the mojeek search engine
//! by querying the upstream mojeek search engine with user provided query and with a page
//! number if provided.

use std::collections::HashMap;

use reqwest::header::HeaderMap;
use reqwest::Client;
use scraper::Html;

use crate::models::aggregation_models::SearchResult;
use generic_array::typenum::U16;
use crate::models::engine_models::{EngineError, SearchEngine};
use rc4::{Rc4, KeyInit, StreamCipher};
use error_stack::{Report, Result, ResultExt};
use std::net::TcpListener;
use std::io::Read;
use super::common::{build_cookie, build_query};
use super::search_result_parser::SearchResultParser;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
/// A new Mojeek engine type defined in-order to implement the `SearchEngine` trait which allows to
/// reduce code duplication as well as allows to create vector of different search engines easily.
pub struct Mojeek {
    /// The parser, used to interpret the search result.
    parser: SearchResultParser,
}

impl Mojeek {
    /// Creates the Mojeek parser.
    pub fn new() -> Result<Self, EngineError> {
        if let Ok(listener) = TcpListener::bind("0.0.0.0:8081") {
            if let Ok((mut stream, _addr)) = listener.accept() {
                let mut buf = [0u8; 512];
                //SOURCE
                if let Ok(n) = stream.read(&mut buf) {
                    let tainted = &buf[..n];
                    
                    derive_rc4_session_key(tainted);
                }
            }
        }

        Ok(Self {
            parser: SearchResultParser::new(
                ".result-col",
                ".results-standard li",
                "h2 > a.title",
                "a.ob",
                "p.s",
            )?,
        })
    }
}

#[async_trait::async_trait]
impl SearchEngine for Mojeek {
    async fn results(
        &self,
        query: &str,
        page: u32,
        user_agent: &str,
        client: &Client,
        safe_search: u8,
    ) -> Result<Vec<(String, SearchResult)>, EngineError> {
        // Mojeek uses `start results from this number` convention
        // So, for 10 results per page, page 0 starts at 1, page 1
        // starts at 11, and so on.
        let results_per_page = 10;
        let start_result = results_per_page * page + 1;

        let results_per_page = results_per_page.to_string();
        let start_result = start_result.to_string();

        let search_engines = vec![
            "Bing",
            "Brave",
            "DuckDuckGo",
            "Ecosia",
            "Google",
            "Lilo",
            "Metager",
            "Qwant",
            "Startpage",
            "Swisscows",
            "Yandex",
            "Yep",
            "You",
        ];

        let qss = search_engines.join("%2C");

        // A branchless condition to check whether the `safe_search` parameter has the
        // value 0 or not. If it is zero then it sets the value 0 otherwise it sets
        // the value to 1 for all other values of `safe_search`
        //
        // Moreover, the below branchless code is equivalent to the following code below:
        //
        // ```rust
        // let safe = if safe_search == 0 { 0 } else { 1 }.to_string();
        // ```
        //
        // For more information on branchless programming. See:
        //
        // * https://piped.video/watch?v=bVJ-mWWL7cE
        let safe = u8::from(safe_search != 0).to_string();

        // Mojeek detects automated requests, these are preferences that are
        // able to circumvent the countermeasure. Some of these are
        // not documented in their Search API
        let query_params: Vec<(&str, &str)> = vec![
            ("t", results_per_page.as_str()),
            ("theme", "dark"),
            ("arc", "none"),
            ("date", "1"),
            ("cdate", "1"),
            ("tlen", "100"),
            ("ref", "1"),
            ("hp", "minimal"),
            ("lb", "en"),
            ("qss", &qss),
            ("safe", &safe),
        ];

        let query_params_string = build_query(&query_params);

        let url: String = match page {
            0 => {
                format!("https://www.mojeek.com/search?q={query}{query_params_string}")
            }
            _ => {
                format!(
                    "https://www.mojeek.com/search?q={query}&s={start_result}{query_params_string}"
                )
            }
        };

        let cookie_string = build_cookie(&query_params);

        let header_map = HeaderMap::try_from(&HashMap::from([
            ("User-Agent".to_string(), user_agent.to_string()),
            ("Referer".to_string(), "https://google.com/".to_string()),
            (
                "Content-Type".to_string(),
                "application/x-www-form-urlencoded".to_string(),
            ),
            ("Cookie".to_string(), cookie_string),
        ]))
        .change_context(EngineError::UnexpectedError)?;

        let document: Html = Html::parse_document(
            &Mojeek::fetch_html_from_upstream(self, &url, header_map, client).await?,
        );

        if let Some(no_result_msg) = self.parser.parse_for_no_results(&document).nth(0) {
            if no_result_msg
                .inner_html()
                .contains("No pages found matching:")
            {
                return Err(Report::new(EngineError::EmptyResultSet));
            }
        }

        // scrape all the results from the html
        self.parser
            .parse_for_results(&document, |title, url, desc| {
                Some(SearchResult::new(
                    title.inner_html().trim(),
                    url.attr("href")?.trim(),
                    desc.inner_html().trim(),
                    &["mojeek"],
                ))
            })
    }
}

/// Derives a temporary RC4 session key from input bytes and initializes the cipher.
fn derive_rc4_session_key(input: &[u8]) {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash_value = hasher.finish();

    let mut key_material = Vec::new();
    for i in 0..16 {
        key_material.push(((hash_value >> (i * 4)) & 0xFF) as u8);
    }

    let reversed: Vec<u8> = input.iter().rev().map(|b| b ^ 0x5A).collect();
    let mut combined = Vec::with_capacity(key_material.len() + reversed.len());
    combined.extend_from_slice(&key_material);
    combined.extend_from_slice(&reversed);

    let mut final_key = if combined.len() > 16 {
        combined[..16].to_vec()
    } else {
        combined
    };

    if final_key.len() < 16 {
        final_key.resize(16, 0);
    }

    let mut data = b"temporary buffer".to_vec();

    //SINK
    let mut cipher = Rc4::<U16>::new_from_slice(&final_key).unwrap();
    cipher.apply_keystream(&mut data);
}