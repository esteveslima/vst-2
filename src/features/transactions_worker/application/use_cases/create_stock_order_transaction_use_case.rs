use async_trait::async_trait;

use crate::{
    application::interfaces::use_case::UseCase,
    features::transactions_worker::{
        application::interfaces::gateways::{
            http::stock_market_http_api_gateway::{
                PurchaseStockStockMarketGatewayParametersDTO,
                SellStockStockMarketGatewayParametersDTO, StockMarketHttpAPIGateway,
            },
            producers::stock_order_transaction_producer_gateway::{
                ProduceStockOrderTransactionParametersDTO,
                ProduceStockOrderTransactionParametersPayloadDTO,
                StockOrderTransactionProducerGateway,
            },
        },
        domain::entities::stock_order_transaction::{
            StockOrderTransactionOperation, StockOrderTransactionStatus,
        },
    },
};

pub struct CreateStockOrderTransactionPayloadDTO {
    pub operation: StockOrderTransactionOperation,
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
        stock_market_http_api_gateway: &'a Box<dyn StockMarketHttpAPIGateway + 'a>,
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
    stock_market_http_api_gateway: &'a Box<dyn StockMarketHttpAPIGateway + 'a>,
    stock_order_transaction_producer_gateway:
        &'a Box<dyn StockOrderTransactionProducerGateway + 'a>,
}

//  //  //

impl<'a> CreateStockOrderTransactionUseCase for CreateStockOrderTransactionUseCaseImpl<'a> {}

impl<'a> CreateStockOrderTransactionUseCaseConstructor<'a>
    for CreateStockOrderTransactionUseCaseImpl<'a>
{
    fn new(
        stock_market_http_api_gateway: &'a Box<dyn StockMarketHttpAPIGateway + 'a>,
        stock_order_transaction_producer_gateway: &'a Box<
            dyn StockOrderTransactionProducerGateway + 'a,
        >,
    ) -> Self {
        CreateStockOrderTransactionUseCaseImpl {
            stock_market_http_api_gateway,
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
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: look into anyhow, using box std errors seems kinda problematic
        let CreateStockOrderTransactionUseCaseParametersDTO {
            user_id,
            payload:
                CreateStockOrderTransactionPayloadDTO {
                    operation,
                    stock,
                    shares,
                },
        } = params;

        let transaction_result = {
            match operation {
                StockOrderTransactionOperation::PURCHASE => {
                    self.stock_market_http_api_gateway
                        .purchase_stock(PurchaseStockStockMarketGatewayParametersDTO {
                            stock: stock.clone(),
                            shares,
                        })
                        .await
                }
                StockOrderTransactionOperation::SELL => {
                    self.stock_market_http_api_gateway
                        .sell_stock(SellStockStockMarketGatewayParametersDTO {
                            stock: stock.clone(),
                            shares,
                        })
                        .await
                }
            }
        };

        match transaction_result {
            Ok(transaction) => {
                let _ = self
                    .stock_order_transaction_producer_gateway
                    .produce_stock_order_transaction(ProduceStockOrderTransactionParametersDTO {
                        user_id,
                        payload: ProduceStockOrderTransactionParametersPayloadDTO {
                            status: StockOrderTransactionStatus::SUCCESS,
                            operation,
                            stock,
                            shares,
                            price: transaction.price,
                        },
                    })
                    .await;
            }
            Err(error) => {
                println!("Transaction failed, Error: {}", error);
                let _ = self
                    .stock_order_transaction_producer_gateway
                    .produce_stock_order_transaction(ProduceStockOrderTransactionParametersDTO {
                        user_id,
                        payload: ProduceStockOrderTransactionParametersPayloadDTO {
                            status: StockOrderTransactionStatus::FAIL,
                            operation,
                            stock,
                            shares,
                            price: -0.0,
                        },
                    })
                    .await;
            }
        }
        Ok(())
    }
}
