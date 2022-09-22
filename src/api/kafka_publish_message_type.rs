//! enum for supported message types with the ``kafka_threadpool``

/// KafkaClientMessageType
///
/// Supported types of [`KafkaClientMessage`] that allows for each
/// thread to handle the same way.
///
/// - ``Data`` - normal pub/sub message without
/// any payload logging restrictions
/// - ``Shutdown`` - graceful thread shutdown signal message
/// - ``LogBrokerDetails`` - when a thread encounters this message type
/// it will log all the metadata and connectivity information
/// - ``LogBrokerTopicDetails`` - when a thread encounters this message type
/// it will log the broker's metadata and connectivity information
/// - ``Sensitive`` - when a thread encounters this message type
/// it will not verbosely log the message payload and is processed like
/// a normal ``Data`` message type
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KafkaPublishMessageType {
    Data,
    Shutdown,
    LogBrokerDetails,
    LogBrokerTopicDetails,
    Sensitive,
}

// https://users.rust-lang.org/t/derive-default-for-enum-non-only-struct/44046
impl Default for KafkaPublishMessageType {
    fn default() -> Self {
        KafkaPublishMessageType::Data
    }
}
