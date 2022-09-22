//! Create a [`rdkafka::consumer::BaseConsumer`](rdkafka::consumer::BaseConsumer) from
//! a [`KafkaClientConfig`](crate::config::kafka_client_config::KafkaClientConfig)
//!
//! If the tls CA, key and cert are not set, then the consumer will use ``PLAINTEXT`` instead
//! of SSL. ``PLAINTEXT`` means no encryption in transit
//! (aka - this is not safe to use with kafka connections that go over the WAN / internet).
//!
use log::info;

use rdkafka::config::ClientConfig;
use rdkafka::consumer::BaseConsumer;

use crate::config::kafka_client_config::KafkaClientConfig;

/// get_kafka_consumer
///
/// # Returns
///
/// An intialized: [`rdkafka::consumer::BaseConsumer`](rdkafka::consumer::BaseConsumer)
///
/// # Arguments
///
/// * `config` - existing [`KafkaClientConfig`] for
/// configurable static connectivity values
///
pub fn get_kafka_consumer(config: &KafkaClientConfig) -> BaseConsumer {
    if config.tls_key.is_empty()
        && config.tls_cert.is_empty()
        && config.tls_ca.is_empty()
    {
        info!("connecting with PLAINTEXT");
        ClientConfig::new()
            .set("bootstrap.servers", config.broker_list.join(","))
            .set("security.protocol", "PLAINTEXT")
            .create()
            .expect("Consumer creation error")
    } else {
        ClientConfig::new()
            .set("bootstrap.servers", config.broker_list.join(","))
            .set("security.protocol", "SSL")
            .set("ssl.ca.location", config.tls_ca.clone())
            .set("ssl.key.location", config.tls_key.clone())
            .set("ssl.certificate.location", config.tls_cert.clone())
            .set("enable.ssl.certificate.verification", "true")
            .create()
            .expect("Consumer creation error")
    }
}
