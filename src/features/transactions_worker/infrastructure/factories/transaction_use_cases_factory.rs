use crate::{features::transactions_worker::application::use_cases::create_stock_order_transaction_use_case::{CreateStockOrderTransactionUseCase, CreateStockOrderTransactionUseCaseImpl, CreateStockOrderTransactionUseCaseConstructor}, common::infrastructure::factories::common_gateways_factory::CommonGateways};

use super::transaction_gateways_factory::TransactionGateways;

pub trait TransactionUseCasesFactory<'a> {
    fn build(
        common_gateways: &'a CommonGateways,
        transaction_gateways: &'a TransactionGateways,
    ) -> TransactionUseCases<'a>;
}

//  //  //

pub struct TransactionUseCases<'a> {
    pub create_stock_order_transaction_use_case: Box<dyn CreateStockOrderTransactionUseCase + 'a>,
}

//  //  //

impl<'a> TransactionUseCasesFactory<'a> for TransactionUseCases<'a> {
    fn build(
        common_gateways: &'a CommonGateways,
        transaction_gateways: &'a TransactionGateways,
    ) -> TransactionUseCases<'a> {
        let CommonGateways {
            stock_order_transaction_dao_gateway,
        } = common_gateways;
        let TransactionGateways {
            stock_market_http_api_gateway,
            stock_order_transaction_producer_gateway,
        } = transaction_gateways;

        TransactionUseCases {
            create_stock_order_transaction_use_case: Box::new(
                CreateStockOrderTransactionUseCaseImpl::new(
                    &stock_order_transaction_dao_gateway,
                    &stock_market_http_api_gateway,
                    &stock_order_transaction_producer_gateway,
                ),
            ),
        }
    }
}
