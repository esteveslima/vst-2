use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std;

use crate::features::stocks_api::application::interfaces::use_case::UseCase;

#[derive(Deserialize, Serialize, Debug)]
pub struct SellStockParametersPayloadDTO {
    pub stock: String,
    pub shares: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SellStockParametersDTO {
    pub user_id: usize,
    pub payload: SellStockParametersPayloadDTO,
}

//

#[derive(Deserialize, Serialize, Debug)]
pub struct SellStockResultDTO {
    pub id: usize,
    pub stock: String,
    pub shares: usize,
    pub price: f32,
}

// // //

pub struct SellStockUseCase {}

#[async_trait]
impl UseCase<SellStockParametersDTO, SellStockResultDTO> for SellStockUseCase {
    async fn execute(
        params: SellStockParametersDTO,
    ) -> Result<SellStockResultDTO, Box<dyn std::error::Error>> {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        // if params.shares <= 0 {
        //     return Err(Box::new(std::io::Error::new(
        //         std::io::ErrorKind::InvalidInput,
        //         format!("Error: {}", 123),
        //     )));
        // }

        let result = SellStockResultDTO {
            id: 0,
            stock: params.payload.stock,
            shares: params.payload.shares,
            price: 123.45,
        };

        return Ok(result);
    }
}
