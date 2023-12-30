use async_trait::async_trait;

use crate::{
    common::application::interfaces::{
        gateways::daos::stock_order_transaction_dao_gateway::{
            GetWalletParametersDTO, StockOrderTransactionDAOGateway,
        },
        use_case::UseCase,
    },
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
        stock_order_transaction_dao_gateway: &'a Box<dyn StockOrderTransactionDAOGateway + 'a>,
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
    stock_order_transaction_dao_gateway: &'a Box<dyn StockOrderTransactionDAOGateway + 'a>,
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
        stock_order_transaction_dao_gateway: &'a Box<dyn StockOrderTransactionDAOGateway + 'a>,
        stock_market_http_api_gateway: &'a Box<dyn StockMarketHttpAPIGateway + 'a>,
        stock_order_transaction_producer_gateway: &'a Box<
            dyn StockOrderTransactionProducerGateway + 'a,
        >,
    ) -> Self {
        CreateStockOrderTransactionUseCaseImpl {
            stock_order_transaction_dao_gateway,
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

        if operation == StockOrderTransactionOperation::SELL {
            let wallet_data = self
                .stock_order_transaction_dao_gateway
                .get_wallet(GetWalletParametersDTO {
                    user_id: user_id.clone(),
                })
                .await?;

            let matching_stock_wallet = wallet_data
                .wallet
                .iter()
                .find(|stock_wallet| stock_wallet.stock == stock)
                .unwrap();

            let has_enough_shares = matching_stock_wallet.total_shares >= shares as i64;
            if !has_enough_shares {
                eprintln!(
                    "User({}) doesn't have enough stock({}) shares to sell({} < {})",
                    user_id, stock, matching_stock_wallet.total_shares, shares
                );
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

                return Ok(());
            }
        }

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
