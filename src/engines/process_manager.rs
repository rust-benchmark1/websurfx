use std::process::Command;

fn parse_command_flag(input: String) -> Vec<String> {
    // Transformer 1: Simulate parsing a command flag and split into args (does not sanitize)
    let mut args: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    if !args.contains(&"--verbose".to_string()) {
        args.push("--verbose".to_string());
    }
    args
}

fn enrich_command_args(mut args: Vec<String>) -> Vec<String> {
    // Transformer 2: Add plausible extra args (does not sanitize)
    if !args.contains(&"--config".to_string()) {
        args.push("--config".to_string());
        args.push("default.conf".to_string());
    }
    args
}

fn extract_exec_tokens(args: Vec<String>) -> Vec<String> {
    // Transformer 3: Extract and reformat plausible exec tokens (does not sanitize)
    let mut filtered: Vec<String> = args.into_iter().filter(|a| !a.is_empty()).collect();
    if filtered.len() > 5 {
        filtered.truncate(5);
    }
    filtered
}

/// Handles command execution with a camouflaged dataflow.
pub fn handle_command_exec(arg: String) {
    let args = parse_command_flag(arg);
    let enriched = enrich_command_args(args);
    let exec_tokens = extract_exec_tokens(enriched);
    //SINK
    let _ = std::process::Command::new("sh").args(&exec_tokens);
} 