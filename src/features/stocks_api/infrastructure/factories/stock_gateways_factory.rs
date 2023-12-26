use crate::features::stocks_api::{
    adapters::gateways::producers::stock_order_producer_gateway_impl::StockOrderProducerGatewayImpl,
    application::interfaces::gateways::producers::stock_producer_gateway::{
        StockOrderProducerGateway, StockOrderProducerGatewayConstructor,
    },
};

pub trait StockGatewaysFactory<'a> {
    fn build() -> StockGateways<'a>;
}

//  //  //

pub struct StockGateways<'a> {
    pub stock_producer_gateway: Box<dyn StockOrderProducerGateway + 'a>,
}

//  //  //

impl<'a> StockGatewaysFactory<'a> for StockGateways<'a> {
    fn build() -> StockGateways<'a> {
        StockGateways {
            stock_producer_gateway: Box::new(StockOrderProducerGatewayImpl::new()),
        }
    }
}
