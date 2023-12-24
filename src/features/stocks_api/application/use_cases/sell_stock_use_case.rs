use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std;

use crate::{
    application::interfaces::use_case::UseCase,
    features::stocks_api::application::interfaces::gateways::producers::stock_producer_gateway::{
        OrderPayloadDTO, ProduceSellStockOrderParametersDTO, StockProducerGateway,
    },
};

#[derive(Deserialize, Serialize, Debug)]
pub struct SellStockParametersPayloadDTO {
    pub stock: String,
    pub shares: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SellStockParametersDTO {
    pub user_id: String,
    pub payload: SellStockParametersPayloadDTO,
}

//

#[derive(Deserialize, Serialize, Debug)]
pub struct SellStockResultDTO {
    pub id: String,
    pub stock: String,
    pub shares: usize,
    pub price: f32,
}

// // //

pub trait SellStockUseCaseConstructor<'a> {
    fn new(stock_producer_gateway: &'a Box<dyn StockProducerGateway + 'a>) -> Self;
}

#[async_trait]
pub trait SellStockUseCase: UseCase<SellStockParametersDTO, SellStockResultDTO> {}

//  //  //

pub struct SellStockUseCaseImpl<'a> {
    stock_producer_gateway: &'a Box<dyn StockProducerGateway + 'a>,
}

//  //  //

impl<'a> SellStockUseCase for SellStockUseCaseImpl<'a> {}

impl<'a> SellStockUseCaseConstructor<'a> for SellStockUseCaseImpl<'a> {
    fn new(stock_producer_gateway: &'a Box<dyn StockProducerGateway + 'a>) -> Self {
        SellStockUseCaseImpl {
            stock_producer_gateway,
        }
    }
}

#[async_trait]
impl<'a> UseCase<SellStockParametersDTO, SellStockResultDTO> for SellStockUseCaseImpl<'a> {
    async fn execute(
        &self,
        params: SellStockParametersDTO,
    ) -> Result<SellStockResultDTO, Box<dyn std::error::Error + Send + Sync>> {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        let SellStockParametersDTO {
            user_id,
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
            .produce_sell_stock_order(ProduceSellStockOrderParametersDTO {
                user_id: user_id.to_string(),
                payload: OrderPayloadDTO {
                    shares: shares.clone(),
                    stock: stock.clone(),
                },
            })
            .await;

        let result = SellStockResultDTO {
            id: 0.to_string(),
            stock: stock,
            shares: shares,
            price: 123.45,
        };

        return Ok(result);
    }
}
