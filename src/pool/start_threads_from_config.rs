//! Start the configured number of threads using
//! ``tokio::spawn(async move {}))``  
//!
use std::sync::Arc;
use std::sync::Mutex;

use log::info;

use crate::config::kafka_client_config::KafkaClientConfig;
use crate::kafka_publisher::KafkaPublisher;
use crate::thread_process_messages_handler::thread_process_messages_handler;

/// start_threads_from_config
///
/// # Arguments
///
/// * `config` - initialized [`KafkaClientConfig`] for the threadpool
///
/// # Examples
///
/// ```rust
/// use kafka_threadpool::api::build_kafka_client_config::build_kafka_client_config;
/// use kafka_threadpool::pool::start_threads_from_config::start_threads_from_config;
/// let config = build_kafka_client_config("testing");
/// let kafka_publisher = match start_threads_from_config(&config) {
///     Ok(kp) => kp,
///     Err(e) => panic!("failed to start threads with err={e}")
/// };
/// println!("shutting down kafka_threadpool");
/// kafka_publisher.shutdown();
/// ```
pub async fn start_threads_from_config(
    config: KafkaClientConfig,
) -> Result<KafkaPublisher, String> {
    info!("{} - starting threads={}", config.label, config.num_threads);
    let new_publisher = KafkaPublisher {
        config: config.clone(),
        // create the shared lockable vector of messages
        publish_msgs: Arc::new(Mutex::new(Vec::new())),
    };

    // start threads
    for cur_thread_num in 0..new_publisher.config.num_threads {
        info!("{} - creating thread={cur_thread_num}", config.label);
        let cloned_config = new_publisher.config.clone();
        let cloned_publishable_work_vec = new_publisher.publish_msgs.clone();
        tokio::spawn(async move {
            thread_process_messages_handler(
                cur_thread_num,
                cloned_config,
                cloned_publishable_work_vec,
            )
            .await;
        });
    }
    Ok(new_publisher)
}
