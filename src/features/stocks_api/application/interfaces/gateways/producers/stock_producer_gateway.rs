use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderPayloadDTO {
    pub stock: String,
    pub shares: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProducePurchaseStockOrderParametersDTO {
    pub user_id: String, // using the user id to ensure ordering
    pub payload: OrderPayloadDTO,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProduceSellStockOrderParametersDTO {
    pub user_id: String, // using the user id to ensure ordering
    pub payload: OrderPayloadDTO,
}

//

#[async_trait]
pub trait StockProducerGateway: Send + Sync {
    async fn produce_purchase_stock_order(
        &self,
        params: ProducePurchaseStockOrderParametersDTO,
    ) -> Result<(), Box<dyn std::error::Error>>;
    async fn produce_sell_stock_order(
        &self,
        params: ProduceSellStockOrderParametersDTO,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait StockProducerGatewayConstructor {
    fn new() -> Self;
}
