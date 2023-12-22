use async_trait::async_trait;
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use serde::Serialize;
use std::time::Duration;

//TODO: refactor to have producer + consumer in the same client?
pub struct StreamProducerClientBuildParameters {
    pub broker_host: String,
    pub topic: String,
}

//  //  //

pub trait StreamProducerClientConstructor {
    fn new(params: StreamProducerClientBuildParameters) -> Self;
}

#[async_trait]
pub trait StreamProducerClient: Send + Sync {
    async fn produce<T: Serialize + Send>(
        &self,
        payload: T,
        key: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

//  //  //

pub struct StreamProducerClientImpl {
    producer: FutureProducer,
    params: StreamProducerClientBuildParameters,
}

//  //  //

impl StreamProducerClientConstructor for StreamProducerClientImpl {
    fn new(params: StreamProducerClientBuildParameters) -> Self {
        let mut producer_client_config = ClientConfig::new();
        producer_client_config.set("bootstrap.servers", &params.broker_host);
        producer_client_config.set("allow.auto.create.topics", "true");

        let producer = producer_client_config
            .create::<FutureProducer>()
            .expect("Failed to connect producer to kafka");

        return StreamProducerClientImpl { producer, params };
    }
}

#[async_trait]
impl StreamProducerClient for StreamProducerClientImpl {
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
