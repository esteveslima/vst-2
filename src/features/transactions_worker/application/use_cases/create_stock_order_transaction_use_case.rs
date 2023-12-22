use async_trait::async_trait;
use std;

use crate::{
    features::transactions_worker::application::interfaces::gateways::producers::stock_order_transaction_producer_gateway::{ProduceStockOrderTransactionParametersDTO, StockOrderTransactionPayloadDTO, StockOrderTransactionProducerGateway},
    application::interfaces::use_case::UseCase
};

pub struct CreateStockOrderTransactionPayloadDTO {
    pub stock: String,
    pub shares: usize,
}

pub struct CreateStockOrderTransactionUseCaseParametersDTO {
    pub user_id: String,
    pub payload: CreateStockOrderTransactionPayloadDTO,
}

// pub struct CreateStockOrderTransactionUseCaseResultDTO {

// }

// // //

pub trait CreateStockOrderTransactionUseCaseConstructor<'a> {
    fn new(
        stock_order_transaction_producer_gateway: &'a Box<
            dyn StockOrderTransactionProducerGateway + 'a,
        >,
    ) -> Self;
}

#[async_trait]
pub trait CreateStockOrderTransactionUseCase:
    UseCase<CreateStockOrderTransactionUseCaseParametersDTO, ()>
{
}

//  //  //

pub struct CreateStockOrderTransactionUseCaseImpl<'a> {
    stock_order_transaction_producer_gateway:
        &'a Box<dyn StockOrderTransactionProducerGateway + 'a>,
}

//  //  //

impl<'a> CreateStockOrderTransactionUseCase for CreateStockOrderTransactionUseCaseImpl<'a> {}

impl<'a> CreateStockOrderTransactionUseCaseConstructor<'a>
    for CreateStockOrderTransactionUseCaseImpl<'a>
{
    fn new(
        stock_order_transaction_producer_gateway: &'a Box<
            dyn StockOrderTransactionProducerGateway + 'a,
        >,
    ) -> Self {
        CreateStockOrderTransactionUseCaseImpl {
            stock_order_transaction_producer_gateway,
        }
    }
}

#[async_trait]
impl<'a> UseCase<CreateStockOrderTransactionUseCaseParametersDTO, ()>
    for CreateStockOrderTransactionUseCaseImpl<'a>
{
    async fn execute(
        &self,
        params: CreateStockOrderTransactionUseCaseParametersDTO,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let CreateStockOrderTransactionUseCaseParametersDTO {
            user_id,
            payload: CreateStockOrderTransactionPayloadDTO { stock, shares },
        } = params;

        // TODO
        // Simulate making the transaction
        let transaction_result: Result<bool, ()> = Ok(true);
        let price = 0;
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        let produce_stock_order_transaction_params = ProduceStockOrderTransactionParametersDTO {
            user_id,
            payload: StockOrderTransactionPayloadDTO {
                stock,
                shares,
                price,
            },
        };

        if transaction_result.is_ok() {
            self.stock_order_transaction_producer_gateway
                .produce_successful_stock_order_transaction(produce_stock_order_transaction_params)
                .await?;
        } else {
            self.stock_order_transaction_producer_gateway
                .produce_failed_stock_order_transaction(produce_stock_order_transaction_params)
                .await?;
        }

        Ok(())
    }
}
