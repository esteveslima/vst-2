use crate::infrastructure::stream::client::stream_consumer_client::{
    StreamConsumerClient, StreamConsumerClientSetupParameters, StreamConsumerClientTrait,
};

use crate::features::stocks_api::adapters::entrypoints::consumers::stock_consumer::StockConsumer;

pub async fn setup_consumer_router<'a: 'static>(
    stock_consumer: &'a Box<dyn StockConsumer + 'a>,
) -> () {
    let stock_consumer_router = StreamConsumerClient::setup(
        StreamConsumerClientSetupParameters {
            broker_host: std::env::var("STOCK_KAFKA_BROKER_HOST").unwrap(),
            topic: std::env::var("STOCK_KAFKA_TOPIC").unwrap(),
            optional_group: None,
        },
        move |payload| async { stock_consumer.handle_consume_test_operation(payload).await },
    );

    //...

    tokio::join!(
        stock_consumer_router,
        //...
    );
}
