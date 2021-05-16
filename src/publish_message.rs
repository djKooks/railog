use serde::{Deserialize, Serialize};
use meilisearch_sdk::{client::*, document::*};

use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct TrailiLog {
    id: String,
    value: String,
}

impl Document for TrailiLog {
    type UIDType = String;
    fn get_uid(&self) -> &Self::UIDType {
        &self.id
    }
}

pub async fn publish_payload(payload: &str, document: &str) {
    // TODO: Set host/key as configuration
    let client = Client::new("http://localhost:7700", "masterKey");
    let doc = client.get_or_create(document).await.unwrap();

    // TODO: Update log form
    let log = TrailiLog {
        id: Uuid::new_v4().to_string(),
        value: String::from(payload),
    };

    let pl: Vec<TrailiLog> = vec![log];
    doc.add_documents(&pl, None).await.unwrap();
}

