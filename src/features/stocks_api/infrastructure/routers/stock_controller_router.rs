use warp::Filter;

use crate::features::stocks_api::adapters::entrypoints::controllers::stock_order_controller::StockOrderController;

pub fn setup_controller_router<'a: 'static>(
    controller: &'a Box<dyn StockOrderController + 'a>,
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone + 'a {
    // .../stocks/...
    let base_route = warp::path("stocks");

    //

    //  POST .../purchase
    let purchase_stock_route = base_route
        .and(warp::path!("purchase"))
        .and(warp::post())
        .and(warp::header::<String>("USER_ID")) // Simulating auth with multiple users with a simple header
        .and(warp::body::json())
        .and_then(move |user_id, body| async { controller.purchase_stock(user_id, body).await });

    //  POST .../sell
    let sell_stock_route = base_route
        .and(warp::path!("sell"))
        .and(warp::post())
        .and(warp::header::<String>("USER_ID")) // Simulating auth with multiple users with a simple header
        .and(warp::body::json())
        .and_then(move |user_id, body| async { controller.sell_stock(user_id, body).await });

    // GET .../summary
    let get_stocks_summary_route = base_route
        .and(warp::path!("summary"))
        .and(warp::get())
        .and(warp::header::<String>("USER_ID")) // Simulating auth with multiple users with a simple header
        .and_then(move |user_id| async { controller.get_stocks_summary(user_id).await });

    //...

    //

    let controller_router = warp::any()
        .and(purchase_stock_route)
        .or(sell_stock_route)
        .or(get_stocks_summary_route);

    return controller_router;
}
