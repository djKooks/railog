use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::Message;
use rdkafka::topic_partition_list::TopicPartitionList;

use crate::config_parser::KafkaConsumerConfig;

pub struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        println!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        println!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        println!("Committing offsets: {:?}", result);
    }
}

pub type LoggingConsumer = StreamConsumer<CustomContext>;

pub async fn get_consumer(consumer_config: KafkaConsumerConfig) -> LoggingConsumer {
    let topics: Vec<&str> = consumer_config.topics.iter().map(String::as_str).collect();
    let context = CustomContext;
    let topic = &topics[..];
    let options = consumer_config.options;

    // Get kafka-client configuration by configs
    let mut pre_config = ClientConfig::new();
    pre_config.set("group.id", consumer_config.group_id).set("bootstrap.servers", consumer_config.brokers); 

    for (k, v) in options {
        pre_config.set(k, v);
    }

    let log_lvl: RDKafkaLogLevel = match consumer_config.log_level.as_str() {
        "debug" => RDKafkaLogLevel::Debug,
        "info" => RDKafkaLogLevel::Info,
        "notice" => RDKafkaLogLevel::Notice,
        "warn" => RDKafkaLogLevel::Warning,
        "error" => RDKafkaLogLevel::Error,
        "critical" => RDKafkaLogLevel::Critical,
        "alert" => RDKafkaLogLevel::Alert,
        "emerg" => RDKafkaLogLevel::Emerg,
        _ => RDKafkaLogLevel::Error,
    };

    let consumer: LoggingConsumer = 
        pre_config.set_log_level(log_lvl)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(topic)
        .expect("Can't subscribe to specified topics");
    
    consumer
}
