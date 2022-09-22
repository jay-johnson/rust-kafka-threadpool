//! class definition and implementation for
//! [`KafkaPublishMessage`](crate::api::kafka_publish_message_type::KafkaPublishMessage)
//!
use std::collections::HashMap;

use crate::api::kafka_publish_message_type::KafkaPublishMessageType;

/// KafkaPublishMessage
///
/// Kafka publish message abstraction allowing for holding
/// common message properties:
///
/// - metadata
/// - messaging info
/// - headers
///
#[derive(Clone)]
pub struct KafkaPublishMessage {
    pub msg_type: KafkaPublishMessageType,
    pub topic: String,
    pub key: String,
    pub headers: Option<HashMap<String, String>>,
    pub payload: String,
}

impl Default for KafkaPublishMessage {
    fn default() -> Self {
        Self::new()
    }
}

impl KafkaPublishMessage {
    /// new
    ///
    /// Create a
    /// [`KafkaPublishMessage`](crate::api::kafka_publish_message_type::KafkaPublishMessage)
    /// from defaults
    ///
    /// By default:
    ///
    /// ``self.msg_type`` = ``KafkaPublishMessageType::Data``
    ///
    pub fn new() -> Self {
        KafkaPublishMessage {
            msg_type: KafkaPublishMessageType::Data,
            topic: "".to_string(),
            key: "".to_string(),
            headers: None,
            payload: "".to_string(),
        }
    }

    /// new_from
    ///
    /// Create a
    /// [`KafkaPublishMessage`](crate::api::kafka_publish_message_type::KafkaPublishMessage)
    /// from arguments
    ///
    /// # Arguments
    ///
    /// * `msg_type` - type of message
    /// * `topic` - kafka topic
    /// * `key` - kafka partition key
    /// * `headers` - key/value headers to add during publishing
    /// * `payload` - data for this message
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use kafka_threadpool::api::kafka_publish_message::KafkaPublishMessage;
    /// use kafka_threadpool::api::kafka_publish_message_type::KafkaPublishMessageType;
    /// let mut hmap: HashMap<String, String> = HashMap::new();
    /// hmap.insert("header 1".to_string(), "value 1".to_string());
    /// let msg: KafkaPublishMessage = KafkaPublishMessage::new_from(
    ///     KafkaPublishMessageType::Data,
    ///     "testing",
    ///     "custom-key",
    ///     Some(hmap),
    ///     "payload");
    /// println!("msg: {msg}");
    /// ```
    pub fn new_from(
        msg_type: KafkaPublishMessageType,
        topic: &str,
        key: &str,
        headers: Option<HashMap<String, String>>,
        payload: &str,
    ) -> Self {
        KafkaPublishMessage {
            msg_type,
            topic: topic.to_string(),
            key: key.to_string(),
            headers,
            payload: payload.to_string(),
        }
    }
}

impl std::fmt::Debug for KafkaPublishMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.msg_type != KafkaPublishMessageType::Sensitive {
            write!(
                f,
                "DEBUG KafkaPublishMessage \
                type={:?} \
                topic={} \
                key={} \
                headers={:?} \
                payload={}",
                self.msg_type, self.topic, self.key, self.headers, self.payload
            )
        } else {
            write!(
                f,
                "DEBUG SENSITIVE KafkaPublishMessage \
                type={:?} \
                topic={} \
                key={} \
                headers={:?}",
                self.msg_type, self.topic, self.key, self.headers
            )
        }
    }
}

impl std::fmt::Display for KafkaPublishMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.msg_type != KafkaPublishMessageType::Sensitive {
            write!(
                f,
                "KafkaPublishMessage \
                type={:?} \
                topic={} \
                key={} \
                headers={:?} \
                payload={}",
                self.msg_type, self.topic, self.key, self.headers, self.payload,
            )
        } else {
            write!(
                f,
                "SENSITIVE KafkaPublishMessage \
                type={:?} \
                topic={} \
                key={} \
                headers={:?}",
                self.msg_type, self.topic, self.key, self.headers
            )
        }
    }
}
