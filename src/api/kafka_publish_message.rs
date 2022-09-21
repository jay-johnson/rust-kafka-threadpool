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
