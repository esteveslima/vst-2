use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct StockOrderTransactionPayloadDTO {
    pub stock: String,
    pub shares: usize,
    pub price: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProduceStockOrderTransactionParametersDTO {
    pub user_id: String, // using the user id to ensure ordering
    pub payload: StockOrderTransactionPayloadDTO,
}

//

#[async_trait]
pub trait StockOrderTransactionProducerGateway: Send + Sync {
    async fn produce_successful_stock_order_transaction(
        &self,
        params: ProduceStockOrderTransactionParametersDTO,
    ) -> Result<(), Box<dyn std::error::Error>>;
    async fn produce_failed_stock_order_transaction(
        &self,
        params: ProduceStockOrderTransactionParametersDTO,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait StockOrderTransactionProducerGatewayConstructor {
    fn new() -> Self;
}
