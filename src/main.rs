mod kafka_consumer;
mod config_parser;

use clap::{App, Arg};
use kafka_consumer::consume_message;
use config_parser::{TrailiConfig, parse_config};

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
                .required(true)
        ).get_matches();

    let config_file_path = matches.value_of("config").unwrap();
    let config: TrailiConfig = parse_config(config_file_path).unwrap();
    
    println!("Read config -> {:?}", config);

    let tp = config.topics.iter().map(String::as_str).collect();
    consume_message(&config.brokers, &config.group_id, tp, &config.document).await;
}
