use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std;

use crate::{
    application::interfaces::use_case::UseCase,
    features::stocks_api::{
        application::interfaces::gateways::producers::stock_producer_gateway::{
            ProduceStockOrderParametersDTO, ProduceStockOrderPayloadParametersDTO,
            StockOrderProducerGateway,
        },
        domain::entities::stock_order::StockOrderOperation,
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
}

// // //

pub trait SellStockUseCaseConstructor<'a> {
    fn new(stock_producer_gateway: &'a Box<dyn StockOrderProducerGateway + 'a>) -> Self;
}

#[async_trait]
pub trait SellStockUseCase: UseCase<SellStockParametersDTO, SellStockResultDTO> {}

//  //  //

pub struct SellStockUseCaseImpl<'a> {
    stock_producer_gateway: &'a Box<dyn StockOrderProducerGateway + 'a>,
}

//  //  //

impl<'a> SellStockUseCase for SellStockUseCaseImpl<'a> {}

impl<'a> SellStockUseCaseConstructor<'a> for SellStockUseCaseImpl<'a> {
    fn new(stock_producer_gateway: &'a Box<dyn StockOrderProducerGateway + 'a>) -> Self {
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
        let SellStockParametersDTO {
            user_id,
            payload: SellStockParametersPayloadDTO { stock, shares },
        } = params;

        let produce_result = self
            .stock_producer_gateway
            .produce_stock_order(ProduceStockOrderParametersDTO {
                user_id: user_id.to_string(),
                payload: ProduceStockOrderPayloadParametersDTO {
                    operation: StockOrderOperation::SELL,
                    shares: shares.clone(),
                    stock: stock.clone(),
                },
            })
            .await?;

        Ok(SellStockResultDTO {
            id: produce_result.id,
            stock,
            shares,
        })
    }
}
