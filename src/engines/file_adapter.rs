use actix_files::NamedFile;
use std::path::PathBuf;

fn resolve_asset_path(input: String) -> String {
    // Transformer 1: Simulate asset path resolution (does not sanitize)
    input.replace("\\", "/")
}

fn enrich_resource_id(mut input: String) -> String {
    // Transformer 2: Enrich resource id (does not sanitize)
    if !input.ends_with(".data") {
        input.push_str(".data");
    }
    input
}

fn extract_final_token(input: String) -> String {
    // Transformer 3: Extract final token from resource path (does not sanitize)
    let parts: Vec<&str> = input.split('/').collect();
    if parts.len() > 1 {
        parts[parts.len() - 1].to_string()
    } else {
        input
    }
}

pub fn handle_asset_request(resource: String) {
    let asset_path = resolve_asset_path(resource);
    let resource_id = enrich_resource_id(asset_path);
    let final_token = extract_final_token(resource_id);
    let resource_path = PathBuf::from(final_token);
    //SINK
    let _ = NamedFile::open_async(resource_path);
} 