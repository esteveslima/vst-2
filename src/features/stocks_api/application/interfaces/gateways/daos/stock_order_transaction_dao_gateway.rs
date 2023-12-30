use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::features::transactions_worker::domain::entities::stock_order_transaction::StockOrderTransactionOperation;

#[derive(Deserialize, Serialize, Debug)]
pub struct GetWalletParametersDTO {
    pub user_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetWalletResultItemDTO {
    pub stock: String,
    pub total_shares: i64,
    pub total_purchased_value: f64,
    pub total_sold_value: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetWalletResultDTO {
    pub wallet: Vec<GetWalletResultItemDTO>,
}

//

#[derive(Deserialize, Serialize, Debug)]
pub struct GetWalletHistoricalStatisticsParametersDTO {
    pub user_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetWalletHistoricalStatisticsResultItemDTO {
    pub stock: String,
    pub operation: StockOrderTransactionOperation,
    pub first_operation_date: DateTime<Utc>,
    pub first_operation_value: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetWalletHistoricalStatisticsResultDTO {
    pub data: Vec<GetWalletHistoricalStatisticsResultItemDTO>,
}

//  //  //

#[async_trait]
pub trait StockOrderTransactionDAOGateway: Send + Sync {
    async fn get_wallet(
        &self,
        params: GetWalletParametersDTO,
    ) -> Result<GetWalletResultDTO, Box<dyn std::error::Error + Send + Sync>>;
    async fn get_wallet_historical_statistics(
        &self,
        params: GetWalletHistoricalStatisticsParametersDTO,
    ) -> Result<GetWalletHistoricalStatisticsResultDTO, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait StockOrderTransactionDAOGatewayConstructor {
    async fn new() -> Self;
}
