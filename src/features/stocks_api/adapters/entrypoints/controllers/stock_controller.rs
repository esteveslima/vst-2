use std::convert::Infallible;
use validator::Validate;
use warp::reply::Reply;

use crate::features::stocks_api::adapters::entrypoints::controllers::dtos::get_stocks_summary_rest_dto;
use crate::features::stocks_api::adapters::entrypoints::controllers::dtos::purchase_stock_rest_dto;
use crate::features::stocks_api::adapters::entrypoints::controllers::dtos::sell_stock_rest_dto;
use crate::features::stocks_api::adapters::entrypoints::model::api_response::APIResponse;
use crate::features::stocks_api::application::interfaces::use_case::UseCase;
use crate::features::stocks_api::application::use_cases::get_stocks_summary_use_case;
use crate::features::stocks_api::application::use_cases::purchase_stock_use_case;
use crate::features::stocks_api::application::use_cases::sell_stock_use_case;

pub struct StockController;

impl StockController {
    pub async fn purchase_stock(
        body: purchase_stock_rest_dto::PurchaseStockRestRequestBodyDTO
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
        let mock_user_id = 0;
        let params = purchase_stock_use_case::PurchaseStockParametersDTO {
            user_id: mock_user_id,
            payload: purchase_stock_use_case::PurchaseStockParametersPayloadDTO {
                shares: body.shares,
                stock: body.stock,
            },
        };

        let use_case_result = purchase_stock_use_case::PurchaseStockUseCase::execute(params).await;

        match use_case_result {
            Ok(result) => {
                let response = purchase_stock_rest_dto::PurchaseStockRestResponseDTO {
                    id: result.id,
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

    pub async fn sell_stock(
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
        let mock_user_id = 0;
        let params = sell_stock_use_case::SellStockParametersDTO {
            user_id: mock_user_id,
            payload: sell_stock_use_case::SellStockParametersPayloadDTO {
                shares: body.shares,
                stock: body.stock,
            },
        };

        let use_case_result = sell_stock_use_case::SellStockUseCase::execute(params).await;

        match use_case_result {
            Ok(result) => {
                let response = sell_stock_rest_dto::SellStockRestResponseDTO {
                    id: result.id,
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

    pub async fn get_stocks_summary() -> Result<impl Reply, Infallible> {
        let mock_user_id = 0;
        let params = get_stocks_summary_use_case::GetStocksSummaryParametersDTO {
            user_id: mock_user_id,
        };

        let use_case_result =
            get_stocks_summary_use_case::GetStocksSummaryUseCase::execute(params).await;

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

    //...
}