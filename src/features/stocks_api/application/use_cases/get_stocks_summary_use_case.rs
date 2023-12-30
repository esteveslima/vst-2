use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std;

use crate::{application::interfaces::use_case::UseCase, features::{stocks_api::application::interfaces::gateways::daos::stock_order_transaction_dao_gateway::{StockOrderTransactionDAOGateway, GetWalletParametersDTO, GetWalletHistoricalStatisticsParametersDTO}, transactions_worker::domain::entities::stock_order_transaction::StockOrderTransactionOperation}};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetStocksSummaryParametersDTO {
    pub user_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetStocksSummaryResultItemDTO {
    pub stock: String,
    pub total_shares: i64,
    pub total_purchase_price: f64,
    pub variation_purchase: f32,
    pub date_first_purchase: DateTime<Utc>,
    pub day_min: f32,
    pub day_avg: f32,
    pub day_max: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetStocksSummaryResultDTO {
    pub stocks: Vec<GetStocksSummaryResultItemDTO>,
}

// // //

pub trait GetStocksSummaryUseCaseConstructor<'a> {
    fn new(
        stock_order_transaction_dao_gateway: &'a Box<dyn StockOrderTransactionDAOGateway + 'a>,
    ) -> Self;
}

#[async_trait]
pub trait GetStocksSummaryUseCase:
    UseCase<GetStocksSummaryParametersDTO, GetStocksSummaryResultDTO>
{
}

//  //  //

pub struct GetStocksSummaryUseCaseImpl<'a> {
    stock_order_transaction_dao_gateway: &'a Box<dyn StockOrderTransactionDAOGateway + 'a>,
}

//  //  //

impl<'a> GetStocksSummaryUseCase for GetStocksSummaryUseCaseImpl<'a> {}

impl<'a> GetStocksSummaryUseCaseConstructor<'a> for GetStocksSummaryUseCaseImpl<'a> {
    fn new(
        stock_order_transaction_dao_gateway: &'a Box<dyn StockOrderTransactionDAOGateway + 'a>,
    ) -> Self {
        GetStocksSummaryUseCaseImpl {
            stock_order_transaction_dao_gateway,
        }
    }
}

#[async_trait]
impl<'a> UseCase<GetStocksSummaryParametersDTO, GetStocksSummaryResultDTO>
    for GetStocksSummaryUseCaseImpl<'a>
{
    async fn execute(
        &self,
        params: GetStocksSummaryParametersDTO,
    ) -> Result<GetStocksSummaryResultDTO, Box<dyn std::error::Error + Send + Sync>> {
        let GetStocksSummaryParametersDTO { user_id } = params;

        let (wallet_result, wallet_historical_statistics_result) = tokio::join!(
            self.stock_order_transaction_dao_gateway
                .get_wallet(GetWalletParametersDTO {
                    user_id: user_id.clone()
                }),
            self.stock_order_transaction_dao_gateway
                .get_wallet_historical_statistics(GetWalletHistoricalStatisticsParametersDTO {
                    user_id,
                })
        );

        let is_data_fetch_successful =
            wallet_result.is_ok() && wallet_historical_statistics_result.is_ok();
        if !is_data_fetch_successful {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Error: [{:?}] [{:?}]",
                    wallet_result, wallet_historical_statistics_result
                ),
            )));
        }

        let wallet = wallet_result.unwrap();
        let wallet_historical_statistics = wallet_historical_statistics_result.unwrap();

        let wallet_clone = wallet.clone();

        let result = GetStocksSummaryResultDTO {
            stocks: wallet_clone
                .wallet
                .iter()
                .map(move |owned_stock| {
                    let matching_stock_wallet_statistics = wallet_historical_statistics
                        .data
                        .iter()
                        .find(|data| {
                            let is_purchase =
                                data.operation == StockOrderTransactionOperation::PURCHASE;
                            let is_matching_stock = data.stock == owned_stock.stock.clone();
                            return is_purchase && is_matching_stock;
                        })
                        .unwrap();

                    GetStocksSummaryResultItemDTO {
                        stock: owned_stock.stock.clone(),
                        total_shares: owned_stock.total_shares,
                        total_purchase_price: owned_stock.total_purchased_value,
                        date_first_purchase: matching_stock_wallet_statistics.first_operation_date,
                        variation_purchase: 0.0, // TODO: fix API request
                        day_max: 0.0,
                        day_avg: 0.0,
                        day_min: 0.0,
                    }
                })
                .collect(),
        };

        return Ok(result);
    }
}
