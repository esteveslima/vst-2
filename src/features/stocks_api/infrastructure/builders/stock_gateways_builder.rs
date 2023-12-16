use crate::features::stocks_api::{
    adapters::gateways::producers::stock_producer_gateway_impl::StockProducerGatewayImpl,
    application::interfaces::gateways::stock_producer_gateway::{
        StockProducerGateway, StockProducerGatewayConstructor,
    },
};

pub trait StockGatewaysBuilder<'a> {
    fn build() -> StockGateways<'a>;
}

pub struct StockGateways<'a> {
    // TODO: how to make these only references(&'a)? compiler doesn't infer the '...Impl' struct directly into the trait it's implementing
    pub stock_producer_gateway: Box<dyn StockProducerGateway + 'a>,
}

impl<'a> StockGatewaysBuilder<'a> for StockGateways<'a> {
    fn build() -> StockGateways<'a> {
        StockGateways {
            stock_producer_gateway: Box::new(StockProducerGatewayImpl::new()),
        }
    }
}
