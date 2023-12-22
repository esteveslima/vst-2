use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    application::interfaces::use_case::UseCase,
    features::stocks_api::application::interfaces::gateways::producers::stock_producer_gateway::{
        OrderPayloadDTO, ProducePurchaseStockOrderParametersDTO, StockProducerGateway,
    },
};

#[derive(Deserialize, Serialize, Debug)]
pub struct PurchaseStockParametersPayloadDTO {
    pub stock: String,
    pub shares: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PurchaseStockParametersDTO {
    pub user_id: usize,
    pub payload: PurchaseStockParametersPayloadDTO,
}

//

#[derive(Deserialize, Serialize, Debug)]
pub struct PurchaseStockResultDTO {
    pub id: usize,
    pub stock: String,
    pub shares: usize,
    pub price: f32,
}

// // //

pub trait PurchaseStockUseCaseConstructor<'a> {
    fn new(stock_producer_gateway: &'a Box<dyn StockProducerGateway + 'a>) -> Self;
}

#[async_trait]
pub trait PurchaseStockUseCase:
    UseCase<PurchaseStockParametersDTO, PurchaseStockResultDTO>
{
}

//  //  //

pub struct PurchaseStockUseCaseImpl<'a> {
    stock_producer_gateway: &'a Box<dyn StockProducerGateway + 'a>,
}

//  //  //

impl<'a> PurchaseStockUseCase for PurchaseStockUseCaseImpl<'a> {}

impl<'a> PurchaseStockUseCaseConstructor<'a> for PurchaseStockUseCaseImpl<'a> {
    fn new(stock_producer_gateway: &'a Box<dyn StockProducerGateway + 'a>) -> Self {
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
    ) -> Result<PurchaseStockResultDTO, Box<dyn std::error::Error>> {
        let PurchaseStockParametersDTO {
            user_id,
            payload: PurchaseStockParametersPayloadDTO { stock, shares },
        } = params;

        let _produce_result = self
            .stock_producer_gateway
            .produce_purchase_stock_order(ProducePurchaseStockOrderParametersDTO {
                user_id: user_id.to_string(),
                payload: OrderPayloadDTO {
                    shares: shares.clone(),
                    stock: stock.clone(),
                },
            })
            .await;

        let result = PurchaseStockResultDTO {
            id: 0,
            stock: stock,
            shares: shares,
            price: 123.00,
        };

        return Ok(result);
    }
}
