use crate::{
    common::infrastructure::clients::stream::stream_consumer_client::{
        StreamConsumerClient, StreamConsumerClientImpl, StreamConsumerClientListenParametersDTO,
    },
    features::transactions_worker::adapters::entrypoints::consumers::stock_order_consumer::StockOrderConsumer,
};

pub async fn setup_consumer_router<'a: 'static>(
    stock_order_consumer: &'a Box<dyn StockOrderConsumer + 'a>,
) -> () {
    let stock_order_consumer_router = StreamConsumerClientImpl::listen(
        StreamConsumerClientListenParametersDTO {
            broker_host: std::env::var("KAFKA_BROKER_HOST").unwrap(),
            topic: std::env::var("KAFKA_TOPIC_STOCK_ORDER").unwrap(),
            optional_group: None,
        },
        move |key, payload| async {
            stock_order_consumer
                .handle_consume_stock_order(key, payload)
                .await
        },
    );

    //...

    tokio::join!(
        stock_order_consumer_router,
        //...
    );
}
