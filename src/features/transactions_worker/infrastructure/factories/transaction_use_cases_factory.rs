use crate::features::transactions_worker::application::use_cases::create_stock_order_transaction_use_case::{CreateStockOrderTransactionUseCase, CreateStockOrderTransactionUseCaseImpl, CreateStockOrderTransactionUseCaseConstructor};

use super::transaction_gateways_factory::TransactionGateways;

pub trait TransactionUseCasesFactory<'a> {
    fn build(gateways: &'a TransactionGateways) -> TransactionUseCases<'a>;
}

//  //  //

pub struct TransactionUseCases<'a> {
    pub create_stock_order_transaction_use_case: Box<dyn CreateStockOrderTransactionUseCase + 'a>,
}

//  //  //

impl<'a> TransactionUseCasesFactory<'a> for TransactionUseCases<'a> {
    fn build(gateways: &'a TransactionGateways) -> TransactionUseCases<'a> {
        let TransactionGateways {
            stock_order_transaction_producer_gateway,
        } = gateways;

        TransactionUseCases {
            create_stock_order_transaction_use_case: Box::new(
                CreateStockOrderTransactionUseCaseImpl::new(
                    &stock_order_transaction_producer_gateway,
                ),
            ),
        }
    }
}
