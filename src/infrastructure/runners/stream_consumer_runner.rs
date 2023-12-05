use crate::features::stocks_api::infrastructure::routers::stock_consumer_router;

pub async fn setup_stream_consumer_runner() {
    let setup_stock_consumer_router = stock_consumer_router::setup_stock_consumer_router();
    // ...
    
    tokio::join!(
        setup_stock_consumer_router,
        //...
    );
}