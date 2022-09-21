//! start the threadpool with a logging label
//!
use log::info;

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
/// let kafka_publisher = start_threadpool(log_label);
/// ```
///
pub async fn start_threadpool(label: Option<&str>) -> KafkaPublisher {
    println!("start_threadpool - building config");
    let use_label = label.unwrap_or("ktp");
    let config = build_kafka_client_config(use_label);
    info!("start_threadpool - starting threads");
    match start_threads_from_config(config).await {
        Ok(kafka_publisher) => {
            info!(
                "{} kafka publish threads started",
                kafka_publisher.config.num_threads
            );
            info!("start_threadpool - end");
            kafka_publisher
        }
        Err(e) => {
            let err_config = build_kafka_client_config(use_label);
            panic!(
                "failed to kafka publish threads with start_threads_from_config \
                config={} err={e} - stopping",
                err_config);
        }
    }
}
