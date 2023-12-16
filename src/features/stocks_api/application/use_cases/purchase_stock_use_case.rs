use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::features::stocks_api::application::interfaces::{
    gateways::stock_producer_gateway::{PurchaseStockEventParametersDTO, StockProducerGateway},
    use_cases::use_case::UseCase,
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

pub struct PurchaseStockUseCaseImpl<'a> {
    stock_producer_gateway: &'a Box<dyn StockProducerGateway + 'a>,
}

pub trait PurchaseStockUseCaseConstructor<'a> {
    fn new(stock_producer_gateway: &'a Box<dyn StockProducerGateway + 'a>) -> Self;
}

#[async_trait]
pub trait PurchaseStockUseCase:
    UseCase<PurchaseStockParametersDTO, PurchaseStockResultDTO>
{
}

impl<'a> PurchaseStockUseCaseConstructor<'a> for PurchaseStockUseCaseImpl<'a> {
    fn new(stock_producer_gateway: &'a Box<dyn StockProducerGateway + 'a>) -> Self {
        PurchaseStockUseCaseImpl {
            stock_producer_gateway,
        }
    }
}

impl<'a> PurchaseStockUseCase for PurchaseStockUseCaseImpl<'a> {}

#[async_trait]
impl<'a> UseCase<PurchaseStockParametersDTO, PurchaseStockResultDTO>
    for PurchaseStockUseCaseImpl<'a>
{
    async fn execute(
        &self,
        params: PurchaseStockParametersDTO,
    ) -> Result<PurchaseStockResultDTO, Box<dyn std::error::Error>> {
        let PurchaseStockParametersDTO {
            user_id: _,
            payload: PurchaseStockParametersPayloadDTO { stock, shares },
        } = params;

        let _produce_result = self
            .stock_producer_gateway
            .produce_event_purchase_stock(PurchaseStockEventParametersDTO {
                shares: shares.clone(),
                stock: stock.clone(),
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
