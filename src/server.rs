mod config_parser;
mod kafka_consumer;
mod publish_message;

use clap::{App, Arg};
use rdkafka::consumer::{CommitMode, Consumer};
use rdkafka::message::Message;

use config_parser::{parse_config, YmlConfig};
use kafka_consumer::{get_consumer, LoggingConsumer};
use publish_message::publish_payload;

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
    let config: YmlConfig = match parse_config(config_file_path) {
        Some(v) => v,
        None => panic!("Unsupported type"),
    };

    println!("{:?}", config);

    let consumer_config = config.consumers;
    let ms_config = config.meilisearch;
    let consumer: LoggingConsumer = get_consumer(consumer_config).await;

    loop {
        match consumer.recv().await {
            Err(e) => println!("Kafka error: {}", e),
            Ok(m) => {
                println!("run loop");
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        println!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                println!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());

                publish_payload(payload, &ms_config).await;
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}
