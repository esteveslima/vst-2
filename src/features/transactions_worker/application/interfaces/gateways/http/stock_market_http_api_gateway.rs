use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetStockDataStockMarketGatewayParametersDTO {
    pub stock: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetStockDataStockMarketGatewayResultDTO {
    pub stock: String,
    pub price: f32,
}

//

#[derive(Deserialize, Serialize, Debug)]
pub struct PurchaseStockStockMarketGatewayParametersDTO {
    pub stock: String,
    pub shares: usize,
}

//

#[derive(Deserialize, Serialize, Debug)]
pub struct SellStockStockMarketGatewayParametersDTO {
    pub stock: String,
    pub shares: usize,
}

//

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StockMarketTransactionOperationGatewayResultDTO {
    pub stock: String,
    pub shares: usize,
    pub price: f32,
}

//  //  //

#[async_trait]
pub trait StockMarketHttpAPIGateway: Send + Sync {
    async fn get_stock_data(
        &self,
        params: GetStockDataStockMarketGatewayParametersDTO,
    ) -> Result<GetStockDataStockMarketGatewayResultDTO, Box<dyn std::error::Error + Send + Sync>>;
    async fn purchase_stock(
        &self,
        params: PurchaseStockStockMarketGatewayParametersDTO,
    ) -> Result<
        StockMarketTransactionOperationGatewayResultDTO,
        Box<dyn std::error::Error + Send + Sync>,
    >;
    async fn sell_stock(
        &self,
        params: SellStockStockMarketGatewayParametersDTO,
    ) -> Result<
        StockMarketTransactionOperationGatewayResultDTO,
        Box<dyn std::error::Error + Send + Sync>,
    >;
}

pub trait StockMarketHttpAPIGatewayConstructor {
    fn new() -> Self;
}
