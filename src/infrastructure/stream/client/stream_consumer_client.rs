#![allow(unused)]
use std::{
    convert::Infallible,
    future::Future,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use rdkafka::{
    self,
    consumer::{CommitMode, Consumer, DefaultConsumerContext, StreamConsumer},
    message::BorrowedMessage,
    ClientConfig, Message,
};

//

#[derive(Clone)]
pub struct StreamConsumerClientSetupParameters {
    pub broker_host: String,
    pub topic: String,
    pub optional_group: Option<String>,
}

#[async_trait]
pub trait StreamConsumerClientTrait {
    async fn setup<F, Fut>(params: StreamConsumerClientSetupParameters, handler: F)
    where
        F: Send + Copy + Sync + 'static + Fn(String) -> Fut,
        Fut: Future<Output = Result<(), Infallible>> + Send;
}

pub struct StreamConsumerClient;

//

#[async_trait]
impl StreamConsumerClientTrait for StreamConsumerClient {
    async fn setup<F, Fut>(params: StreamConsumerClientSetupParameters, handler: F)
    where
        F: Send + Copy + Sync + 'static + Fn(String) -> Fut,
        Fut: Future<Output = Result<(), Infallible>> + Send,
    {
        let StreamConsumerClientSetupParameters {
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

        loop {
            let consumed_message: BorrowedMessage<'_> = {
                let consumer_result = consumer.recv().await;

                let is_message_consumed = consumer_result.is_ok();
                if !is_message_consumed {
                    println!(
                        "[Kafka Client] Error consuming from topic '{}' -> {}",
                        &topic,
                        &consumer_result.as_ref().unwrap_err()
                    )
                }

                let message = consumer_result.unwrap();

                message
            };

            let consumed_message_payload: String = {
                let payload_consumed_result = &consumed_message.payload_view::<str>();

                let has_payload = payload_consumed_result.is_some();
                if !has_payload {
                    println!("[Kafka Client] No message payload")
                }

                let payload_result = &payload_consumed_result.unwrap();

                let is_payload_valid = &payload_result.is_ok();
                if !is_payload_valid {
                    println!("[Kafka Client] Invalid message payload")
                }

                let payload = payload_result.unwrap().to_string();

                payload
            };

            // Handle the consumed message one at the time synchronously
            let handler_result = handler(consumed_message_payload).await;

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

    // // TODO: Handle the consumed message in parallel, asynchronously
    // // P.S.1: There might be problems moving references into tokio task
    // // P.S.2: Commiting messages asynchronously may lead into problems with kafka offset order
    // async fn setup_async<F, Fut>(params: StreamConsumerClientSetupParameters, handler: F)
    // where
    //     F: Send + Copy + Sync + 'static + Fn(String) -> Fut,
    //     Fut: Future<Output = Result<(), Infallible>> + Send,
    //     {

    //     }
}
