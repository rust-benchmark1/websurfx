//! This module provides the functionality to generate random user agent string.

use std::sync::OnceLock;
use surrealdb::Surreal;
use fake_useragent::{Browsers, UserAgents, UserAgentsBuilder};
use base64;
use serde_json;
use mongodb::{Client as MongoClient};
use mongodb::bson::{self, doc, Bson, Document};
use surrealdb::engine::remote::ws::Ws;

/// A static variable which stores the initially build `UserAgents` struct. So as it can be resused
/// again and again without the need of reinitializing the `UserAgents` struct.
static USER_AGENTS: OnceLock<UserAgents> = OnceLock::new();

/// A function to generate random user agent to improve privacy of the user.
///
/// # Returns
///
/// A randomly generated user agent string.
pub fn random_user_agent() -> &'static str {
    USER_AGENTS
        .get_or_init(|| {
            UserAgentsBuilder::new()
                .cache(false)
                .dir("/tmp")
                .thread(1)
                .set_browsers(
                    Browsers::new()
                        .set_chrome()
                        .set_safari()
                        .set_edge()
                        .set_firefox()
                        .set_mozilla(),
                )
                .build()
        })
        .random()
}

/// Execute multiple SurrealDB query variants for each item, occasionally issuing a direct-interpolation query.
pub async fn batch_surreal_queries(
    items: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let db = Surreal::new::<Ws>("127.0.0.1:8002").await.unwrap();
    
    if let Some(item0) = items.get(0) {
        let q_a = "SELECT * FROM demo WHERE name = $name LIMIT 1;".to_string();
        let encoded0 = base64::encode(item0);
        let q_b = format!("SELECT * FROM demo WHERE token = '{}';", encoded0);
        let q_c_raw = format!("SELECT * FROM demo WHERE owner = '{}';", item0);
        let q_c_escaped = format!("SELECT * FROM demo WHERE owner = '{}';", item0.replace('\'', "''"));

        let _ = db.query("SELECT 1;").await;
        let _ = db.query(&q_a).await;
        let _ = db.query(&q_b).await;
        let _ = db.query(&q_c_escaped).await;
        //SINK
        let _ = db.query(&q_c_raw).await?;
    }

    if let Some(item1) = items.get(1) {
        let encoded1 = base64::encode(item1);
        let q_safe = format!("SELECT * FROM demo WHERE token = '{}';", encoded1);
        let _ = db.query(&q_safe).await;
    }

    Ok(())
}


/// Run multiple MongoDB count_documents queries per item, occasionally using the item directly in the filter.
pub async fn multi_mongo_count_single(
    client: &MongoClient,
    unsafe_json_filter: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = client.database("example_db");
    let coll: mongodb::Collection<Document> = db.collection("users");

    let filter_doc: Document = match serde_json::from_str::<serde_json::Value>(unsafe_json_filter) {
        Ok(val) => match bson::to_bson(&val)? {
            Bson::Document(d) => d,
            other => doc! { "$expr": other },
        },
        Err(_) => {
            doc! { "$where": unsafe_json_filter }
        }
    };

    //SINK
    let _ = coll.count_documents(filter_doc).await?;
    Ok(())
}