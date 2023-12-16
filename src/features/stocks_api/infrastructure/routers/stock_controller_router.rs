use warp::Filter;

use crate::features::stocks_api::adapters::entrypoints::controllers::stock_controller::StockController;

pub fn setup_controller_router<'a: 'static>(
    controller: &'a Box<dyn StockController + 'a>,
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone + 'a {
    // .../stocks/...
    let base_route = warp::path("stocks");

    //

    //  POST .../purchase
    let purchase_stock_route = base_route
        .and(warp::path!("purchase"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |body| async { controller.purchase_stock(body).await });

    //  POST .../sell
    let sell_stock_route = base_route
        .and(warp::path!("sell"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |body| async { controller.sell_stock(body).await });

    // GET .../summary
    let get_stocks_summary_route = base_route
        .and(warp::path!("summary"))
        .and(warp::get())
        .and_then(move || async { controller.get_stocks_summary().await });

    //...

    //

    let controller_router = warp::any()
        .and(purchase_stock_route)
        .or(sell_stock_route)
        .or(get_stocks_summary_route);

    return controller_router;
}
