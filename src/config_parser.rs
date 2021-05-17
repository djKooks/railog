use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct YmlConfig {
    meilisearch: MeiliSearchConfig,
    consumers: TrailiConfig
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeiliSearchConfig {
    host: String,
    master_key: String
}

/// TODO: Make config for multiple input types...
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrailiConfig {
    input_type: InputType,
    pub brokers: String,
    pub topics: Vec<String>,
    pub group_id: String,
    pub document: String,
}

/// TODO: Support variable types...
#[serde(rename_all = "snake_case")]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum InputType {
    Kafka,
}

pub fn parse_config(config_path: &str) -> Option<YmlConfig, > {
    let config: std::fs::File = std::fs::File::open(config_path).expect("Cannot find config file");
    let parsed: YmlConfig =
        serde_yaml::from_reader(config).expect("Cannot parse configuration");

    println!("parsed -> {:?}", parsed);

    let input_source = &parsed.consumers;
    // Only kafka type is supported yet...
    match input_source.input_type {
        InputType::Kafka => Some(parsed.clone()),
        _ => None
    }
}

#[test]
fn parse_test() {
    let config: TrailiConfig = parse_config("resources/sample_config.yml").unwrap();

    assert_eq!(config.brokers, "localhost:9092");
    assert_eq!(config.group_id, "test-group");
    assert_eq!(config.topics, &["test-topic"]);
    assert_eq!(config.document, "traili");
}
