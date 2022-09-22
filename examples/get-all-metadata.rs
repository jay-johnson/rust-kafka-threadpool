//! # Build the debug version
//!
//! ```bash
//! cargo build --example get-all-metadata && export RUST_BACKTRACE=1 && export RUST_LOG=info,kafka_threadpool=info && ./target/debug/examples/get-all-metadata
//! ```
//!
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use kafka_threadpool::start_threadpool::start_threadpool;

/// main
///
/// Get all metadata for all brokers, topics and partitions
///
#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    let label = "get-all-metadata";
    let kafka_publisher = start_threadpool(Some(label)).await;

    info!(
        "{label} \
        config={} \
        getting all metadata",
        kafka_publisher.config
    );

    kafka_publisher.get_metadata(true, None).await;

    info!("shutting down");
    // send shutdown message to all worker threads in the pool
    match kafka_publisher.shutdown().await {
        Ok(msg) => trace!("{msg}"),
        Err(err_msg) => {
            error!("publisher shutdown failed with err='{err_msg}'")
        }
    }
}
