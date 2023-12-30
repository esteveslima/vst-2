use crate::features::stocks_api::adapters::entrypoints::controllers::stock_order_controller::{
    StockOrderController, StockOrderControllerConstructor, StockOrderControllerImpl,
};

use super::stock_use_cases_factory::StockUseCases;

pub trait StockEntrypointsFactory<'a> {
    fn build(use_cases: &'a StockUseCases) -> StockEntrypoints<'a>;
}

//  //  //

pub struct StockEntrypoints<'a> {
    pub stock_controller: Box<dyn StockOrderController + 'a>,
}

//  //  //

impl<'a> StockEntrypointsFactory<'a> for StockEntrypoints<'a> {
    fn build(use_cases: &'a StockUseCases) -> StockEntrypoints<'a> {
        let StockUseCases {
            get_stocks_summary_use_case,
            purchase_stock_use_case,
            sell_stock_use_case,
        } = use_cases;

        StockEntrypoints {
            stock_controller: Box::new(StockOrderControllerImpl::new(
                &get_stocks_summary_use_case,
                &purchase_stock_use_case,
                &sell_stock_use_case,
            )),
        }
    }
}
