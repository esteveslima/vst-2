use std::convert::Infallible;

use async_trait::async_trait;

use crate::infrastructure::stream::client::stream_consumer_client::{
    StreamConsumerClient, StreamConsumerClientSetupParameters, StreamConsumerClientTrait,
};

#[async_trait]
pub trait StockConsumerTrait {
    async fn setup();
}

pub struct StockConsumer;

#[async_trait]
impl StockConsumerTrait for StockConsumer {
    async fn setup() {
        let stock_consumer = StreamConsumerClient::setup(
            StreamConsumerClientSetupParameters {
                broker_host: std::env::var("STOCK_KAFKA_BROKER_HOST").unwrap(),
                topic: std::env::var("STOCK_KAFKA_TOPIC").unwrap(),
                optional_group: None,
            },
            stock_consume_handler,
        );
        async fn stock_consume_handler(payload: String) -> Result<(), Infallible> {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            println!("{}", payload);
            Ok(())
        }

        //...

        tokio::join!(
            stock_consumer,
            //...
        );
        return;
    }
}
