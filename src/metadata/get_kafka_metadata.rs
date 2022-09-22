//! Get metadata from kafka for a single topic or all topics
//!
use log::info;

use rdkafka::consumer::BaseConsumer;
use rdkafka::consumer::Consumer;

use crate::config::kafka_client_config::KafkaClientConfig;

/// get_kafka_metadata
///
/// Get metadata from the kafka cluster
///
/// [Original source metadata.rs](https://github.com/fede1024/rust-rdkafka/blob/master/examples/metadata.rs)
///
/// # Arguments
///
/// * `config` - initialized [`KafkaClientConfig`](crate::config::KafkaClientConfig)
/// * `consumer` - initialized [`BaseConsumer`](rdkafka::consumer::BaseConsumer) used to
/// fetch the metadata from the kafka cluster
/// * `fetch_offsets` - when ``true`` this function will count the total number
/// of messages in each topic
/// * `topic` - If set, only get the details for that specific topic if set to ``None``
/// get details for all topics
///
pub fn get_kafka_metadata(
    config: &KafkaClientConfig,
    consumer: BaseConsumer,
    fetch_offsets: bool,
    topic: Option<&str>,
) {
    info!("getting metadata config={config}");
    let fetch_timeout = std::time::Duration::from_millis(30000);
    let metadata = consumer
        // https://docs.rs/rdkafka/latest/rdkafka/consumer/struct.BaseConsumer.html#method.fetch_metadata
        .fetch_metadata(topic, fetch_timeout)
        .expect("Failed to fetch metadata");

    let mut broker_listing: String = String::from("");
    for broker in metadata.brokers() {
        let cur_broker = format!(
            "broker.id={} address={}:{} ",
            broker.id(),
            broker.host(),
            broker.port()
        );
        broker_listing = broker_listing + &cur_broker;
    }

    info!(
        "cluster info brokers={} num_topics={} \
        {broker_listing}",
        metadata.brokers().len(),
        metadata.topics().len()
    );

    let mut message_count = 0;
    for found_topic in metadata.topics() {
        info!("topic={} err={:?}", found_topic.name(), found_topic.error());
        for partition in found_topic.partitions() {
            info!(
                "topic={} - partition={} \
                leader={} replicas={:?} \
                ISR={:?} err={:?}",
                found_topic.name(),
                partition.id(),
                partition.leader(),
                partition.replicas(),
                partition.isr(),
                partition.error()
            );
            if fetch_offsets {
                let (low, high) = consumer
                    .fetch_watermarks(
                        found_topic.name(),
                        partition.id(),
                        std::time::Duration::from_secs(1),
                    )
                    .unwrap_or((-1, -1));
                info!(
                    "topic={} - watermark low={} \
                    high={} (difference={})",
                    found_topic.name(),
                    low,
                    high,
                    high - low
                );
                message_count += high - low;
            }
        }
        if fetch_offsets {
            info!(
                "topic={} - message offset={message_count}",
                found_topic.name()
            );
        }
    }
}
