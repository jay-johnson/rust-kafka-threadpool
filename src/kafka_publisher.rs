//! Clients using ``kafka_threadpool`` get a
//! [`KafkaPublisher`](crate::kafka_publisher) object with calling
//! [`start_threadpool`](crate::start_threadpool).
//!
//! The [`KafkaPublisher`](crate::kafka_publisher) is how
//! callers interface with the ``kafka_threadpool``'s
//! ``lockable work Vec`` called ``publish_msgs``
//! and can gracefully shutdown the threadpool using:
//!
//! ```rust
//! my_threadpool.shutdown().unwrap();
//! ```
//!
//!
use std::sync::Arc;
use std::sync::Mutex;

use log::info;

use crate::api::add_messages_to_locked_work_vec::add_messages_to_locked_work_vec;
use crate::api::build_kafka_publish_message::build_kafka_publish_message;
use crate::api::kafka_publish_message::KafkaPublishMessage;
use crate::api::kafka_publish_message_type::KafkaPublishMessageType;
use crate::config::kafka_client_config::KafkaClientConfig;

/// KafkaPublishMessage
///
/// API object for clients calling [`start_threadpool`]
///
/// * `config` - holds the static configuration for each
/// thread (connectivity endpoints, tls assets, etc.)
/// * `publish_msgs` - lockable work Vec that is shared
/// by any thread(s) that want to publish
/// [`KafkaPublishMessage`]
/// messages to Kafka
///
#[derive(Clone)]
pub struct KafkaPublisher {
    pub config: KafkaClientConfig,
    pub publish_msgs: Arc<Mutex<Vec<KafkaPublishMessage>>>,
}

impl KafkaPublisher {
    /// shutdown
    ///
    /// Gracefully shutdown the threadpool by
    /// sending the ``Shutdown`` control
    /// message to all worker threads
    ///
    /// # Errors
    ///
    /// Threads may get hung if something goes wrong.
    ///
    /// # Examples
    ///
    /// ```rust
    /// my_threadpool.shutdown().unwrap();
    /// ```
    ///
    pub fn shutdown(&self) -> Result<String, String> {
        let shutdown_msg_vec: Vec<KafkaPublishMessage> =
            vec![build_kafka_publish_message(
                KafkaPublishMessageType::Shutdown,
                "",
                "",
                None,
                "",
            )];
        info!("sending shutdown msg");
        match add_messages_to_locked_work_vec(
            &self.publish_msgs,
            shutdown_msg_vec,
        ) {
            Ok(_) => Ok("shutdown started".to_string()),
            Err(e) => Err(e),
        }
    }
}
