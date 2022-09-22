//! # Build the debug version
//!
//! ```bash
//! # export KAFKA_TOPIC=TOPIC_NAME
//! cargo build --example get-metadata-for-topic  && export RUST_BACKTRACE=1 && export RUST_LOG=info,kafka_threadpool=info && ./target/debug/examples/get-metadata-for-topic
//! ```
//!
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use kafka_threadpool::start_threadpool::start_threadpool;

/// main
///
/// Get metadata for single topic stored in the environment variable:
/// ``KAFKA_TOPIC``. If there is no ``KAFKA_TOPIC`` set, the process will exit.
///
#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    let label = "get-metadata-for-topic";
    let kafka_publisher = start_threadpool(Some(label)).await;

    let topic = std::env::var("KAFKA_TOPIC").unwrap_or_else(|_| "".to_string());
    if topic.is_empty() {
        error!("please set a topic with: export KAFKA_TOPIC=TOPIC_NAME")
    } else {
        info!(
            "{label} \
            config={} \
            getting all metadata",
            kafka_publisher.config
        );

        kafka_publisher.get_metadata(true, Some(&topic)).await;
    }

    info!("shutting down");
    // send shutdown message to all worker threads in the pool
    match kafka_publisher.shutdown().await {
        Ok(msg) => trace!("{msg}"),
        Err(err_msg) => {
            error!("publisher shutdown failed with err='{err_msg}'")
        }
    }
}
