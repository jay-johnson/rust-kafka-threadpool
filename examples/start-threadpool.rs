//! # Build the release version
//!
//! ```bash
//! cargo build --release --example start-threadpool && export RUST_BACKTRACE=1 && export RUST_LOG=info,kafka_threadpool=info && ./target/release/examples/start-threadpool
//! ```
//!
//! # Build the debug version
//!
//! ```bash
//! cargo build --example start-threadpool && export RUST_BACKTRACE=1 && export RUST_LOG=info,kafka_threadpool=info && ./target/debug/examples/start-threadpool
//! ```
//!
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::collections::HashMap;

use kafka_threadpool::api::add_messages_to_locked_work_vec::add_messages_to_locked_work_vec;
use kafka_threadpool::api::build_kafka_publish_message::build_kafka_publish_message;
use kafka_threadpool::api::kafka_publish_message::KafkaPublishMessage;
use kafka_threadpool::api::kafka_publish_message_type::KafkaPublishMessageType;
use kafka_threadpool::start_threadpool::start_threadpool;

/// main
///
/// Supported env vars:
///
/// | Environment Variable Name        | Purpose / Value                                |
/// | -------------------------------- | ---------------------------------------------- |
/// | KAFKA_ENABLED                    | toggle the kafka_threadpool on with: ``true`` or ``1`` anything else disables the threadpool |
/// | KAFKA_LOG_LABEL                  | tracking label that shows up in all crate logs |
/// | KAFKA_BROKERS                    | comma-delimited list of brokers (``host1:port,host2:port,host3:port``) |
/// | KAFKA_TOPICS                     | comma-delimited list of supported topics |
/// | KAFKA_PUBLISH_RETRY_INTERVAL_SEC | number of seconds to sleep before each publish retry |
/// | KAFKA_PUBLISH_IDLE_INTERVAL_SEC  | number of seconds to sleep if there are no message to process |
/// | KAFKA_NUM_THREADS                | number of threads for the threadpool |
/// | KAFKA_TLS_CLIENT_KEY             | optional - path to the kafka mTLS key |
/// | KAFKA_TLS_CLIENT_CERT            | optional - path to the kafka mTLS certificate |
/// | KAFKA_TLS_CLIENT_CA              | optional - path to the kafka mTLS certificate authority (CA) |
/// | KAFKA_METADATA_COUNT_MSG_OFFSETS | optional - set to anything but ``true`` to bypass counting the offsets |
///
#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    let test_case = "start_threadpool";
    let kafka_publisher = start_threadpool(Some(test_case)).await;

    info!(
        "{test_case} \
        config={}",
        kafka_publisher.config
    );

    info!("creating messages");
    let mut new_msgs: Vec<KafkaPublishMessage> = vec![];
    let num_to_send = 100;
    for i in 0..num_to_send {
        let mut headers: HashMap<String, String> = HashMap::new();
        let payload = format!("test message {i}");
        headers.insert(format!("header {i}"), format!("value {i}"));
        new_msgs.push(build_kafka_publish_message(
            KafkaPublishMessageType::Data,
            "testing",
            "testing",
            Some(headers),
            &payload,
        ));
    }

    let num_to_publish = new_msgs.len();
    info!(
        "adding {num_to_publish} msgs to the \
        lockable work vec: KafkaPublishMessage.publish_msgs"
    );
    match add_messages_to_locked_work_vec(
        &kafka_publisher.publish_msgs,
        new_msgs,
    ) {
        Ok(num_msgs_in_vec) => {
            info!(
                "added {num_to_publish} msgs with \
                total in vec={num_msgs_in_vec}"
            );
        }
        Err(e) => {
            error!(
                "failed to add {} msgs to \
                locked vec with err={e}",
                num_to_publish
            );
        }
    }

    // waits until threads exit or are shutdown
    info!("waiting 3s to send shutdown");
    std::thread::sleep(std::time::Duration::from_millis(3000));
    // send shutdown message to all worker threads in the pool
    match kafka_publisher.shutdown().await {
        Ok(msg) => trace!("{msg}"),
        Err(err_msg) => {
            error!("publisher shutdown failed with err='{err_msg}'")
        }
    }
}
