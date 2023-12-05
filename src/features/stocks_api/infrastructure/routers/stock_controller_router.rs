use warp::Filter;

use crate::features::stocks_api::adapters::entrypoints::controllers::stock_controller::StockController;

pub fn setup_stock_controller_router(
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // .../stocks/...
    let base_route = warp::path("stocks");

    //

    //  POST .../purchase
    let purchase_stock_route = base_route
        .and(warp::path!("purchase"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(StockController::purchase_stock);

    //  POST .../sell
    let sell_stock_route = base_route
        .and(warp::path!("sell"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(StockController::sell_stock);
     
    // GET .../summary
    let get_stocks_summary_route = base_route
        .and(warp::path!("summary"))
        .and(warp::get())
        .and_then(StockController::get_stocks_summary);

    //...

    //

    let controller_router = warp::any()
        .and(purchase_stock_route)
        .or(sell_stock_route)
        .or(get_stocks_summary_route);

    return controller_router;
      
}