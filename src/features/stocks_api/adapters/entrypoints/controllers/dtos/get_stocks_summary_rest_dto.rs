use serde::{Deserialize, Serialize};
// use validator::Validate;

// #[derive(Deserialize, Serialize, Debug, Validate)]
// pub struct GetStocksSummaryRestRequestDTO {
// }

//

#[derive(Deserialize, Serialize, Debug)]
pub struct GetStocksSummaryRestResponseItemDTO {
    pub stock: String,
    pub total_shares: i64,
    pub total_purchase_price: f64,
    pub variation_purchase: f32,
    pub date_first_purchase: String,
    pub day_min: f32,
    pub day_avg: f32,
    pub day_max: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetStocksSummaryRestResponseDTO {
    pub stocks: Vec<GetStocksSummaryRestResponseItemDTO>,
}
