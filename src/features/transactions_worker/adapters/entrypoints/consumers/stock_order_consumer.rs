use async_trait::async_trait;
use std::{convert::Infallible, str::FromStr};
use validator::Validate;

use crate::features::transactions_worker::application::use_cases::create_stock_order_transaction_use_case::{CreateStockOrderTransactionUseCase, CreateStockOrderTransactionUseCaseParametersDTO, CreateStockOrderTransactionPayloadDTO, OrderOperation};

use super::dtos::stock_order_stream_consume_data_dto::StockOrderStreamConsumeDataDTO;

pub trait StockOrderConsumerConstructor<'a> {
    fn new(
        create_stock_order_transaction_use_case: &'a Box<
            dyn CreateStockOrderTransactionUseCase + 'a,
        >,
    ) -> Self;
}

#[async_trait]
pub trait StockOrderConsumer: Sync {
    async fn handle_consume_stock_order(
        &self,
        key: Option<String>,
        payload: String,
    ) -> Result<(), Infallible>;
}

//  //  //

pub struct StockOrderConsumerImpl<'a> {
    create_stock_order_transaction_use_case: &'a Box<dyn CreateStockOrderTransactionUseCase + 'a>,
}

//  //  //

impl<'a> StockOrderConsumerConstructor<'a> for StockOrderConsumerImpl<'a> {
    fn new(
        create_stock_order_transaction_use_case: &'a Box<
            dyn CreateStockOrderTransactionUseCase + 'a,
        >,
    ) -> Self {
        StockOrderConsumerImpl {
            create_stock_order_transaction_use_case,
        }
    }
}

#[async_trait]
impl<'a> StockOrderConsumer for StockOrderConsumerImpl<'a> {
    async fn handle_consume_stock_order(
        &self,
        key: Option<String>,
        payload: String,
    ) -> Result<(), Infallible> {
        if key.is_none() {
            println!("Required key not found: ({:?})", key);
            return Ok(());
        }
        let user_id = key.unwrap();

        let parsed_payload =
            serde_json::from_str::<StockOrderStreamConsumeDataDTO>(&payload.as_str())
                .expect(&format!("Failed to parse payload: {}", payload));
        match parsed_payload.validate() {
            Ok(_) => (),
            Err(error) => {
                println!(
                    "Invalid payload data: ({:?}). Error: {}",
                    parsed_payload, error
                );
                return Ok(());
            }
        }

        let StockOrderStreamConsumeDataDTO {
            operation,
            stock,
            shares,
        } = parsed_payload;

        let mapped_operation_enum = OrderOperation::from_str(&operation)
            .expect(&format!("Invalid operation: {}", operation));

        let params = CreateStockOrderTransactionUseCaseParametersDTO {
            user_id,
            payload: CreateStockOrderTransactionPayloadDTO {
                operation: mapped_operation_enum,
                stock,
                shares,
            },
        };

        let _use_case_result = self
            .create_stock_order_transaction_use_case
            .execute(params)
            .await;

        Ok(())
    }

    //...
}
