use warp::Filter;
use crate::features::stocks_api::infrastructure::routers::stock_controller_router;

pub async fn setup_web_server_runner() {
    let stock_controller_router = stock_controller_router::setup_stock_controller_router();

    let routers = warp::any().and(stock_controller_router);

    warp::serve(routers).run(([0, 0, 0, 0], 3030)).await;
}