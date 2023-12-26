use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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
pub struct PurchaseStockParametersPayloadDTO {
    pub stock: String,
    pub shares: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PurchaseStockParametersDTO {
    pub user_id: String,
    pub payload: PurchaseStockParametersPayloadDTO,
}

//

#[derive(Deserialize, Serialize, Debug)]
pub struct PurchaseStockResultDTO {
    pub id: String,
    pub stock: String,
    pub shares: usize,
}

// // //

pub trait PurchaseStockUseCaseConstructor<'a> {
    fn new(stock_producer_gateway: &'a Box<dyn StockOrderProducerGateway + 'a>) -> Self;
}

#[async_trait]
pub trait PurchaseStockUseCase:
    UseCase<PurchaseStockParametersDTO, PurchaseStockResultDTO>
{
}

//  //  //

pub struct PurchaseStockUseCaseImpl<'a> {
    stock_producer_gateway: &'a Box<dyn StockOrderProducerGateway + 'a>,
}

//  //  //

impl<'a> PurchaseStockUseCase for PurchaseStockUseCaseImpl<'a> {}

impl<'a> PurchaseStockUseCaseConstructor<'a> for PurchaseStockUseCaseImpl<'a> {
    fn new(stock_producer_gateway: &'a Box<dyn StockOrderProducerGateway + 'a>) -> Self {
        PurchaseStockUseCaseImpl {
            stock_producer_gateway,
        }
    }
}

#[async_trait]
impl<'a> UseCase<PurchaseStockParametersDTO, PurchaseStockResultDTO>
    for PurchaseStockUseCaseImpl<'a>
{
    async fn execute(
        &self,
        params: PurchaseStockParametersDTO,
    ) -> Result<PurchaseStockResultDTO, Box<dyn std::error::Error + Send + Sync>> {
        let PurchaseStockParametersDTO {
            user_id,
            payload: PurchaseStockParametersPayloadDTO { stock, shares },
        } = params;

        let produce_result = self
            .stock_producer_gateway
            .produce_stock_order(ProduceStockOrderParametersDTO {
                user_id: user_id.to_string(),
                payload: ProduceStockOrderPayloadParametersDTO {
                    operation: StockOrderOperation::PURCHASE,
                    shares: shares.clone(),
                    stock: stock.clone(),
                },
            })
            .await?;

        Ok(PurchaseStockResultDTO {
            id: produce_result.id,
            stock,
            shares,
        })
    }
}
