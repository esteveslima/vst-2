use async_trait::async_trait;
use warp::Filter;

#[async_trait]
pub trait WebServerTrait {
    async fn start();
}

pub struct WebServer;

#[async_trait]
impl WebServerTrait for WebServer {
    async fn start() {
        let stocks_controller = crate::features::stocks_api::adapters::entrypoints::controllers::stock_controller::build_controller();

        let routers = warp::any().and(stocks_controller);

        warp::serve(routers).run(([0, 0, 0, 0], 3030)).await;
    }
}
