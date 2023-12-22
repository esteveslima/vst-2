use crate::features::stocks_api::application::use_cases::{
    get_stocks_summary_use_case::{
        GetStocksSummaryUseCase, GetStocksSummaryUseCaseConstructor, GetStocksSummaryUseCaseImpl,
    },
    purchase_stock_use_case::{
        PurchaseStockUseCase, PurchaseStockUseCaseConstructor, PurchaseStockUseCaseImpl,
    },
    sell_stock_use_case::{SellStockUseCase, SellStockUseCaseConstructor, SellStockUseCaseImpl},
};

use super::stock_gateways_factory::StockGateways;

pub trait StockUseCasesFactory<'a> {
    fn build(gateways: &'a StockGateways) -> StockUseCases<'a>;
}

//  //  //

pub struct StockUseCases<'a> {
    pub purchase_stock_use_case: Box<dyn PurchaseStockUseCase + 'a>,
    pub sell_stock_use_case: Box<dyn SellStockUseCase + 'a>,
    pub get_stocks_summary_use_case: Box<dyn GetStocksSummaryUseCase + 'a>,
}

//  //  //

impl<'a> StockUseCasesFactory<'a> for StockUseCases<'a> {
    fn build(gateways: &'a StockGateways) -> StockUseCases<'a> {
        let StockGateways {
            stock_producer_gateway,
        } = gateways;

        StockUseCases {
            purchase_stock_use_case: Box::new(PurchaseStockUseCaseImpl::new(
                &stock_producer_gateway,
            )),
            sell_stock_use_case: Box::new(SellStockUseCaseImpl::new(&stock_producer_gateway)),
            get_stocks_summary_use_case: Box::new(GetStocksSummaryUseCaseImpl::new()),
        }
    }
}
