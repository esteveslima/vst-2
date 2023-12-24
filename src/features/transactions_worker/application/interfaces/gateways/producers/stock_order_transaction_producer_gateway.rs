use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::features::transactions_worker::application::use_cases::create_stock_order_transaction_use_case::OrderOperation;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum StockOrderTransactionStatus {
    SUCCESS,
    FAIL,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProduceStockOrderTransactionParametersPayloadDTO {
    pub status: StockOrderTransactionStatus,
    pub operation: OrderOperation,
    pub stock: String,
    pub shares: usize,
    pub price: f32,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ProduceStockOrderTransactionParametersDTO {
    pub user_id: String, // using the user id to ensure ordering
    pub payload: ProduceStockOrderTransactionParametersPayloadDTO,
}

//

#[async_trait]
pub trait StockOrderTransactionProducerGateway: Send + Sync {
    async fn produce_stock_order_transaction(
        &self,
        params: ProduceStockOrderTransactionParametersDTO,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait StockOrderTransactionProducerGatewayConstructor {
    fn new() -> Self;
}
