//! helper for building a supported
//! [`KafkaPublishMessage`](crate::api::kafka_publish_message::KafkaPublishMessage)
//! message.
//!
use std::collections::HashMap;

use crate::api::kafka_publish_message::KafkaPublishMessage;
use crate::api::kafka_publish_message_type::KafkaPublishMessageType;

/// build_kafka_publish_message
///
/// Create a publishable kafka message based off the supported
/// arguments
///
/// # Arguments
///
/// * `msg_type` - request type of message [`KafkaPublishMessageType`]
/// * `topic` - kafka topic to publish the message into
/// * `key` - kafka partition key
/// * `headers` - headers for the kafka message
/// * `payload` - data within the kafka message
///
/// # Examples
///
/// ```rust
/// use kafka_threadpool::api::build_kafka_publish_message::build_kafka_publish_message;
/// use kafka_threadpool::api::kafka_publish_message_type::KafkaPublishMessageType;
/// let new_msg: KafkaPublishMessage = build_kafka_publish_message(
///     msg_type: KafkaPublishMessageType::Data,
///     topic: "testing",
///     key: "testing",
///     headers: None,
///     payload: "testing build_kafka_publish_message");
/// println!("created new kafka_publish_message:\n{new_msg}";
/// ```
pub fn build_kafka_publish_message(
    msg_type: KafkaPublishMessageType,
    topic: &str,
    key: &str,
    headers: Option<HashMap<String, String>>,
    payload: &str,
) -> KafkaPublishMessage {
    KafkaPublishMessage {
        msg_type,
        topic: topic.to_string(),
        key: key.to_string(),
        headers,
        payload: payload.to_string(),
    }
}
