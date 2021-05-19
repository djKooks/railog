use serde::{Deserialize, Serialize};
use meilisearch_sdk::{client::*, document::*};

use uuid::Uuid;

/// TODO: Think of format to be published in...
#[derive(Serialize, Deserialize, Debug)]
struct TrailiLog {
    id: String,
    payload: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeiliSearchConfig {
    host: String,
    master_key: String,
    pub document: String
}

impl Document for TrailiLog {
    type UIDType = String;
    fn get_uid(&self) -> &Self::UIDType {
        &self.id
    }
}

pub async fn publish_payload(payload: &str, config: &MeiliSearchConfig) {
    // TODO: Set host/key as configuration
    let client = Client::new(&config.host, &config.master_key);
    let doc = client.get_or_create(&config.document).await.unwrap();

    // TODO: Update log form
    let log = TrailiLog {
        id: Uuid::new_v4().to_string(),
        payload: String::from(payload),
    };

    let pl: Vec<TrailiLog> = vec![log];
    doc.add_documents(&pl, None).await.unwrap();
}

