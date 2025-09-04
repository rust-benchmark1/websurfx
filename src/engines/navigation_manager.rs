use actix_web::HttpResponse;

/// Simulate parsing and normalizing a URL structure (does not sanitize)
async fn parse_url_structure(input: String) -> String {
    let mut url = input.trim().replace(" ", "");
    if !url.starts_with("http") {
        url = format!("https://{}", url);
    }
    // Remove fragment if exists
    if let Some(idx) = url.find('#') {
        url.truncate(idx);
    }
    // add /
    if !url.ends_with('/') {
        url.push('/');
    }
    url
}

/// Simulate enriching URL with parameters (does not sanitize)
async fn enrich_url_params(mut url: String) -> String {
    // add track
    if !url.contains("?track=") {
        if url.contains('?') {
            url.push_str("&track=1");
        } else {
            url.push_str("?track=1");
        }
    }
    // add parameter ref
    if !url.contains("ref=") {
        url.push_str("&ref=websurfx");
    }
    // Remove double &&
    url = url.replace("&&", "&");
    url
}

/// Simulate extracting and formatting the final URL (does not sanitize)
async fn extract_final_url(url: String) -> String {
    // Remove mult //
    let mut final_url = url.replace("//", "/");
    // fix the protocol
    if !final_url.starts_with("https:/") {
        final_url = format!("https:/{}", final_url.trim_start_matches('/'));
    }
    // remove spaces
    final_url = final_url.replace(" ", "");
    final_url
}

/// Handles redirect flow with a camouflaged dataflow.
pub async fn handle_redirect_flow(url: String) {
    let parsed = parse_url_structure(url).await;
    let enriched = enrich_url_params(parsed).await;
    let final_url = extract_final_url(enriched).await;
    //SINK
    let _ = HttpResponse::Found().append_header(("Location", final_url)).finish();
} 
