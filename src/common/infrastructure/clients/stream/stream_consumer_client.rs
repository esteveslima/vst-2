#![allow(unused)]
use std::{
    convert::Infallible,
    future::Future,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use rdkafka::{
    self,
    consumer::{self, CommitMode, Consumer, DefaultConsumerContext, StreamConsumer},
    message::BorrowedMessage,
    types::RDKafkaErrorCode,
    ClientConfig, Message,
};

#[derive(Clone, Debug)]
pub struct StreamConsumerClientListenParametersDTO {
    pub broker_host: String,
    pub topic: String,
    pub optional_group: Option<String>,
}

//  //  //

#[async_trait]
pub trait StreamConsumerClient {
    async fn listen<F, Fut>(params: StreamConsumerClientListenParametersDTO, handler: F)
    where
        F: Send + Copy + Sync + Fn(Option<String>, String) -> Fut,
        Fut: Future<Output = Result<(), Infallible>> + Send;
}

//  //  //

pub struct StreamConsumerClientImpl;

//  //  //

#[async_trait]
impl StreamConsumerClient for StreamConsumerClientImpl {
    async fn listen<F, Fut>(params: StreamConsumerClientListenParametersDTO, handler: F)
    where
        F: Send + Copy + Sync + Fn(Option<String>, String) -> Fut,
        Fut: Future<Output = Result<(), Infallible>> + Send,
    {
        let StreamConsumerClientListenParametersDTO {
            broker_host,
            topic,
            optional_group,
        } = params;

        let consumer: StreamConsumer = {
            let default_group = "default_consumer_group".to_string();
            let group = &optional_group.clone().unwrap_or(default_group);

            let mut consumer_client_config = ClientConfig::new();
            consumer_client_config.set("bootstrap.servers", &broker_host);
            consumer_client_config.set("group.id", group);
            consumer_client_config.set("enable.partition.eof", "false");
            consumer_client_config.set("enable.auto.commit", "false");
            consumer_client_config.set("session.timeout.ms", "6000");
            consumer_client_config.set("allow.auto.create.topics", "true");

            let consumer = consumer_client_config
                .create::<StreamConsumer>()
                .expect(&format!(
                    "[Kafka Client] Failed to connect to broker '{broker_host}'"
                ));

            consumer.subscribe(&[&topic]).expect(&format!(
                "[Kafka Client] Fail to subscribe to topic '{topic}'"
            ));

            consumer
        };

        'consumer_loop: loop {
            let consumed_message: BorrowedMessage<'_> = {
                let consumer_result = consumer.recv().await;

                let is_message_consumed = consumer_result.is_ok();
                if !is_message_consumed {
                    {
                        // According to an issue in rust-rdkafka library, the consumer client is unable to create the topic and will result in this error
                        const RDKAFKA_CONSUMER_AUTO_CREATE_TOPIC_IGNORABLE_ERROR: RDKafkaErrorCode =
                            RDKafkaErrorCode::UnknownTopicOrPartition;
                        // It was recommended to ignore it because the consumer would be able to pick up messages as soon as the topic is created
                        let message_consumed_error = consumer_result
                            .as_ref()
                            .unwrap_err()
                            .rdkafka_error_code()
                            .unwrap();
                        if message_consumed_error
                            == RDKAFKA_CONSUMER_AUTO_CREATE_TOPIC_IGNORABLE_ERROR
                        {
                            println!("Ignoring known error [{}], Consumer will pickup events after the topic is created", message_consumed_error);
                            continue;
                        }
                    }

                    println!(
                        "[Kafka Client] Error consuming from topic '{}' -> {}",
                        &topic,
                        &consumer_result.as_ref().unwrap_err()
                    )
                }

                let message = consumer_result.unwrap();

                message
            };

            let consumed_message_key: Option<String> = 'block: {
                let key_bytes = &consumed_message.key();
                if key_bytes.is_none() {
                    break 'block None;
                }

                let parsed_key_result = String::from_utf8(key_bytes.unwrap().to_vec());
                if parsed_key_result.is_err() {
                    break 'block None;
                }

                let key_optional_string = parsed_key_result.unwrap();
                Some(key_optional_string)
            };

            let consumed_message_payload: String = {
                let payload_consumed_result = &consumed_message.payload_view::<str>();

                let has_payload = payload_consumed_result.is_some();
                if !has_payload {
                    println!("[Kafka Client] No message payload");
                    continue 'consumer_loop;
                }

                let payload_result = &payload_consumed_result.unwrap();

                let is_payload_valid = &payload_result.is_ok();
                if !is_payload_valid {
                    println!("[Kafka Client] Invalid message payload");
                    continue 'consumer_loop;
                }

                let payload = payload_result.unwrap().to_string();

                payload
            };

            // Handle the consumed message one at the time
            let handler_result = handler(consumed_message_key, consumed_message_payload).await;

            let is_handler_successful = handler_result.is_ok();
            if !is_handler_successful {
                println!(
                    "Error handling topic '{}' message '{}'",
                    topic,
                    handler_result.unwrap_err()
                );
            }

            consumer
                .commit_message(&consumed_message, CommitMode::Async)
                .unwrap();
        }
    }

    // // Potential improvement: Handle the consumed message in parallel, asynchronously
    // // P.S.1: There might be problems moving references into tokio task
    // // P.S.2: Commiting messages asynchronously may lead into problems with kafka offset order
    // async fn setup_async<F, Fut>(params: StreamConsumerClientListenParametersDTO, handler: F)
    // where
    //     F: Send + Copy + Sync + 'static + Fn(String) -> Fut,
    //     Fut: Future<Output = Result<(), Infallible>> + Send,
    //     {

    //     }
}
