use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std;

use crate::features::stocks_api::application::interfaces::use_case::UseCase;

#[derive(Deserialize, Serialize, Debug)]
pub struct PurchaseStockParametersPayloadDTO {
    pub stock: String,
    pub shares: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PurchaseStockParametersDTO {
    pub user_id: usize,
    pub payload: PurchaseStockParametersPayloadDTO,
}

//

#[derive(Deserialize, Serialize, Debug)]
pub struct PurchaseStockResultDTO {
    pub id: usize,
    pub stock: String,
    pub shares: usize,
    pub price: f32,
}

// // //

pub struct PurchaseStockUseCase {}

#[async_trait]
impl UseCase<PurchaseStockParametersDTO, PurchaseStockResultDTO> for PurchaseStockUseCase {
    async fn execute(
        params: PurchaseStockParametersDTO,
    ) -> Result<PurchaseStockResultDTO, Box<dyn std::error::Error>> {
        let result = PurchaseStockResultDTO {
            id: 0,
            stock: params.payload.stock,
            shares: params.payload.shares,
            price: 123.00,
        };

        return Ok(result);
    }
}
