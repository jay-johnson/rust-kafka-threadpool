//! Handler that each tokio-spawned thread uses to process all messages
//!
use std::sync::Arc;
use std::sync::Mutex;

use log::error;
use log::info;
use log::trace;

use rdkafka::message::OwnedHeaders;

use crate::api::add_messages_to_locked_work_vec::add_messages_to_locked_work_vec;
use crate::api::drain_messages_from_locked_work_vec::drain_messages_from_locked_work_vec;
use crate::api::get_kafka_producer::get_kafka_producer;
use crate::api::kafka_publish_message::KafkaPublishMessage;
use crate::api::kafka_publish_message_type::KafkaPublishMessageType;
use crate::config::kafka_client_config::KafkaClientConfig;
use crate::msg::publish_message::convert_hashmap_headers_to_ownedheaders;
use crate::msg::publish_message::publish_message;

/// thread_process_messages_handler
///
/// Each tokio-spawned thread calls this method
///
/// # Arguments
///
/// * `cur_thread_num` - thread counter assigned by
/// [`start_threads_from_config`]
/// * `config` - initialized [`KafkaClientConfig`] for this thread
/// * `lockable_work_vec` - shared work vec of
/// [`KafkaPublishMessage`] messages to process within a lockable
/// [`Arc<Mutex<lockable_work_vec>>`] thread-safe object
///
pub async fn thread_process_messages_handler(
    cur_thread_num: u8,
    config: KafkaClientConfig,
    lockable_work_vec: Arc<Mutex<Vec<KafkaPublishMessage>>>,
) {
    // THREAD CONTEXT - start
    let mut work_vec: Vec<KafkaPublishMessage> = Vec::with_capacity(20);
    let log_label = format!("{}-tid-{}", config.label, cur_thread_num + 1);
    // connect to the kafka cluster before starting
    if config.broker_list.is_empty() {
        error!(
            "{log_label} - \
            no brokers to connect to KAFKA_BROKERS={:?} - stopping thread",
            config.broker_list
        );
        return;
    }
    if config.broker_list[0].is_empty() {
        error!(
            "{log_label} - \
            no brokers to connect to KAFKA_BROKERS={:?} - stopping thread",
            config.broker_list
        );
        return;
    }
    if cur_thread_num == 0 {
        info!(
            "threadpool connecting to brokers={:?} topics={:?} \
            tls ca={} key={} cert={} \
            work_vec_cap={}",
            config.broker_list,
            config.publish_topics,
            config.tls_ca,
            config.tls_key,
            config.tls_ca,
            work_vec.capacity()
        );
    }
    let producer = get_kafka_producer(&config);
    trace!("{log_label} - start");
    // In a loop, read data from the socket and write the data back.
    loop {
        let mut should_shutdown = false;
        work_vec = drain_messages_from_locked_work_vec(&lockable_work_vec);
        if work_vec.is_empty() {
            trace!("{log_label} - idle");
            std::thread::sleep(std::time::Duration::from_millis(
                config.idle_sleep_sec,
            ));
            continue;
        } else {
            trace!("{log_label} - processing {} msgs", work_vec.len());
            // publish the messages with a retry timer
            while !work_vec.is_empty() {
                let msg = work_vec.remove(0);
                if msg.msg_type == KafkaPublishMessageType::Shutdown {
                    should_shutdown = true;
                    // requeue shutdown message for other threads
                    let requeue_vec: Vec<KafkaPublishMessage> =
                        vec![msg.clone()];
                    match add_messages_to_locked_work_vec(
                        &lockable_work_vec,
                        requeue_vec,
                    ) {
                        Ok(num_msgs_in_vec) => {
                            trace!(
                                "{log_label} - requeue shutdown message \
                                success with total in vec={num_msgs_in_vec}"
                            );
                        }
                        Err(e) => {
                            error!(
                                "{log_label} - failed to requeue shutdown \
                                message into vec with err={e}"
                            );
                        }
                    }
                    // success ends the retry loop
                    break;
                } else if msg.msg_type == KafkaPublishMessageType::Data {
                    let payload_sub = msg.payload[..10].to_string();
                    trace!(
                        "{log_label} pub \
                        topic={} data='{}'",
                        msg.topic,
                        payload_sub
                    );
                    let topic = msg.topic.clone();
                    let mut owned_headers: OwnedHeaders = OwnedHeaders::new();
                    if msg.headers.is_some() {
                        owned_headers = convert_hashmap_headers_to_ownedheaders(
                            msg.headers.clone().unwrap(),
                            owned_headers,
                        );
                    }
                    // success ends the retry loop
                    loop {
                        let delivery_status =
                            publish_message(&producer, &msg, &owned_headers)
                                .await;
                        if delivery_status == 0 {
                            trace!("published message topic={topic}");
                            break;
                        } else {
                            error!(
                                "failed to publish \
                                delivery status={} retrying msg={:?}",
                                delivery_status, msg
                            );
                            std::thread::sleep(
                                std::time::Duration::from_millis(
                                    config.retry_sleep_sec,
                                ),
                            );
                        }
                    }
                } else if msg.msg_type
                    == KafkaPublishMessageType::LogBrokerDetails
                {
                    info!(
                        "{log_label} not supported yet - get broker details \
                        type={:?} - coming soon",
                        msg.msg_type
                    );
                    break;
                } else {
                    error!(
                        "{log_label} - \
                        unsupported KafkaPublishMessageType={:?}",
                        msg.msg_type
                    );
                    break;
                }
            }
            // after processing everything in the vec - break the main thread loop if shutting down
            if should_shutdown {
                let num_left = work_vec.len();
                if num_left == 0 {
                    trace!("{log_label} - work vec empty={num_left}");
                } else {
                    error!("{log_label} - work vec NOT empty={num_left}");
                }
                break;
            }
            // if everything published, clear the temp drained vec
            work_vec.clear();
        }
    }
    info!("{log_label} - done exiting thread");
    // THREAD CONTEXT - end
}
