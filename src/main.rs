use warp::Filter;

pub mod features {
    pub mod stocks_api;
}

#[tokio::main]
async fn main() {
    let stocks_controller =
        features::stocks_api::adapters::entrypoints::controllers::stock_controller::build_controller();

    let routers = warp::any().and(stocks_controller);

    warp::serve(routers).run(([0, 0, 0, 0], 3030)).await;
}
