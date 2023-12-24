use crate::features::transactions_worker::{
    adapters::gateways::{
        http::stock_market_http_api_gateway_impl::StockMarketHttpAPIGatewayImpl,
        producers::stock_order_transaction_producer_gateway_impl::StockOrderTransactionProducerGatewayImpl,
    },
    application::interfaces::gateways::{
        http::stock_market_http_api_gateway::{
            StockMarketHttpAPIGateway, StockMarketHttpAPIGatewayConstructor,
        },
        producers::stock_order_transaction_producer_gateway::{
            StockOrderTransactionProducerGateway, StockOrderTransactionProducerGatewayConstructor,
        },
    },
};

pub trait TransactionGatewaysFactory<'a> {
    fn build() -> TransactionGateways<'a>;
}

//  //  //

pub struct TransactionGateways<'a> {
    pub stock_market_http_api_gateway: Box<dyn StockMarketHttpAPIGateway + 'a>,
    pub stock_order_transaction_producer_gateway:
        Box<dyn StockOrderTransactionProducerGateway + 'a>,
}

//  //  //

impl<'a> TransactionGatewaysFactory<'a> for TransactionGateways<'a> {
    fn build() -> TransactionGateways<'a> {
        TransactionGateways {
            stock_market_http_api_gateway: Box::new(StockMarketHttpAPIGatewayImpl::new()),
            stock_order_transaction_producer_gateway: Box::new(
                StockOrderTransactionProducerGatewayImpl::new(),
            ),
        }
    }
}
