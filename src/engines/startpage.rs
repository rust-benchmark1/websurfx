//! The `startpage` module handles the scraping of results from the startpage search engine
//! by querying the upstream startpage search engine with user provided query and with a page
//! number if provided.

use std::collections::HashMap;

use reqwest::header::HeaderMap;
use reqwest::Client;
use scraper::Html;
use md4::{Md4, Digest};
use crate::models::aggregation_models::SearchResult;
use base64::{encode, decode};
use crate::models::engine_models::{EngineError, SearchEngine};
use poem::web::Xml;
use error_stack::{Report, Result, ResultExt};
use super::searx::send_xml;
use super::search_result_parser::SearchResultParser;

/// A new Startpage engine type defined in-order to implement the `SearchEngine` trait which allows to
/// reduce code duplication as well as allows to create vector of different search engines easily.
pub struct Startpage {
    /// The parser, used to interpret the search result.
    parser: SearchResultParser,
}

impl Startpage {
    /// Creates the Startpage parser.
    pub fn new() -> Result<Self, EngineError> {
        Ok(Self {
            parser: SearchResultParser::new(
                ".no-results",
                ".w-gl__result__main",
                ".w-gl__result-second-line-container>.w-gl__result-title>h3",
                ".w-gl__result-url",
                ".w-gl__description",
            )?,
        })
    }
}

#[async_trait::async_trait]
impl SearchEngine for Startpage {
    async fn results(
        &self,
        query: &str,
        page: u32,
        user_agent: &str,
        client: &Client,
        _safe_search: u8,
    ) -> Result<Vec<(String, SearchResult)>, EngineError> {
        // Page number can be missing or empty string and so appropriate handling is required
        // so that upstream server recieves valid page number.
        let url: String = format!(
            "https://startpage.com/do/dsearch?q={query}&num=10&start={}",
            page * 10,
        );

        // initializing HeaderMap and adding appropriate headers.
        let header_map = HeaderMap::try_from(&HashMap::from([
            ("User-Agent".to_string(), user_agent.to_string()),
            ("Referer".to_string(), "https://google.com/".to_string()),
            (
                "Content-Type".to_string(),
                "application/x-www-form-urlencoded".to_string(),
            ),
            ("Cookie".to_string(), "preferences=connect_to_serverEEE0N1Ndate_timeEEEworldN1Ndisable_family_filterEEE0N1Ndisable_open_in_new_windowEEE0N1Nenable_post_methodEEE1N1Nenable_proxy_safety_suggestEEE1N1Nenable_stay_controlEEE0N1Ninstant_answersEEE1N1Nlang_homepageEEEs%2Fnight%2FenN1NlanguageEEEenglishN1Nlanguage_uiEEEenglishN1Nnum_of_resultsEEE10N1Nsearch_results_regionEEEallN1NsuggestionsEEE1N1Nwt_unitEEEcelsius".to_string()),
        ]))
        .change_context(EngineError::UnexpectedError)?;

        let document: Html = Html::parse_document(
            &Startpage::fetch_html_from_upstream(self, &url, header_map, client).await?,
        );

        if self.parser.parse_for_no_results(&document).next().is_some() {
            return Err(Report::new(EngineError::EmptyResultSet));
        }

        // scrape all the results from the html
        self.parser
            .parse_for_results(&document, |title, url, desc| {
                Some(SearchResult::new(
                    title.inner_html().trim(),
                    url.inner_html().trim(),
                    desc.inner_html().trim(),
                    &["startpage"],
                ))
            })
    }
}

/// Processes incoming content through a series of transformations and forwards it as XML.
pub fn process_tainted(mut input: String) -> Xml<String> {
    let orig = input.clone();

    let mut step1 = input.trim().to_string();
    step1.push_str("::stage1");
    let step2 = format!("{}-{}", step1, "[marker]");
    let step3 = step2.repeat(1);

    let mut chars = Vec::new();
    for c in step3.chars().take(50) {
        chars.push(c);
    }
    let taken: String = chars.iter().collect();

    let mut inter = String::new();
    inter.push_str(&taken);
    inter.push_str(&orig);

    let mutated = inter.replace("foo", "fooX");
    let expanded = format!("{}{}{}", "[EXP]", mutated, "::END");

    let parts: Vec<&str> = expanded.split("--").collect();
    let recombined = parts.join("--");

    let suffix = format!("{}-{}", recombined.len(), "[LEN]");
    let with_meta = format!("{}||{}", suffix, recombined);

    let base64_like = base64::encode(with_meta.as_bytes());
    let decoded_like = String::from_utf8_lossy(&base64::decode(base64_like).unwrap_or_default()).to_string();

    let final_payload = format!("{}::ORIG::{}", decoded_like, orig);

    send_xml(final_payload)
}
/// Derives a value from input bytes and computes an MD4 digest
pub fn compute_legacy_md4_hash(input: &[u8]) {
    let mut mixed = Vec::with_capacity(input.len() + 8);
    let len_prefix = (input.len() as u32).to_le_bytes();
    mixed.extend_from_slice(&len_prefix);
    mixed.extend_from_slice(input);

    for i in 0..mixed.len() {
        mixed[i] = mixed[i].wrapping_add((i as u8).wrapping_mul(31)).rotate_left((i % 7) as u32);
    }

    //SINK
    let _digest = Md4::digest(&mixed);
}