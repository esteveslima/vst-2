use crate::features::transactions_worker::adapters::entrypoints::consumers::stock_order_consumer::{StockOrderConsumer, StockOrderConsumerImpl, StockOrderConsumerConstructor};

use super::transaction_use_cases_factory::TransactionUseCases;

pub trait TransactionEntrypointsFactory<'a> {
    fn build(use_cases: &'a TransactionUseCases) -> TransactionEntrypoints<'a>;
}

//  //  //

pub struct TransactionEntrypoints<'a> {
    pub stock_order_consumer: Box<dyn StockOrderConsumer + 'a>,
}

//  //  //

impl<'a> TransactionEntrypointsFactory<'a> for TransactionEntrypoints<'a> {
    fn build(use_cases: &'a TransactionUseCases) -> TransactionEntrypoints<'a> {
        let TransactionUseCases {
            create_stock_order_transaction_use_case,
        } = use_cases;

        TransactionEntrypoints {
            stock_order_consumer: Box::new(StockOrderConsumerImpl::new(
                &create_stock_order_transaction_use_case,
            )),
        }
    }
}
