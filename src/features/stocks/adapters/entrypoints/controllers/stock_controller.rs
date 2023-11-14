use std::convert::Infallible;
use validator::Validate;
use warp::{reply::Reply, Filter};

use crate::features::stocks::adapters::entrypoints::controllers::dtos::get_stocks_summary_rest_dto;
use crate::features::stocks::adapters::entrypoints::controllers::dtos::purchase_stock_rest_dto;
use crate::features::stocks::adapters::entrypoints::controllers::dtos::sell_stock_rest_dto;
use crate::features::stocks::adapters::entrypoints::model::api_response::APIResponse;
use crate::features::stocks::application::use_cases::get_stocks_summary_use_case;
use crate::features::stocks::application::use_cases::purchase_stock_use_case;
use crate::features::stocks::application::use_cases::sell_stock_use_case;

pub fn build_controller(
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let base_route = warp::path("stocks");

    //  POST /stocks/purchase
    let purchase_stock_route = base_route
        .and(warp::path!("purchase"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(purchase_stock);
    async fn purchase_stock(
        body: purchase_stock_rest_dto::PurchaseStockRestRequestBodyDTO,
    ) -> Result<impl Reply, Infallible> {
        match body.validate() {
            Ok(_) => (),
            Err(error) => {
                return Ok(warp::reply::with_status(
                    warp::reply::json(&(APIResponse::<&str>::error(error.to_string().as_str()))),
                    warp::http::StatusCode::BAD_REQUEST,
                ))
            }
        }
        let params = purchase_stock_use_case::PurchaseStockParametersDTO {
            shares: body.shares,
            stock: body.stock,
        };

        let use_case_result = purchase_stock_use_case::use_case(params).await;

        match use_case_result {
            Ok(result) => {
                let response = purchase_stock_rest_dto::PurchaseStockRestResponseDTO {
                    price: result.price,
                    shares: result.shares,
                    stock: result.stock,
                };
                return Ok(warp::reply::with_status(
                    warp::reply::json(&(APIResponse::<&purchase_stock_rest_dto::PurchaseStockRestResponseDTO>::success(&response))),
                    warp::http::StatusCode::OK,
                ));
            }
            Err(error) => match error {
                // SomeErr => ...
                _ => Ok(warp::reply::with_status(
                    warp::reply::json(&(APIResponse::<&str>::error("Internal Server error"))),
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                )),
            },
        }
    }

    //  POST /stocks/sell
    let sell_stock_route = base_route
        .and(warp::path!("sell"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(sell_stock);
    async fn sell_stock(
        body: sell_stock_rest_dto::SellStockRestRequestBodyDTO,
    ) -> Result<impl Reply, Infallible> {
        match body.validate() {
            Ok(_) => (),
            Err(error) => {
                return Ok(warp::reply::with_status(
                    warp::reply::json(&(APIResponse::<&str>::error(error.to_string().as_str()))),
                    warp::http::StatusCode::BAD_REQUEST,
                ))
            }
        }
        let params = sell_stock_use_case::SellStockParametersDTO {
            shares: body.shares,
            stock: body.stock,
        };

        let use_case_result = sell_stock_use_case::use_case(params).await;

        match use_case_result {
            Ok(result) => {
                let response = sell_stock_rest_dto::SellStockRestResponseDTO {
                    price: result.price,
                    shares: result.shares,
                    stock: result.stock,
                };
                return Ok(warp::reply::with_status(
                    warp::reply::json(
                        &(APIResponse::<&sell_stock_rest_dto::SellStockRestResponseDTO>::success(
                            &response,
                        )),
                    ),
                    warp::http::StatusCode::OK,
                ));
            }
            Err(error) => match error {
                // SomeErr => ...
                _ => Ok(warp::reply::with_status(
                    warp::reply::json(&(APIResponse::<&str>::error("Internal Server error"))),
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                )),
            },
        }
    }

    // GET /stocks/summary
    let get_stocks_summary_route = base_route
        .and(warp::path!("summary"))
        .and(warp::get())
        .and_then(get_stocks_summary);
    async fn get_stocks_summary() -> Result<impl Reply, Infallible> {
        let params = get_stocks_summary_use_case::GetStocksSummaryParametersDTO {};

        let use_case_result = get_stocks_summary_use_case::use_case(params).await;

        match use_case_result {
            Ok(result) => {
                let response = get_stocks_summary_rest_dto::GetStocksSummaryRestResponseDTO {
                    stock_name: result.stock_name,
                    total_shares: result.total_shares,
                    total_value: result.total_value,
                    date_purchase: result.date_purchase,
                    variation_purchase: result.variation_purchase,
                    day_max: result.day_max,
                    day_avg: result.day_avg,
                    day_min: result.day_min,
                };
                return Ok(warp::reply::with_status(
                    warp::reply::json(
                        &(APIResponse::<
                            &get_stocks_summary_rest_dto::GetStocksSummaryRestResponseDTO,
                        >::success(&response)),
                    ),
                    warp::http::StatusCode::OK,
                ));
            }
            Err(error) => match error {
                // SomeErr => ...
                _ => Ok(warp::reply::with_status(
                    warp::reply::json(&(APIResponse::<&str>::error("Internal Server error"))),
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                )),
            },
        }
    }

    //

    let controller_router = warp::any()
        .and(purchase_stock_route)
        .or(sell_stock_route)
        .or(get_stocks_summary_route);
    return controller_router;
}
