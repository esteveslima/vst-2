use async_trait::async_trait;
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use serde::Serialize;
use std::time::Duration;

//TODO: refactor to have producer + consumer in the same client?
pub struct StreamProducerClientBuildParametersDTO {
    pub broker_host: String,
    pub topic: String,
}

//

pub struct StreamProducerClientProduceParametersDTO<T> {
    pub payload: T,
    pub optional_key: Option<String>,
}
pub struct StreamProducerClientProduceResultDTO {
    pub id: String,
}

//  //  //

pub trait StreamProducerClientConstructor {
    fn new(params: StreamProducerClientBuildParametersDTO) -> Self;
}

#[async_trait]
pub trait StreamProducerClient: Send + Sync {
    async fn produce<T: Serialize + Send>(
        &self,
        params: StreamProducerClientProduceParametersDTO<T>,
    ) -> Result<StreamProducerClientProduceResultDTO, Box<dyn std::error::Error + Send + Sync>>;
}

//  //  //

pub struct StreamProducerClientImpl {
    producer: FutureProducer,
    params: StreamProducerClientBuildParametersDTO,
}

//  //  //

impl StreamProducerClientConstructor for StreamProducerClientImpl {
    fn new(params: StreamProducerClientBuildParametersDTO) -> Self {
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
        params: StreamProducerClientProduceParametersDTO<T>,
    ) -> Result<StreamProducerClientProduceResultDTO, Box<dyn std::error::Error + Send + Sync>>
    {
        let StreamProducerClientProduceParametersDTO {
            payload,
            optional_key,
        } = params;

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
            Ok(delivery) => Ok(StreamProducerClientProduceResultDTO {
                id: format!("{}_{}", delivery.0, delivery.1), // compose the id with the partition and offset of the message
            }),
            Err((error, _)) => Err(Box::new(error)),
        }
    }
}
