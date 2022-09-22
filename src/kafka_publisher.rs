//! Clients using ``kafka_threadpool`` get a
//! [`KafkaPublisher`](crate::kafka_publisher) object when calling
//! [`start_threadpool()`](crate::start_threadpool). The
//! [`KafkaPublisher`](crate::kafka_publisher) is how
//! callers interface with the ``kafka_threadpool``'s
//! ``lockable work Vec`` called ``publish_msgs``
//! and can gracefully shutdown the threadpool.
//!
//! Example for shutting down the threadpool:
//!
//! ```rust
//! my_kafka_publisher.shutdown().await.unwrap();
//! ```
//!
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use log::info;

use crate::api::add_messages_to_locked_work_vec::add_messages_to_locked_work_vec;
use crate::api::build_kafka_publish_message::build_kafka_publish_message;
use crate::api::drain_messages_from_locked_work_vec::drain_messages_from_locked_work_vec;
use crate::api::get_kafka_consumer::get_kafka_consumer;
use crate::api::kafka_publish_message::KafkaPublishMessage;
use crate::api::kafka_publish_message_type::KafkaPublishMessageType;
use crate::config::kafka_client_config::KafkaClientConfig;
use crate::metadata::get_kafka_metadata::get_kafka_metadata;

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
#[derive(Default, Clone)]
pub struct KafkaPublisher {
    pub config: KafkaClientConfig,
    pub publish_msgs: Arc<Mutex<Vec<KafkaPublishMessage>>>,
}

impl KafkaPublisher {
    /// new
    ///
    /// create a new singleton
    /// [`KafkaPublisher`](crate::kafka_publisher::KafkaPublisher)
    /// for interfacing with the backend kafka publish threadpool
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crate::kafka_publisher::KafkaPublisher;
    /// let kp = KafkaPublisher::new();
    /// ```
    ///
    pub fn new() -> Self {
        KafkaPublisher {
            config: KafkaClientConfig::new(
                &std::env::var("KAFKA_LOG_LABEL")
                    .unwrap_or_else(|_| "ktp".to_string()),
            ),
            publish_msgs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// add_data_msg
    ///
    /// Build a publishable data message from function
    /// arguments and add it to the
    /// lockable publish vector. Client libraries that
    /// want to just send a single
    /// message without worrying about the
    /// ``KafkaPublishMessageType`` should use this function.
    ///
    /// # Arguments
    ///
    /// * `topic` - kafka topic to publish the message into
    /// * `key` - kafka partition key
    /// * `headers` - optional - headers for the kafka message
    /// * `payload` - data within the kafka messag
    ///
    /// Uses the utility API method:
    /// [`add_messages_to_locked_work_vec`](crate::api::add_messages_to_locked_work_vec)
    ///
    /// # Returns
    ///
    /// ``Result<usize, String>``
    /// where
    /// - ``usize`` = updated number of messages in ``self.publish_msgs``
    /// after adding the new ``msg``
    /// - ``String`` = error reason
    ///
    pub async fn add_data_msg(
        &self,
        topic: &str,
        key: &str,
        headers: Option<HashMap<String, String>>,
        payload: &str,
    ) -> Result<usize, String> {
        if self.config.is_enabled {
            let msg = build_kafka_publish_message(
                KafkaPublishMessageType::Data,
                topic,
                key,
                headers,
                payload,
            );
            let pub_vec: Vec<KafkaPublishMessage> = vec![msg];
            add_messages_to_locked_work_vec(&self.publish_msgs, pub_vec)
        } else {
            Ok(0)
        }
    }

    /// add_msg
    ///
    /// Add a single message to the lockable publish vector
    ///
    /// # Arguments
    ///
    /// * `msg` - an initialized
    /// [`KafkaPublishMessage`](crate::api::kafka_publish_message::KafkaPublishMessage)
    /// to add to the lockable work vector: ``self.publish_msgs``
    ///
    /// Uses the utility API method:
    /// [`add_messages_to_locked_work_vec`](crate::api::add_messages_to_locked_work_vec)
    ///
    /// # Returns
    ///
    /// ``Result<usize, String>``
    /// where
    /// - ``usize`` = updated number of messages in ``self.publish_msgs``
    /// after adding the new ``msg``
    /// - ``String`` = error reason
    ///
    pub async fn add_msg(
        &self,
        msg: KafkaPublishMessage,
    ) -> Result<usize, String> {
        if self.config.is_enabled {
            let pub_vec: Vec<KafkaPublishMessage> = vec![msg];
            add_messages_to_locked_work_vec(&self.publish_msgs, pub_vec)
        } else {
            Ok(0)
        }
    }

    /// add_msgs
    ///
    /// Add a vector of messages to the lockable publish vector
    ///
    /// # Arguments
    ///
    /// * `msgs` - vector of
    /// [`KafkaPublishMessage`](crate::api::kafka_publish_message::KafkaPublishMessage)
    /// to add to the lockable work vector: ``self.publish_msgs``
    ///
    /// Uses the utility API method:
    /// [`add_messages_to_locked_work_vec`](crate::api::add_messages_to_locked_work_vec)
    ///
    /// # Returns
    ///
    /// ``Result<usize, String>``
    /// where
    /// - ``usize`` = updated number of messages in ``self.publish_msgs``
    /// after adding the new ``msgs``
    /// - ``String`` = error reason
    ///
    pub async fn add_msgs(
        &self,
        msgs: Vec<KafkaPublishMessage>,
    ) -> Result<usize, String> {
        if self.config.is_enabled {
            add_messages_to_locked_work_vec(&self.publish_msgs, msgs)
        } else {
            Ok(0)
        }
    }

    /// drain_msgs
    ///
    /// Helper function for testing - allows draining
    /// all data in the lockable work vec: ``self.publish_msgs``
    ///
    /// # Returns
    ///
    /// ``Vec<KafkaPublishMessage>`` containing all drained messages
    ///
    pub async fn drain_msgs(&self) -> Vec<KafkaPublishMessage> {
        if self.config.is_enabled {
            drain_messages_from_locked_work_vec(&self.publish_msgs)
        } else {
            vec![]
        }
    }

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
    /// my_threadpool.shutdown().await.unwrap();
    /// ```
    ///
    pub async fn shutdown(&self) -> Result<String, String> {
        if self.config.is_enabled {
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
        } else {
            Ok("kafka not enabled".to_string())
        }
    }

    /// get_metadata
    ///
    /// Get kafka cluster information by all topics or for
    /// just one topic
    ///
    /// # Arguments
    ///
    /// * `fetch_offsets` - when ``true`` this function will count the total number
    /// of messages in each topic
    /// * `topic` - If set, only get the details for that specific topic if set to ``None``
    /// get details for all topics
    ///
    pub async fn get_metadata(&self, fetch_offsets: bool, topic: Option<&str>) {
        if self.config.is_enabled {
            info!("creating consumer");
            let consumer = get_kafka_consumer(&self.config);
            get_kafka_metadata(&self.config, consumer, fetch_offsets, topic)
        } else {
            info!("kafka not enabled KAFKA_ENABLED={}", self.config.is_enabled);
        }
    }
}
