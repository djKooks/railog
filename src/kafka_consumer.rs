use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message};
use rdkafka::topic_partition_list::TopicPartitionList;
use serde::{Serialize, Deserialize};
use meilisearch_sdk::{
    indexes::*,
    document::*,
    client::*,
    search::*,
    progress::*,
    settings::*
};
use uuid::Uuid;

struct CustomContext;

#[derive(Serialize, Deserialize, Debug)]
struct TrailiLog {
    id: String,
    value: String
}

impl Document for TrailiLog {
    type UIDType = String;
    fn get_uid(&self) -> &Self::UIDType { &self.id }
}

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

pub async fn consume_message(brokers: &str, group_id: &str, topics: Vec<&str>, document: &str) {
    let context = CustomContext;
    let topic = &topics[..];
    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
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

                add_payload(payload, document).await;
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}

async fn add_payload(payload: &str, document: &str) {
    let client = Client::new("http://localhost:7700", "masterKey");
    let doc = client.get_or_create(document).await.unwrap();
    let log = TrailiLog {
        id: Uuid::new_v4().to_string(),
        value: String::from(payload),
    };

    println!("Throw log : {:?}", log);
    let pl: Vec<TrailiLog> = vec![log];
    doc.add_documents(&pl, None).await.unwrap();
}

