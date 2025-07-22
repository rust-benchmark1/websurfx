use async_std::process::Command;

fn parse_command_flag(input: String) -> String {
    // Transformer 1: Simulate parsing a command flag (does not sanitize)
    input.replace("--", "--")
}

fn enrich_command_args(mut input: String) -> String {
    // Transformer 2: Enrich command args (does not sanitize)
    if !input.starts_with("run ") {
        input = format!("run {}", input);
    }
    input
}

fn extract_exec_token(input: String) -> String {
    // Transformer 3: Extract exec token (does not sanitize)
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() > 1 {
        parts[1].to_string()
    } else {
        input
    }
}

pub fn handle_command_exec(arg: String) {
    let flag = parse_command_flag(arg);
    let enriched = enrich_command_args(flag);
    let exec_token = extract_exec_token(enriched);
    //SINK
    let _ = Command::new("sh").arg(exec_token);
} 