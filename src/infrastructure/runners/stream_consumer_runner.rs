use crate::features::stocks_api::{
    adapters::entrypoints::consumers::stock_consumer::StockConsumer,
    infrastructure::routers::stock_consumer_router,
};

pub struct StreamConsumerRunnerParameters<'a> {
    pub stock_consumer: &'a Box<dyn StockConsumer>,
}

pub async fn setup_stream_consumer_runner<'a: 'static>(params: StreamConsumerRunnerParameters<'a>) {
    let StreamConsumerRunnerParameters { stock_consumer } = params;

    let setup_stock_consumer_router = stock_consumer_router::setup_consumer_router(stock_consumer);

    // ...

    tokio::join!(
        setup_stock_consumer_router,
        //...
    );
}
