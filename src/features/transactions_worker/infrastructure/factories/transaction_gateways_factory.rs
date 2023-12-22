use crate::features::transactions_worker::{
    adapters::gateways::producers::stock_order_transaction_producer_gateway_impl::StockOrderTransactionProducerGatewayImpl,
    application::interfaces::gateways::producers::stock_order_transaction_producer_gateway::{
        StockOrderTransactionProducerGateway, StockOrderTransactionProducerGatewayConstructor,
    },
};

pub trait TransactionGatewaysFactory<'a> {
    fn build() -> TransactionGateways<'a>;
}

//  //  //

pub struct TransactionGateways<'a> {
    pub stock_order_transaction_producer_gateway:
        Box<dyn StockOrderTransactionProducerGateway + 'a>,
}

//  //  //

impl<'a> TransactionGatewaysFactory<'a> for TransactionGateways<'a> {
    fn build() -> TransactionGateways<'a> {
        TransactionGateways {
            stock_order_transaction_producer_gateway: Box::new(
                StockOrderTransactionProducerGatewayImpl::new(),
            ),
        }
    }
}
