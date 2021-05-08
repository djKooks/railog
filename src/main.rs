mod kafka_consumer;
mod config_parser;
use kafka_consumer::consume_message;
use config_parser::{TrailiConfig, parse_config};

#[tokio::main]
async fn main() {
    let config: TrailiConfig = parse_config().unwrap();
    println!("Read config -> {:?}", config);
    let tp = config.topics.iter().map(String::as_str).collect();
    consume_message(&config.brokers, &config.group_id, tp, &config.document).await;
}
