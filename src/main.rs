mod kafka_consumer;
use kafka_consumer::consume_message;
use rdkafka::util::get_rdkafka_version;

use clap::{App, Arg};

#[tokio::main]
async fn main() {
    let topics = ["test-topic"];
    let brokers = "localhost:9092";
    let group_id = "group";

    consume_message(brokers, group_id, &topics).await
}
