//! Static configuration for the threadpool where
//! the values are read from environment variables
//! at startup
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
use std::collections::HashMap;

use log::info;

/// KafkaClientConfig
///
/// Kafka client configuration holding connectivity and static
/// values (tls assets, publishable topics, etc.)
///
#[derive(Default, Clone)]
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

impl KafkaClientConfig {
    pub fn new(label: &str) -> Self {
        let is_enabled_s = std::env::var("KAFKA_ENABLED")
            .unwrap_or_else(|_| "true".to_string())
            .to_lowercase();
        let mut is_enabled = true;
        if is_enabled_s != "true" && is_enabled_s != "1" {
            is_enabled = false;
        }

        if !is_enabled {
            info!("kafka disabled KAFKA_ENABLED={is_enabled_s}");
            return KafkaClientConfig {
                label: label.to_string(),
                is_enabled,
                broker_list: Vec::new(),
                publish_topics: HashMap::new(),
                num_threads: 0,
                retry_sleep_sec: 0,
                idle_sleep_sec: 0,
                tls_key: "".to_string(),
                tls_cert: "".to_string(),
                tls_ca: "".to_string(),
            };
        }

        let use_label = std::env::var("KAFKA_LOG_LABEL")
            .unwrap_or_else(|_| label.to_string());
        let broker_list_s =
            std::env::var("KAFKA_BROKERS").unwrap_or_else(|_| "".to_string());
        let tls_key = std::env::var("KAFKA_TLS_CLIENT_KEY")
            .unwrap_or_else(|_| "".to_string());
        let tls_cert = std::env::var("KAFKA_TLS_CLIENT_CERT")
            .unwrap_or_else(|_| "".to_string());
        let tls_ca = std::env::var("KAFKA_TLS_CLIENT_CA")
            .unwrap_or_else(|_| "".to_string());
        let env_topics =
            std::env::var("KAFKA_TOPICS").unwrap_or_else(|_| "".to_string());
        let retry_sleep_interval_s =
            std::env::var("KAFKA_PUBLISH_RETRY_INTERVAL_SEC")
                .unwrap_or_else(|_| "1".to_string());
        let idle_sleep_interval_s =
            std::env::var("KAFKA_PUBLISH_IDLE_INTERVAL_SEC")
                .unwrap_or_else(|_| "0.5".to_string());
        let num_threads_s = std::env::var("KAFKA_NUM_THREADS")
            .unwrap_or_else(|_| "5".to_string())
            .to_lowercase();

        let retry_sleep_sec_f64 = match retry_sleep_interval_s.parse::<f64>() {
            Ok(val) => val * 1000.0,
            Err(_) => panic!(
                "invalid retry sleep interval for \
                KAFKA_PUBLISH_RETRY_INTERVAL_SEC={retry_sleep_interval_s} \
                please set to a positive float between [0.001, inf]"
            ),
        };
        let retry_sleep_sec = retry_sleep_sec_f64 as u64;
        if retry_sleep_sec <= 1 {
            panic!(
                "please use a positive float for the retry sleep interval \
                KAFKA_PUBLISH_RETRY_INTERVAL_SEC={retry_sleep_sec} \
                please set to a number between [0.001, inf]"
            )
        }
        let idle_sleep_sec_f64 = match idle_sleep_interval_s.parse::<f64>() {
            Ok(val) => val * 1000.0,
            Err(_) => panic!(
                "invalid idle sleep interval for \
                KAFKA_PUBLISH_IDLE_INTERVAL_SEC={idle_sleep_interval_s} \
                please set to a positive float between [0.001, inf]"
            ),
        };
        let idle_sleep_sec = idle_sleep_sec_f64 as u64;
        if idle_sleep_sec <= 1 {
            panic!(
                "please use a positive float for the idle sleep interval \
                KAFKA_PUBLISH_IDLE_INTERVAL_SEC={idle_sleep_sec} \
                please set to a number between [0.001, inf]"
            )
        }
        let num_threads = match num_threads_s.parse::<u8>() {
            Ok(val) => val,
            Err(_) => panic!(
                "invalid number of threads for KAFKA_NUM_THREADS={num_threads_s} \
                please set to a number between 1-50"
            ),
        };
        if num_threads == 0 {
            panic!(
                "please use a valid number for the number of threads \
                KAFKA_NUM_THREADS={num_threads_s} \
                please set to a number between 1-100"
            )
        }

        let mut publish_topics: HashMap<String, String> = HashMap::new();
        let mut broker_list: Vec<String> = Vec::new();
        broker_list_s.split(',').for_each(|br| {
            broker_list.push(br.to_string());
        });
        env_topics.split(',').for_each(|tp| {
            publish_topics.insert(tp.to_string(), "0".to_string());
        });

        info!(
            "build_kafka_client_config - label={label} \
            enabled={is_enabled}
            tls key={tls_key} cert={tls_cert} ca={tls_ca} \
            retry_sleep={retry_sleep_sec} \
            threads={num_threads} \
            broker_list={:?} \
            topics={:?}",
            broker_list, publish_topics
        );

        KafkaClientConfig {
            label: use_label,
            is_enabled,
            broker_list,
            publish_topics,
            num_threads,
            retry_sleep_sec,
            idle_sleep_sec,
            tls_key,
            tls_cert,
            tls_ca,
        }
    }
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
