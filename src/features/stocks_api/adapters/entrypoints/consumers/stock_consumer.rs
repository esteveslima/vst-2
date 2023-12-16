use async_trait::async_trait;
use std::convert::Infallible;

use crate::features::stocks_api::application::use_cases::test_consume_use_case::{
    TestConsumeUseCase, TestConsumeUseCaseParametersDTO,
};

pub struct StockConsumerImpl<'a> {
    test_consume_use_case: &'a Box<dyn TestConsumeUseCase + 'a>,
}

pub trait StockConsumerConstructor<'a> {
    fn new(test_consume_use_case: &'a Box<dyn TestConsumeUseCase + 'a>) -> Self;
}

#[async_trait]
pub trait StockConsumer: Sync {
    async fn handle_consume_test_operation(&self, payload: String) -> Result<(), Infallible>;
}

impl<'a> StockConsumerConstructor<'a> for StockConsumerImpl<'a> {
    fn new(test_consume_use_case: &'a Box<dyn TestConsumeUseCase + 'a>) -> Self {
        StockConsumerImpl {
            test_consume_use_case,
        }
    }
}

#[async_trait]
impl<'a> StockConsumer for StockConsumerImpl<'a> {
    async fn handle_consume_test_operation(&self, payload: String) -> Result<(), Infallible> {
        let params = TestConsumeUseCaseParametersDTO {
            data: "".to_string(),
        };

        let _use_case_result = self.test_consume_use_case.execute(params).await;

        println!("{}", payload);

        Ok(())
    }

    //...
}
