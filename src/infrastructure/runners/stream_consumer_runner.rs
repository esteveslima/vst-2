use crate::features::transactions_worker::{
    adapters::entrypoints::consumers::stock_order_consumer::StockOrderConsumer,
    infrastructure::routers::transaction_consumer_router,
};

pub struct StreamConsumerRunnerParameters<'a> {
    pub stock_order_consumer: &'a Box<dyn StockOrderConsumer>,
}

pub async fn setup_stream_consumer_runner<'a: 'static>(params: StreamConsumerRunnerParameters<'a>) {
    let StreamConsumerRunnerParameters {
        stock_order_consumer,
    } = params;

    let transaction_consumer_router =
        transaction_consumer_router::setup_consumer_router(stock_order_consumer);

    // ...

    tokio::join!(
        transaction_consumer_router,
        //...
    );
}
