use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::features::stocks_api::domain::entities::stock_order::StockOrderOperation;

#[derive(Deserialize, Serialize, Debug)]
pub struct ProduceStockOrderPayloadParametersDTO {
    pub operation: StockOrderOperation,
    pub stock: String,
    pub shares: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProduceStockOrderParametersDTO {
    pub user_id: String, // using the user id to ensure ordering
    pub payload: ProduceStockOrderPayloadParametersDTO,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProduceStockOrderResultDTO {
    pub id: String,
}

//

#[async_trait]
pub trait StockOrderProducerGateway: Send + Sync {
    async fn produce_stock_order(
        &self,
        params: ProduceStockOrderParametersDTO,
    ) -> Result<ProduceStockOrderResultDTO, Box<dyn std::error::Error + Send + Sync>>;
}

pub trait StockOrderProducerGatewayConstructor {
    fn new() -> Self;
}
