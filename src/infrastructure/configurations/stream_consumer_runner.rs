use async_trait::async_trait;

use crate::features::stocks_api::adapters::entrypoints::consumers::stock_consumer::{
    StockConsumer, StockConsumerTrait,
};

#[async_trait]
pub trait StreamConsumerRunnerTrait {
    async fn setup();
}

pub struct StreamConsumerRunner;

#[async_trait]
impl StreamConsumerRunnerTrait for StreamConsumerRunner {
    async fn setup() {
        let stock_stream_consumer = StockConsumer::setup();
        // ...

        tokio::join!(
            stock_stream_consumer,
            //...
        );
        return;
    }
}
