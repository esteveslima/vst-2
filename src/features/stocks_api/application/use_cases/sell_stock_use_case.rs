use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std;

use crate::features::stocks_api::application::interfaces::{
    gateways::stock_producer_gateway::{SellStockEventParametersDTO, StockProducerGateway},
    use_cases::use_case::UseCase,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct SellStockParametersPayloadDTO {
    pub stock: String,
    pub shares: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SellStockParametersDTO {
    pub user_id: usize,
    pub payload: SellStockParametersPayloadDTO,
}

//

#[derive(Deserialize, Serialize, Debug)]
pub struct SellStockResultDTO {
    pub id: usize,
    pub stock: String,
    pub shares: usize,
    pub price: f32,
}

// // //

pub struct SellStockUseCaseImpl<'a> {
    stock_producer_gateway: &'a Box<dyn StockProducerGateway + 'a>,
}

pub trait SellStockUseCaseConstructor<'a> {
    fn new(stock_producer_gateway: &'a Box<dyn StockProducerGateway + 'a>) -> Self;
}

#[async_trait]
pub trait SellStockUseCase: UseCase<SellStockParametersDTO, SellStockResultDTO> {}

impl<'a> SellStockUseCaseConstructor<'a> for SellStockUseCaseImpl<'a> {
    fn new(stock_producer_gateway: &'a Box<dyn StockProducerGateway + 'a>) -> Self {
        SellStockUseCaseImpl {
            stock_producer_gateway,
        }
    }
}

impl<'a> SellStockUseCase for SellStockUseCaseImpl<'a> {}

#[async_trait]
impl<'a> UseCase<SellStockParametersDTO, SellStockResultDTO> for SellStockUseCaseImpl<'a> {
    async fn execute(
        &self,
        params: SellStockParametersDTO,
    ) -> Result<SellStockResultDTO, Box<dyn std::error::Error>> {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        let SellStockParametersDTO {
            user_id: _,
            payload: SellStockParametersPayloadDTO { stock, shares },
        } = params;

        // if params.shares <= 0 {
        //     return Err(Box::new(std::io::Error::new(
        //         std::io::ErrorKind::InvalidInput,
        //         format!("Error: {}", 123),
        //     )));
        // }

        let _produce_result = self
            .stock_producer_gateway
            .produce_event_sell_stock(SellStockEventParametersDTO {
                shares: shares.clone(),
                stock: stock.clone(),
            })
            .await;

        let result = SellStockResultDTO {
            id: 0,
            stock: stock,
            shares: shares,
            price: 123.45,
        };

        return Ok(result);
    }
}
