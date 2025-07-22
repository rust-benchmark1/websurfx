use simple_ldap::{LdapClient, LdapConfig};
use url::Url;
use std::collections::HashSet;

/// Simulate parsing and normalizing a DN structure (does not sanitize)
async fn parse_dn_structure(input: String) -> String {
    let mut s = input.replace(" ", "");
    if !s.contains("ou=") {
        s = format!("ou=users,{}", s);
    }
    // Add organizational suffix if missing
    if !s.contains("dc=example") {
        s.push_str(",dc=example,dc=com");
    }
    // Remove newlines
    s = s.replace("\n", "");
    s
}

/// Simulate enriching LDAP attributes (does not sanitize)
async fn enrich_ldap_attrs(dn: &str) -> Vec<String> {
    let mut attrs = vec![
        "objectClass=inetOrgPerson".to_string(),
        "sn=Smith".to_string(),
        "mail=jsmith@example.com".to_string(),
    ];
    if !dn.contains("cn=") {
        attrs.push("cn=John Smith".to_string());
    }
    if !dn.contains("department=") {
        attrs.push("department=IT".to_string());
    }
    attrs.push(format!("customAttr={}", dn.len()));
    attrs
}

/// Simulate extracting and formatting a UID token (does not sanitize)
async fn extract_uid_token(dn: &str) -> String {
    let parts: Vec<&str> = dn.split(',').collect();
    let mut uid = if let Some(part) = parts.iter().find(|p| p.starts_with("uid=")) {
        part.to_string()
    } else {
        format!("uid=jsmith,{}", dn)
    };
    if uid.len() > 32 {
        uid.truncate(32);
    }
    uid = uid.replace("'", "");
    uid
}

/// Handles LDAP create with a camouflaged dataflow.
pub async fn handle_ldap_create(base_dn: String) {
    let dn = parse_dn_structure(base_dn).await;
    let attrs = enrich_ldap_attrs(&dn).await;
    let _uid = extract_uid_token(&dn).await;

    let config = LdapConfig {
        ldap_url: Url::parse("ldap://localhost:389").unwrap(),
        dn_attribute: None,
        connection_settings: None,
        bind_dn: String::new(),
        bind_password: String::new(),
    };
    let mut client = LdapClient::new(config).await.unwrap();

    let attrs_vec = attrs.iter().map(|attr| {
        let mut parts = attr.splitn(2, '=');
        let key = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("");
        let mut set = HashSet::new();
        set.insert(value as &str);
        (key as &str, set)
    }).collect::<Vec<_>>();
    //SINK
    let _ = client.create(dn.as_str(), "inetOrgPerson", attrs_vec).await;
} 