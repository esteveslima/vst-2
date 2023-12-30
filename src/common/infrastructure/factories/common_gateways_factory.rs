use async_trait::async_trait;

use crate::common::{
    adapters::gateways::daos::stock_order_transaction_dao_gateway_impl::StockOrderTransactionDAOGatewayImpl,
    application::interfaces::gateways::daos::stock_order_transaction_dao_gateway::{
        StockOrderTransactionDAOGateway, StockOrderTransactionDAOGatewayConstructor,
    },
};

#[async_trait]
pub trait CommonGatewaysFactory<'a> {
    async fn build() -> CommonGateways<'a>;
}

//  //  //

pub struct CommonGateways<'a> {
    pub stock_order_transaction_dao_gateway: Box<dyn StockOrderTransactionDAOGateway + 'a>,
}

//  //  //

#[async_trait]
impl<'a> CommonGatewaysFactory<'a> for CommonGateways<'a> {
    async fn build() -> CommonGateways<'a> {
        CommonGateways {
            stock_order_transaction_dao_gateway: Box::new(
                StockOrderTransactionDAOGatewayImpl::new().await,
            ),
        }
    }
}
