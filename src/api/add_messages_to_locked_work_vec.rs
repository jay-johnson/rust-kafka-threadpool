//! Helper for locking the work Vec and draining all messages while locked
//!
use std::sync::Arc;
use std::sync::Mutex;

use log::error;

use crate::api::kafka_publish_message::KafkaPublishMessage;

/// add_messages_to_locked_work_vec
///
/// API for adding many messages into the ``lockable_work_vec`` Vec
/// whiel the ``Mutex`` is locked.
///
/// # Returns
///
/// ``Result<usize, String>``
///
/// where:
///
/// Ok - ``Ok(num_messages_in_lockable_work_vec_after_append)``
/// Error - ``Err(reason_for_error_as_string)``
///
/// # Arguments
///
/// * `lockable_work_vec` - shared work vec of
/// [`KafkaPublishMessage`] messages to process within a lockable
/// [`Arc<Mutex<lockable_work_vec>>`] thread-safe object
/// * `msgs` - Vec of [`KafkaPublishMessage`] messages to add
/// to the locked ``lockable_work_vec``
///
pub fn add_messages_to_locked_work_vec(
    lockable_work_vec: &Arc<Mutex<Vec<KafkaPublishMessage>>>,
    mut msgs: Vec<KafkaPublishMessage>,
) -> Result<usize, String> {
    let num_to_add = msgs.len();
    if num_to_add == 0 {
        let err_msg = "no msgs to add";
        error!("{err_msg}");
        Err(err_msg.to_string())
    } else {
        // CRITICAL SECTION - start - lock the mutex
        match lockable_work_vec.lock() {
            Ok(mut local_access_to_work_vec) => {
                // add messages while locked
                local_access_to_work_vec.append(&mut msgs);
                Ok(local_access_to_work_vec.len())
            }
            Err(e) => {
                let err_msg =
                    format!("failed to get lock on work vec with err={e}");
                error!("{err_msg}");
                Err(err_msg)
            }
        }
        // CRITICAL SECTION - start - unlock the mutex
    }
}
