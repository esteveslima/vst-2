use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std;

use crate::features::stocks_api::application::interfaces::use_case::UseCase;

#[derive(Deserialize, Serialize, Debug)]
pub struct GetStocksSummaryParametersDTO {
    pub user_id: usize,
}

//

#[derive(Deserialize, Serialize, Debug)]
pub struct GetStocksSummaryResultDTO {
    pub stock_name: String,
    pub total_shares: usize,
    pub total_value: f32,
    pub variation_purchase: f32,
    pub date_purchase: String,
    pub day_min: f32,
    pub day_avg: f32,
    pub day_max: f32,
}

// // //

pub struct GetStocksSummaryUseCase {}

#[async_trait]
impl UseCase<GetStocksSummaryParametersDTO, GetStocksSummaryResultDTO> for GetStocksSummaryUseCase {
    async fn execute(
        _params: GetStocksSummaryParametersDTO,
    ) -> Result<GetStocksSummaryResultDTO, Box<dyn std::error::Error>> {
        let result = GetStocksSummaryResultDTO {
            stock_name: "stock".to_string(),
            total_shares: 1,
            total_value: 123.45,
            date_purchase: Utc::now().to_string(),
            variation_purchase: 0.0,
            day_max: 0.0,
            day_avg: 0.0,
            day_min: 0.0,
        };

        return Ok(result);
    }
}
