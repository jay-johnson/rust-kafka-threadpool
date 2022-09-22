//! helper for building a static configuration ``KafkaClientConfig`` object from
//! environment variables
//!
//! # Supported Environment Variables
//!
//! | Environment Variable Name        | Purpose / Value                                |
//! | -------------------------------- | ---------------------------------------------- |
//! | KAFKA_ENABLED                    | toggle the kafka_threadpool on with: ``true`` or ``1`` anything else disables the threadpool |
//! | KAFKA_LOG_LABEL                  | tracking label that shows up in all crate logs |
//! | KAFKA_BROKERS                    | comma-delimited list of brokers (``host1:port,host2:port,host3:port``) |
//! | KAFKA_TOPICS                     | comma-delimited list of supported topics |
//! | KAFKA_PUBLISH_RETRY_INTERVAL_SEC | number of seconds to sleep before each publish retry |
//! | KAFKA_PUBLISH_IDLE_INTERVAL_SEC  | number of seconds to sleep if there are no message to process |
//! | KAFKA_NUM_THREADS                | number of threads for the threadpool |
//! | KAFKA_TLS_CLIENT_KEY             | optional - path to the kafka mTLS key |
//! | KAFKA_TLS_CLIENT_CERT            | optional - path to the kafka mTLS certificate |
//! | KAFKA_TLS_CLIENT_CA              | optional - path to the kafka mTLS certificate authority (CA) |
//! | KAFKA_METADATA_COUNT_MSG_OFFSETS | optional - set to anything but ``true`` to bypass counting the offsets |
//!

use crate::config::kafka_client_config::KafkaClientConfig;

/// build_kafka_client_config
///
/// build a `KafkaClientConfig` from environment variables and defaults
///
/// # Arguments
///
/// * `label` - tracking label for logs
///
/// # Examples
///
/// ```rust
/// use kafka_threadpool::config::kafka_client_config::build_kafka_client_config;
/// let kafka_config: KafkaClientConfig = build_kafka_client_config("ktp")
/// ```
///
pub fn build_kafka_client_config(label: &str) -> KafkaClientConfig {
    KafkaClientConfig::new(label)
}
