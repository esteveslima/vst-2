use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PurchaseStockEventParametersDTO {
    pub stock: String,
    pub shares: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SellStockEventParametersDTO {
    pub stock: String,
    pub shares: usize,
}

//

#[async_trait]
pub trait StockProducerGateway: Send + Sync {
    async fn produce_event_purchase_stock(
        &self,
        params: PurchaseStockEventParametersDTO,
    ) -> Result<(), Box<dyn std::error::Error>>;
    async fn produce_event_sell_stock(
        &self,
        params: SellStockEventParametersDTO,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait StockProducerGatewayConstructor {
    fn new() -> Self;
}
