use async_trait::async_trait;
use std::convert::Infallible;
use validator::Validate;
use warp::reply::Reply;

use super::dtos::get_stocks_summary_rest_dto::{
    GetStocksSummaryRestResponseDTO, GetStocksSummaryRestResponseItemDTO,
};
use super::dtos::purchase_stock_rest_dto::{
    PurchaseStockRestRequestBodyDTO, PurchaseStockRestResponseDTO,
};
use super::dtos::sell_stock_rest_dto::SellStockRestRequestBodyDTO;
use super::dtos::sell_stock_rest_dto::SellStockRestResponseDTO;
use crate::features::stocks_api::adapters::entrypoints::models::api_response::APIResponse;
use crate::features::stocks_api::application::use_cases::get_stocks_summary_use_case::{
    GetStocksSummaryParametersDTO, GetStocksSummaryUseCase,
};
use crate::features::stocks_api::application::use_cases::purchase_stock_use_case::{
    PurchaseStockParametersDTO, PurchaseStockParametersPayloadDTO, PurchaseStockUseCase,
};
use crate::features::stocks_api::application::use_cases::sell_stock_use_case::{
    SellStockParametersDTO, SellStockParametersPayloadDTO, SellStockUseCase,
};

pub trait StockOrderControllerConstructor<'a> {
    fn new(
        get_stocks_summary_use_case: &'a Box<dyn GetStocksSummaryUseCase + 'a>,
        purchase_stock_use_case: &'a Box<dyn PurchaseStockUseCase + 'a>,
        sell_stock_use_case: &'a Box<dyn SellStockUseCase + 'a>,
    ) -> Self;
}

#[async_trait]
pub trait StockOrderController: Sync {
    async fn purchase_stock(
        &self,
        body: PurchaseStockRestRequestBodyDTO,
    ) -> Result<Box<dyn Reply>, Infallible>;
    async fn sell_stock(
        &self,
        body: SellStockRestRequestBodyDTO,
    ) -> Result<Box<dyn Reply>, Infallible>;
    async fn get_stocks_summary(&self) -> Result<Box<dyn Reply>, Infallible>;
}

//  //  //

pub struct StockOrderControllerImpl<'a> {
    purchase_stock_use_case: &'a Box<dyn PurchaseStockUseCase + 'a>,
    sell_stock_use_case: &'a Box<dyn SellStockUseCase + 'a>,
    get_stocks_summary_use_case: &'a Box<dyn GetStocksSummaryUseCase + 'a>,
}

//  //  //

impl<'a> StockOrderControllerConstructor<'a> for StockOrderControllerImpl<'a> {
    fn new(
        get_stocks_summary_use_case: &'a Box<dyn GetStocksSummaryUseCase + 'a>,
        purchase_stock_use_case: &'a Box<dyn PurchaseStockUseCase + 'a>,
        sell_stock_use_case: &'a Box<dyn SellStockUseCase + 'a>,
    ) -> Self {
        StockOrderControllerImpl {
            purchase_stock_use_case,
            sell_stock_use_case,
            get_stocks_summary_use_case,
        }
    }
}

#[async_trait]
impl<'a> StockOrderController for StockOrderControllerImpl<'a> {
    async fn purchase_stock(
        &self,
        body: PurchaseStockRestRequestBodyDTO,
    ) -> Result<Box<dyn Reply>, Infallible> {
        match body.validate() {
            Ok(_) => (),
            Err(error) => {
                return Ok(Box::new(warp::reply::with_status(
                    warp::reply::json(&(APIResponse::<&str>::error(error.to_string().as_str()))),
                    warp::http::StatusCode::BAD_REQUEST,
                )))
            }
        }
        let mock_user_id = 0.to_string();
        let params = PurchaseStockParametersDTO {
            user_id: mock_user_id,
            payload: PurchaseStockParametersPayloadDTO {
                shares: body.shares,
                stock: body.stock,
            },
        };

        let use_case_result = self.purchase_stock_use_case.execute(params).await;

        match use_case_result {
            Ok(result) => {
                let response = PurchaseStockRestResponseDTO {
                    id: result.id,
                    shares: result.shares,
                    stock: result.stock,
                };
                return Ok(Box::new(warp::reply::with_status(
                    warp::reply::json(
                        &(APIResponse::<&PurchaseStockRestResponseDTO>::success(&response)),
                    ),
                    warp::http::StatusCode::ACCEPTED,
                )));
            }
            Err(error) => match error {
                // SomeErr => ...
                _ => {
                    eprintln!("{}", error);
                    Ok(Box::new(warp::reply::with_status(
                        warp::reply::json(&(APIResponse::<&str>::error("Internal Server error"))),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    )))
                }
            },
        }
    }

    async fn sell_stock(
        &self,
        body: SellStockRestRequestBodyDTO,
    ) -> Result<Box<dyn Reply>, Infallible> {
        match body.validate() {
            Ok(_) => (),
            Err(error) => {
                return Ok(Box::new(warp::reply::with_status(
                    warp::reply::json(&(APIResponse::<&str>::error(error.to_string().as_str()))),
                    warp::http::StatusCode::BAD_REQUEST,
                )))
            }
        }
        let mock_user_id = 0.to_string();
        let params = SellStockParametersDTO {
            user_id: mock_user_id,
            payload: SellStockParametersPayloadDTO {
                shares: body.shares,
                stock: body.stock,
            },
        };

        let use_case_result = self.sell_stock_use_case.execute(params).await;

        match use_case_result {
            Ok(result) => {
                let response = SellStockRestResponseDTO {
                    id: result.id,
                    shares: result.shares,
                    stock: result.stock,
                };
                return Ok(Box::new(warp::reply::with_status(
                    warp::reply::json(
                        &(APIResponse::<&SellStockRestResponseDTO>::success(&response)),
                    ),
                    warp::http::StatusCode::ACCEPTED,
                )));
            }
            Err(error) => match error {
                // SomeErr => ...
                _ => {
                    eprintln!("{}", error);
                    Ok(Box::new(warp::reply::with_status(
                        warp::reply::json(&(APIResponse::<&str>::error("Internal Server error"))),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    )))
                }
            },
        }
    }

    async fn get_stocks_summary(&self) -> Result<Box<dyn Reply>, Infallible> {
        let mock_user_id = 0;
        let params = GetStocksSummaryParametersDTO {
            user_id: mock_user_id.to_string(),
        };

        let use_case_result = self.get_stocks_summary_use_case.execute(params).await;

        match use_case_result {
            Ok(result) => {
                let response = GetStocksSummaryRestResponseDTO {
                    stocks: result
                        .stocks
                        .iter()
                        .map(|owned_stock| GetStocksSummaryRestResponseItemDTO {
                            stock: owned_stock.stock.clone(),
                            total_shares: owned_stock.total_shares,
                            total_purchase_price: owned_stock.total_purchase_price,
                            date_first_purchase: owned_stock.date_first_purchase.to_string(),
                            variation_purchase: owned_stock.variation_purchase,
                            day_max: owned_stock.day_max,
                            day_avg: owned_stock.day_avg,
                            day_min: owned_stock.day_min,
                        })
                        .collect(),
                };
                return Ok(Box::new(warp::reply::with_status(
                    warp::reply::json(
                        &(APIResponse::<&GetStocksSummaryRestResponseDTO>::success(&response)),
                    ),
                    warp::http::StatusCode::OK,
                )));
            }
            Err(error) => match error {
                // SomeErr => ...
                _ => {
                    eprintln!("{}", error);
                    Ok(Box::new(warp::reply::with_status(
                        warp::reply::json(&(APIResponse::<&str>::error("Internal Server error"))),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    )))
                }
            },
        }
    }

    //...
}
