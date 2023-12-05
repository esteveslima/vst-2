use crate::infrastructure::stream::client::stream_consumer_client::{
    StreamConsumerClient, StreamConsumerClientSetupParameters, StreamConsumerClientTrait,
};

use crate::features::stocks_api::adapters::entrypoints::consumers::stock_consumer::StockConsumer;

pub async fn setup_stock_consumer_router() -> () {
    let stock_consumer = StreamConsumerClient::setup(
        StreamConsumerClientSetupParameters {
            broker_host: std::env::var("STOCK_KAFKA_BROKER_HOST").unwrap(),
            topic: std::env::var("STOCK_KAFKA_TOPIC").unwrap(),
            optional_group: None,
        },
        StockConsumer::handle_consume_test_operation,
    );

    //...

    tokio::join!(
        stock_consumer,
        //...
    );
}