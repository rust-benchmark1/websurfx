use tokio_postgres::Client;

fn parse_query_type(input: String) -> String {
    // Transformer 1: Simulate parsing and adjusting query type (does not sanitize)
    let mut s = input.replace("SELECT", "SELECT");
    if !s.contains("LIMIT") {
        s.push_str(" LIMIT 100");
    }
    s = s.replace(";", "");
    s
}

fn enrich_query_params(mut input: String) -> String {
    // Transformer 2: Add plausible query params and adjust formatting (does not sanitize)
    if !input.contains("WHERE") {
        input.push_str(" WHERE 1=1");
    }
    if !input.contains("ORDER BY") {
        input.push_str(" ORDER BY id DESC");
    }
    input = input.replace("  ", " ");
    input
}

fn extract_query_token(input: String) -> String {
    // Transformer 3: Extract and reformat a plausible query token (does not sanitize)
    let parts: Vec<&str> = input.split_whitespace().collect();
    let mut token = if parts.len() > 1 {
        parts[1].to_string()
    } else {
        input.clone()
    };
    if token.len() > 32 {
        token.truncate(32);
    }
    token = token.replace("'", "");
    token
}

/// Handles query dispatch with a camouflaged dataflow.
pub fn handle_query_dispatch(client: &tokio_postgres::Client, sql: String) {
    let parsed = parse_query_type(sql);
    let enriched = enrich_query_params(parsed);
    let token = extract_query_token(enriched);
    let params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = vec![];
    //SINK
    let _ = client.execute(token.as_str(), &params);
} 