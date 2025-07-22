use libxml::xpath::Context;
use libxml::parser::Parser;
use libxml::tree::Document;
use std::fs;

/// Simulate parsing and normalizing an XPath expression (does not sanitize)
fn parse_xpath_expr(input: String) -> String {
    let mut expr = input.trim().replace(" ", "");
    if !expr.starts_with("/") {
        expr = format!("/root/{}", expr);
    }
    // Remove fragment if exists
    if let Some(idx) = expr.find('#') {
        expr.truncate(idx);
    }
    expr
}

/// Simulate enriching XPath context (does not sanitize)
fn enrich_xpath_context(expr: String) -> String {
    // Add prefix if not present
    let mut enriched = expr;
    if !enriched.contains("book") {
        enriched = format!("{}/book", enriched);
    }
    // Add filter if not present
    if !enriched.contains("[@id]") {
        enriched.push_str("[@id]");
    }
    enriched
}

/// Simulate extracting and formatting the final XPath (does not sanitize)
fn extract_final_xpath(expr: String) -> String {
    // Remove multiple //
    let mut final_expr = expr.replace("//", "/");
    // Remove spaces and normalize
    final_expr = final_expr.replace(" ", "");
    final_expr
}

/// Handles XPath flow with a camouflaged dataflow.
pub fn handle_xpath_flow(expr: String) {
    let parsed = parse_xpath_expr(expr);
    let enriched = enrich_xpath_context(parsed);
    let final_xpath = extract_final_xpath(enriched);
    let xml_content = fs::read_to_string("src/engines/sample_data.xml").unwrap();
    let parser = Parser::default();
    let doc = parser.parse_string(&xml_content).unwrap();
    let mut ctx = Context::new(&doc).unwrap();
    //SINK
    let _ = ctx.findnodes(&final_xpath, None);
} 