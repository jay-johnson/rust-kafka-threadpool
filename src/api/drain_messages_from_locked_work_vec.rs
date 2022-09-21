//! Helper for locking the work Vec and draining all messages while locked
//!
use std::sync::Arc;
use std::sync::Mutex;

use log::error;

use crate::api::kafka_publish_message::KafkaPublishMessage;

/// drain_messages_from_locked_work_vec
///
/// API for draining all messages out of the shared publish work vec while
/// the ``Mutex`` is locked.
///
/// # Returns
///
/// All messages drained process from the ``local_access_to_work_vec`` returned in a:
/// ``Vec<KafkaPublishMessage>``
///
/// # Arguments
///
/// * `lockable_work_vec` - shared work vec of
/// [`KafkaPublishMessage`] messages to process within a lockable
/// [`Arc<Mutex<lockable_work_vec>>`] thread-safe object
///
pub fn drain_messages_from_locked_work_vec(
    lockable_work_vec: &Arc<Mutex<Vec<KafkaPublishMessage>>>,
) -> Vec<KafkaPublishMessage> {
    // CRITICAL SECTION - start - lock the mutex
    match lockable_work_vec.lock() {
        Ok(mut local_access_to_work_vec) => {
            // drain messages while locked
            let num_msgs = local_access_to_work_vec.len();
            if num_msgs > 10 {
                local_access_to_work_vec.drain(0..10).collect()
            } else {
                local_access_to_work_vec.drain(0..num_msgs).collect()
            }
        }
        Err(e) => {
            error!("failed to get lock on work vec with err={e}");
            vec![]
        }
    }
    // CRITICAL SECTION - start - unlock the mutex
}
