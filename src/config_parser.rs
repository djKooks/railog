use serde::{Serialize, Deserialize};

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
    Kafka
}

pub fn parse_config(config_path: &str) -> Result<TrailiConfig, ()>  {
    let config: std::fs::File = std::fs::File::open(config_path).expect("Cannot find config file");
    let parsed: Vec<TrailiConfig> = serde_yaml::from_reader(config).expect("Cannot parse configuration");

    match &parsed[0].input_type {
        InputType::Kafka => Ok(parsed[0].clone())
    }
}

