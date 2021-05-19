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

type LoggingConsumer = StreamConsumer<CustomContext>;

pub async fn get_consumer(consumer_config: KafkaConsumerConfig) -> StreamConsumer<CustomContext>  {
    let topics: Vec<&str> = consumer_config.topics.iter().map(String::as_str).collect();
    let context = CustomContext;
    let topic = &topics[..];
    let options = consumer_config.options;

    // Get kafka-client configuration by configs
    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", consumer_config.group_id)
        .set("bootstrap.servers", consumer_config.brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("allow.auto.create.topics", "true")
        .set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(topic)
        .expect("Can't subscribe to specified topics");
    
    consumer
}
