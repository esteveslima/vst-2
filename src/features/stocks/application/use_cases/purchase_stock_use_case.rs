use std;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::features::stocks::application::interfaces::use_case::UseCase;

#[derive(Deserialize, Serialize, Debug)]
pub struct PurchaseStockParametersDTO {
    pub stock: String,
    pub shares: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PurchaseStockResultDTO {
    pub stock: String,
    pub shares: usize,
    pub price: f32,
}

//

pub struct PurchaseStockUseCase {}

#[async_trait]
impl UseCase<PurchaseStockParametersDTO, PurchaseStockResultDTO> for PurchaseStockUseCase {
    async fn execute(
        params: PurchaseStockParametersDTO,
    ) -> Result<PurchaseStockResultDTO, Box<dyn std::error::Error>> {
        let result = PurchaseStockResultDTO {
            stock: params.stock,
            shares: params.shares,
            price: 123.00,
        };
    
        return Ok(result);
    }
}
