//! Start the threadpool and return a
//! [`KafkaPublisher`](crate::kafka_publisher::KafkaPublisher)
//!
use log::info;
use log::trace;

use crate::api::build_kafka_client_config::build_kafka_client_config;
use crate::kafka_publisher::KafkaPublisher;
use crate::pool::start_threads_from_config::start_threads_from_config;

/// start_threadpool
///
/// # Arguments
///
/// * `label` - optional tracking log label
/// (``ktp`` is the default if not set)
///
/// # Examples
///
/// ```rust
/// use kafka_threadpool::start_threadpool::start_threadpool;
/// let log_label = "ktp";
/// let kafka_publisher = start_threadpool(log_label).await;
/// ```
///
pub async fn start_threadpool(label: Option<&str>) -> KafkaPublisher {
    trace!("start_threadpool - building config");
    let ll =
        std::env::var("KAFKA_LOG_LABEL").unwrap_or_else(|_| "ktp".to_string());
    let use_label: &str = match label {
        Some(in_label) => in_label,
        None => &ll,
    };
    let config = build_kafka_client_config(use_label);
    trace!("start_threadpool - starting threads");
    match start_threads_from_config(config).await {
        Ok(kafka_publisher) => {
            info!(
                "{use_label} - started {} kafka publish threads",
                kafka_publisher.config.num_threads
            );
            kafka_publisher
        }
        Err(e) => {
            // rebuild the config after using (move-ing) it
            let err_config = build_kafka_client_config(use_label);
            panic!(
                "{use_label} failed to kafka publish threads with start_threads_from_config \
                config={} err={e} - stopping",
                err_config);
        }
    }
}
