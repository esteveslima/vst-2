use serde::{Deserialize, Serialize};
// use validator::Validate;

// #[derive(Deserialize, Serialize, Debug, Validate)]
// pub struct GetStocksSummaryRestRequestDTO {
// }

//

#[derive(Deserialize, Serialize, Debug)]
pub struct GetStocksSummaryRestResponseDTO {
    pub stock_name: String,
    pub total_shares: usize,
    pub total_value: f32,
    pub variation_purchase: f32,
    pub date_purchase: String,
    pub day_min: f32,
    pub day_avg: f32,
    pub day_max: f32,
}
