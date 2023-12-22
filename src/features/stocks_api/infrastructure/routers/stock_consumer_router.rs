use crate::infrastructure::clients::stream::stream_consumer_client::{
    StreamConsumerClient, StreamConsumerClientImpl, StreamConsumerClientListenParameters,
};

use crate::features::stocks_api::adapters::entrypoints::consumers::stock_consumer::StockConsumer;

pub async fn setup_consumer_router<'a: 'static>(
    stock_consumer: &'a Box<dyn StockConsumer + 'a>,
) -> () {
    let stock_consumer_router = StreamConsumerClientImpl::listen(
        StreamConsumerClientListenParameters {
            broker_host: std::env::var("KAFKA_BROKER_HOST").unwrap(),
            topic: std::env::var("KAFKA_TOPIC_STOCK_ORDER").unwrap(),
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
