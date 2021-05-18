mod config_parser;
mod kafka_consumer;
mod publish_message;

use clap::{App, Arg};
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::Message;
use rdkafka::topic_partition_list::TopicPartitionList;


use config_parser::{parse_config, YmlConfig};
use kafka_consumer::{consume_message, get_consumer, CustomContext};
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
        None => panic!("Unsupported type")
    };
    
    println!("{:?}", config);
    let consumer_config = config.consumers;
    let ms_config = config.meilisearch;
    let consumer: StreamConsumer<CustomContext> = get_consumer(consumer_config).await;

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

                publish_payload(payload, document).await;
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }

    // TODO: consuming logic should be selected by data source type
    // let test = consume_message(&consumer_config.brokers, &consumer_config.group_id, topics, &consumer_config.document).await;

    
}
