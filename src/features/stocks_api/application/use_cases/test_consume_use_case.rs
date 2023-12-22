use async_trait::async_trait;
use std;

use crate::features::stocks_api::application::interfaces::{
    gateways::stock_producer_gateway::StockProducerGateway,
    use_cases::use_case::UseCase,
};

pub struct TestConsumeUseCaseParametersDTO {
    pub data: String,
}

pub struct TestConsumeUseCaseResultDTO {
    pub output: String,
}

// // //

pub trait TestConsumeUseCaseConstructor<'a> {
    fn new(_stock_producer_gateway: &'a Box<dyn StockProducerGateway + 'a>) -> Self;
}

#[async_trait]
pub trait TestConsumeUseCase:
    UseCase<TestConsumeUseCaseParametersDTO, TestConsumeUseCaseResultDTO>
{
}

//  //  //

pub struct TestConsumeUseCaseImpl<'a> {
    _stock_producer_gateway: &'a Box<dyn StockProducerGateway + 'a>,
}

//  //  //

impl<'a> TestConsumeUseCase for TestConsumeUseCaseImpl<'a> {}

impl<'a> TestConsumeUseCaseConstructor<'a> for TestConsumeUseCaseImpl<'a> {
    fn new(_stock_producer_gateway: &'a Box<dyn StockProducerGateway + 'a>) -> Self {
        TestConsumeUseCaseImpl {
            _stock_producer_gateway,
        }
    }
}

#[async_trait]
impl<'a> UseCase<TestConsumeUseCaseParametersDTO, TestConsumeUseCaseResultDTO>
    for TestConsumeUseCaseImpl<'a>
{
    async fn execute(
        &self,
        params: TestConsumeUseCaseParametersDTO,
    ) -> Result<TestConsumeUseCaseResultDTO, Box<dyn std::error::Error>> {
        let TestConsumeUseCaseParametersDTO { data } = params;
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        Ok(TestConsumeUseCaseResultDTO { output: data })
    }
}
