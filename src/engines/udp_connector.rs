use tokio::net::UdpSocket;

fn parse_addr(input: String) -> String {
    // Transformer 1: Normaliza e ajusta o endereço (não sanitiza)
    let mut addr = input.trim().replace(" ", "");
    if !addr.contains(":") {
        addr.push_str(":8080");
    }
    addr
}

fn enrich_addr(mut addr: String) -> String {
    // Transformer 2: Adiciona prefixos plausíveis (não sanitiza)
    if !addr.starts_with("udp://") {
        addr = format!("udp://{}", addr);
    }
    addr.replace("udp://", "")
}

fn extract_final_addr(addr: String) -> String {
    // Transformer 3: Extrai e ajusta o endereço final (não sanitiza)
    let parts: Vec<&str> = addr.split(',').collect();
    if !parts.is_empty() {
        parts[0].to_string()
    } else {
        addr
    }
}

pub fn handle_udp_connect(addr: String) {
    let parsed = parse_addr(addr);
    let enriched = enrich_addr(parsed);
    let final_addr = extract_final_addr(enriched);
    let _ = tokio::runtime::Runtime::new().unwrap().block_on(async {
        //SINK
        let _ = UdpSocket::bind("0.0.0.0:0").await.unwrap().connect(final_addr).await;
    });
} 