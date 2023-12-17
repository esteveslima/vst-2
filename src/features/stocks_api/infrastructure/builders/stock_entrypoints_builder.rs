use crate::features::stocks_api::adapters::entrypoints::{
    consumers::stock_consumer::{StockConsumer, StockConsumerConstructor, StockConsumerImpl},
    controllers::stock_controller::{
        StockController, StockControllerConstructor, StockControllerImpl,
    },
};

use super::stock_use_cases_builder::StockUseCases;

pub trait StockEntrypointsBuilder<'a> {
    fn build(use_cases: &'a StockUseCases) -> StockEntrypoints<'a>;
}

//  //  //

pub struct StockEntrypoints<'a> {
    pub stock_controller: Box<dyn StockController + 'a>,
    pub stock_consumer: Box<dyn StockConsumer + 'a>,
}

//  //  //

impl<'a> StockEntrypointsBuilder<'a> for StockEntrypoints<'a> {
    fn build(use_cases: &'a StockUseCases) -> StockEntrypoints<'a> {
        let StockUseCases {
            get_stocks_summary_use_case,
            purchase_stock_use_case,
            sell_stock_use_case,
            test_consume_use_case,
        } = use_cases;

        StockEntrypoints {
            stock_controller: Box::new(StockControllerImpl::new(
                &get_stocks_summary_use_case,
                &purchase_stock_use_case,
                &sell_stock_use_case,
            )),
            stock_consumer: Box::new(StockConsumerImpl::new(&test_consume_use_case)),
        }
    }
}
