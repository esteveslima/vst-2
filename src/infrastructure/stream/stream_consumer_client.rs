use async_trait::async_trait;
use rdkafka::{
    self,
    consumer::{Consumer, StreamConsumer},
    Message,
};

//

pub struct StreamConsumerClientSetupParameters {
    pub broker_host: String,
    pub topic: String,
    pub optional_group: Option<String>,
}

#[async_trait]
pub trait StreamConsumerClientTrait {
    //TODO: refactor from owned parameters to borrowed parameters
    fn setup(params: StreamConsumerClientSetupParameters) -> Self;
    async fn run(&self);
}

pub struct StreamConsumerClient {
    consumer: StreamConsumer,
    params: StreamConsumerClientSetupParameters,
}

//

#[async_trait]
impl StreamConsumerClientTrait for StreamConsumerClient {
    fn setup(params: StreamConsumerClientSetupParameters) -> Self {
        let default_group = "default_consumer_group".to_string();
        let group = &params.optional_group.clone().unwrap_or(default_group);

        let mut consumer_client_config = rdkafka::ClientConfig::new();
        consumer_client_config.set("bootstrap.servers", &params.broker_host);
        consumer_client_config.set("group.id", group);
        consumer_client_config.set("enable.partition.eof", "false");
        consumer_client_config.set("enable.auto.commit", "false");
        consumer_client_config.set("session.timeout.ms", "6000");

        let consumer = consumer_client_config
            .create::<StreamConsumer>()
            .expect(&format!(
                "[Kafka Client] Failed to connect to broker '{}'",
                &params.broker_host
            ));

        consumer.subscribe(&[&params.topic]).expect(&format!(
            "[Kafka Client] Fail to subscribe to topic '{}'",
            &params.topic
        ));

        return StreamConsumerClient { consumer, params };
    }

    async fn run(&self) {   // TODO: inject handler
        loop {
            let consumer_result = &self.consumer.recv().await;

            let is_message_consumed = &consumer_result.is_ok();
            if !is_message_consumed {
                println!(
                    "[Kafka Client] Error consuming from topic '{}' -> {}",
                    &self.params.topic,
                    &consumer_result.as_ref().unwrap_err()
                )
            }

            let message = &consumer_result.as_ref().unwrap();
            let payload_consumed_result = &message.payload_view::<str>();

            let has_payload = &payload_consumed_result.is_some();
            if !has_payload {
                println!(
                    "[Kafka Client] No message payload from topic '{}'",
                    &self.params.topic
                )
            }

            let payload_result = &payload_consumed_result.unwrap();

            let is_payload_valid = &payload_result.is_ok();
            if !is_payload_valid {
                println!(
                    "[Kafka Client] Invalid message payload from topic '{}'",
                    &self.params.topic
                )
            }

            let payload = &payload_result.unwrap();
            println!("HERE: {}", &payload);
        }
    }
}
