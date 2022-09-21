//! Static configuration for the threadpool where
//! the values are read from environment variables
//! at startup
//!
use std::collections::HashMap;

/// KafkaClientConfig
///
/// Kafka client configuration holding connectivity and static
/// values (tls assets, publishable topics, etc.)
///
#[derive(Clone)]
pub struct KafkaClientConfig {
    pub label: String,
    pub is_enabled: bool,
    pub broker_list: Vec<String>,
    pub publish_topics: HashMap<String, String>,
    pub num_threads: u8,
    pub retry_sleep_sec: u64,
    pub idle_sleep_sec: u64,
    pub tls_key: String,
    pub tls_cert: String,
    pub tls_ca: String,
}

impl std::fmt::Debug for KafkaClientConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "DEBUG KafkaClientConfig label={} \
            enabled={} \
            tls key={} cert={} ca={} \
            retry_sleep={} \
            idle_sleep={} \
            threads={} \
            broker_list={:?} \
            topics={:?}",
            self.label,
            self.is_enabled,
            self.tls_key,
            self.tls_cert,
            self.tls_ca,
            self.retry_sleep_sec,
            self.idle_sleep_sec,
            self.num_threads,
            self.broker_list,
            self.publish_topics
        )
    }
}

impl std::fmt::Display for KafkaClientConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "KafkaClientConfig label={} \
            enabled={} \
            tls key={} cert={} ca={} \
            retry_sleep={} \
            idle_sleep={} \
            threads={} \
            broker_list={:?} \
            topics={:?}",
            self.label,
            self.is_enabled,
            self.tls_key,
            self.tls_cert,
            self.tls_ca,
            self.retry_sleep_sec,
            self.idle_sleep_sec,
            self.num_threads,
            self.broker_list,
            self.publish_topics
        )
    }
}
