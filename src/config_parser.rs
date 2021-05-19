use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::publish_message::MeiliSearchConfig;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct YmlConfig {
    pub meilisearch: MeiliSearchConfig,
    pub consumers: KafkaConsumerConfig,
}

/// TODO: Make config for multiple input types...
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KafkaConsumerConfig {
    input_type: InputType,
    pub brokers: String,
    pub topics: Vec<String>,
    pub group_id: String,
    pub log_level: String,
    pub options: HashMap<String, String>,
}

/// TODO: Support variable types...
#[serde(rename_all = "snake_case")]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum InputType {
    Kafka,
}

pub fn parse_config(config_path: &str) -> Option<YmlConfig> {
    let config: std::fs::File = std::fs::File::open(config_path).expect("Cannot find config file");
    let parsed: YmlConfig = serde_yaml::from_reader(config).expect("Cannot parse configuration");

    let input_source = &parsed.consumers;

    // TODO: Only kafka type is supported yet...
    match input_source.input_type {
        InputType::Kafka => Some(parsed.clone()),
        _ => None,
    }
}

#[test]
fn parse_test() {
    let config: YmlConfig = parse_config("resources/sample_config.yml").unwrap();
    let consumer_config = config.consumers;
    let ms_config = config.meilisearch;

    assert_eq!(consumer_config.brokers, "127.0.0.1:9092,127.0.0.2:9092");
    assert_eq!(consumer_config.group_id, "group");
    assert_eq!(consumer_config.topics, &["test-topic"]);

    assert_eq!(ms_config.host, "http://localhost:7700");
    assert_eq!(ms_config.master_key, "masterKey");
    assert_eq!(ms_config.document, "traili");
}
