use crate::features::stocks_api::{
    adapters::gateways::producers::stock_producer_gateway_impl::StockProducerGatewayImpl,
    application::interfaces::gateways::stock_producer_gateway::{
        StockProducerGateway, StockProducerGatewayConstructor,
    },
};

pub trait StockGatewaysFactory<'a> {
    fn build() -> StockGateways<'a>;
}

//  //  //

pub struct StockGateways<'a> {
    pub stock_producer_gateway: Box<dyn StockProducerGateway + 'a>,
}

//  //  //

impl<'a> StockGatewaysFactory<'a> for StockGateways<'a> {
    fn build() -> StockGateways<'a> {
        StockGateways {
            stock_producer_gateway: Box::new(StockProducerGatewayImpl::new()),
        }
    }
}
