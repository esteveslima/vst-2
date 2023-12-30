use async_trait::async_trait;

use crate::features::stocks_api::{
    adapters::gateways::{
        daos::stock_order_transaction_dao_gateway_impl::StockOrderTransactionDAOGatewayImpl,
        producers::stock_order_producer_gateway_impl::StockOrderProducerGatewayImpl,
    },
    application::interfaces::gateways::{
        daos::stock_order_transaction_dao_gateway::{
            StockOrderTransactionDAOGateway, StockOrderTransactionDAOGatewayConstructor,
        },
        producers::stock_order_producer_gateway::{
            StockOrderProducerGateway, StockOrderProducerGatewayConstructor,
        },
    },
};

#[async_trait]
pub trait StockGatewaysFactory<'a> {
    async fn build() -> StockGateways<'a>;
}

//  //  //

pub struct StockGateways<'a> {
    pub stock_producer_gateway: Box<dyn StockOrderProducerGateway + 'a>,
    pub stock_order_transaction_dao_gateway: Box<dyn StockOrderTransactionDAOGateway + 'a>,
}

//  //  //

#[async_trait]
impl<'a> StockGatewaysFactory<'a> for StockGateways<'a> {
    async fn build() -> StockGateways<'a> {
        StockGateways {
            stock_producer_gateway: Box::new(StockOrderProducerGatewayImpl::new()),
            stock_order_transaction_dao_gateway: Box::new(
                StockOrderTransactionDAOGatewayImpl::new().await,
            ),
        }
    }
}
