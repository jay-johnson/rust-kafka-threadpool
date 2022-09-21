//! Publish a [`KafkaPublishMessage`](crate::api::kafka_publish_message)
//! to a Kafka topic

use rdkafka::message::OwnedHeaders;
use rdkafka::producer::FutureProducer;
use rdkafka::producer::FutureRecord;
use std::collections::HashMap;

use crate::api::kafka_publish_message::KafkaPublishMessage;

/// now()
///
/// helper for setting a message timestamp
///
fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .try_into()
        .unwrap()
}

/// convert_hashmap_headers_to_ownedheaders
///
/// Internal method to serialize the public
/// [`KafkaPublishMessage.headers`](crate::api::kafka_publish_message::KafkaPublishMessage)
/// into an [`rdkafka::message::OwnedHeaders`](rdkafka::message::OwnedHeaders) before
/// publishing
///
/// Created after finding this stack overflow:
/// https://stackoverflow.com/questions/63015654/borrow-and-use-of-moved-value
///
/// # Arguments
///
/// * `hmap` - HashMap containing key/value pair of Strings to convert
/// * `owned_headers` - initialized and mutable
/// [`rdkafka::message::OwnedHeaders`](rdkafka::message::OwnedHeaders)
/// for storing headers that are compliant with ``rdkafka``
///
/// # Returns
///
/// [`rdkafka::message::OwnedHeaders`](rdkafka::message::OwnedHeaders) containing
/// all key/value pairs from ``hmap``
///
pub fn convert_hashmap_headers_to_ownedheaders(
    hmap: HashMap<String, String>,
    mut owned_headers: OwnedHeaders,
) -> OwnedHeaders {
    for (k, v) in hmap.iter() {
        owned_headers = owned_headers.add(k, &v);
    }
    owned_headers
}

/// publish_message
///
/// Worker threads publish messages to kafka using this method
///
/// This function publishes
/// [`KafkaPublishMessage`](crate::api::kafka_publish_message) where the ``msg_type``
/// (of type: [`KafkaPublishMessageType`](crate::api::kafka_publish_message_type)) is set to
/// ``Data`` or ``Sensitive``
///
/// # Arguments
///
/// * `label` - calling thread's logging label
/// * `producer` - initialized and connected
/// [`rdkafka::producer::FutureProducer`](rdkafka::producer::FutureProducer)
/// for publishing messages
/// * `msg` - initialized
/// [`KafkaPublishMessage`](crate::api::kafka_publish_message) containing
/// all routing, metadata and payload information for the message
///
pub async fn publish_message(
    producer: &FutureProducer,
    msg: &KafkaPublishMessage,
    owned_headers: &OwnedHeaders,
) -> i32 {
    let (delivery_status, _id) = producer
        .send_result(
            FutureRecord::to(&msg.topic)
                .payload(&msg.payload)
                .key(&msg.key)
                .headers(owned_headers.to_owned())
                .timestamp(now()),
        )
        .unwrap()
        .await
        .unwrap()
        .unwrap();
    delivery_status
}
