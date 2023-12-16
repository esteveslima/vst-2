use async_trait::async_trait;
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use serde::Serialize;
use std::time::Duration;

//

pub struct StreamProducerClientSetupParameters {
    pub broker_host: String,
    pub topic: String,
}
//TODO: rename to ...Client + ...ClientImpl
#[async_trait]
pub trait StreamProducerClientTrait {
    fn setup(params: StreamProducerClientSetupParameters) -> Self;
    async fn produce<T: Serialize + Send>(
        &self,
        payload: T,
        key: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct StreamProducerClient {
    producer: FutureProducer,
    params: StreamProducerClientSetupParameters,
}

//

#[async_trait]
impl StreamProducerClientTrait for StreamProducerClient {
    fn setup(params: StreamProducerClientSetupParameters) -> Self {
        let mut producer_client_config = ClientConfig::new();
        producer_client_config.set("bootstrap.servers", &params.broker_host);
        producer_client_config.set("allow.auto.create.topics", "true");

        let producer = producer_client_config
            .create::<FutureProducer>()
            .expect("Failed to connect producer to kafka");

        return StreamProducerClient { producer, params };
    }

    async fn produce<T: Serialize + Send>(
        &self,
        payload: T,
        optional_key: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let serialized_payload = serde_json::to_string(&payload).expect("Failed to serialize data");
        let mut future_record =
            FutureRecord::<String, _>::to(&self.params.topic.as_str()).payload(&serialized_payload);

        let has_key = optional_key.is_some();
        if has_key {
            future_record = future_record.key(&optional_key.as_ref().unwrap());
        }

        let message_result = self
            .producer
            .send(future_record, Duration::from_secs(0))
            .await;

        match message_result {
            Ok(_delivery) => Ok(()),
            Err((error, _)) => Err(Box::new(error)),
        }
    }
}
