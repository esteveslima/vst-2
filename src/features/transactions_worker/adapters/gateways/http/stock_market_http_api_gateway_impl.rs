#![allow(unused_imports)]
use async_trait::async_trait;
use rand::Rng;

use crate::features::transactions_worker::application::interfaces::gateways::http::stock_market_http_api_gateway::{
        GetStockDataStockMarketGatewayParametersDTO,
        GetStockDataStockMarketGatewayResultDTO,
        StockMarketHttpAPIGateway,
        StockMarketHttpAPIGatewayConstructor, PurchaseStockStockMarketGatewayParametersDTO, SellStockStockMarketGatewayParametersDTO, StockMarketTransactionOperationGatewayResultDTO
};

use super::dtos::nasdaq_api_fetch_stock_data_dto::NasdaqAPIFetchStockDataResultDTO;

// TODO: replace stock API provider
// P.S.: API requests disabled because the requests are being stalled/denied by the server, mocking the result for now
pub struct StockMarketHttpAPIGatewayImpl {
    _api_host: String,
    _http_client: reqwest::Client,
}

//  //  //

impl<'a> StockMarketHttpAPIGatewayConstructor for StockMarketHttpAPIGatewayImpl {
    fn new() -> Self {
        StockMarketHttpAPIGatewayImpl {
            _api_host: "https://api.nasdaq.com".to_string(),
            _http_client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .expect("Failed to create http client"),
        }
    }
}

#[async_trait]
impl StockMarketHttpAPIGateway for StockMarketHttpAPIGatewayImpl {
    async fn get_stock_data(
        &self,
        params: GetStockDataStockMarketGatewayParametersDTO,
    ) -> Result<GetStockDataStockMarketGatewayResultDTO, Box<dyn std::error::Error + Send + Sync>>
    {
        let GetStockDataStockMarketGatewayParametersDTO { stock } = params;

        // let api_response = self
        //     .http_client
        //     .get(&format!(
        //         "{}/api/quote/{}/info", self.api_host, stock
        //     ))
        //     .query(&[("assetclass", "stocks")])
        //     .send()
        //     .await?
        //     .json::<NasdaqAPIFetchStockDataResultDTO>()
        //     .await?;

        // P.S.: Mocking the result
        let result = GetStockDataStockMarketGatewayResultDTO {
            stock,
            price: rand::prelude::thread_rng().gen_range(0.01..1000.0) as f32,
        };

        Ok(result)
    }

    // Mock method, simulating stock transaction
    async fn purchase_stock(
        &self,
        params: PurchaseStockStockMarketGatewayParametersDTO,
    ) -> Result<
        StockMarketTransactionOperationGatewayResultDTO,
        Box<dyn std::error::Error + Send + Sync>,
    > {
        let PurchaseStockStockMarketGatewayParametersDTO { shares, stock } = params;

        // P.S.: Mocking the result
        let result = StockMarketTransactionOperationGatewayResultDTO {
            stock,
            shares,
            price: rand::prelude::thread_rng().gen_range(0.01..1000.0) as f32,
        };

        Ok(result)
    }

    // Mock method, simulating stock transaction
    async fn sell_stock(
        &self,
        params: SellStockStockMarketGatewayParametersDTO,
    ) -> Result<
        StockMarketTransactionOperationGatewayResultDTO,
        Box<dyn std::error::Error + Send + Sync>,
    > {
        let SellStockStockMarketGatewayParametersDTO { shares, stock } = params;

        // P.S.: Mocking the result
        let result = StockMarketTransactionOperationGatewayResultDTO {
            stock,
            shares,
            price: rand::prelude::thread_rng().gen_range(0.01..1000.0) as f32,
        };

        Ok(result)
    }
}
