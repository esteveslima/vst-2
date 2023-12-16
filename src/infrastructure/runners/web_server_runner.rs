use crate::features::stocks_api::{
    adapters::entrypoints::controllers::stock_controller::StockController,
    infrastructure::routers::stock_controller_router,
};
use warp::Filter;

pub async fn setup_web_server_runner<'a: 'static>(stock_controller: &'a Box<dyn StockController>) {
    let stock_controller_router =
        stock_controller_router::setup_controller_router(stock_controller);
    // ...

    let routers = warp::any().and(stock_controller_router);

    warp::serve(routers).run(([0, 0, 0, 0], 3030)).await;
}
