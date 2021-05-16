mod config_parser;
mod kafka_consumer;
mod publish_message;

use clap::{App, Arg};
use config_parser::{parse_config, TrailiConfig, YmlConfig};
use kafka_consumer::consume_message;

#[tokio::main]
async fn main() {
    let matches = App::new("Traili")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Traili")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Configuration file path")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let config_file_path: &str = matches.value_of("config").unwrap();

    parse_config(config_file_path)
    /*
    let config: YmlConfig = parse_config(config_file_path).unwrap();

    println!("Read config -> {:?}", config);

    let topics: Vec<&str> = config.topics.iter().map(String::as_str).collect();

    // TODO: consuming logic should be selected by data source type
    consume_message(&config.brokers, &config.group_id, topics, &config.document).await;
    */
}
