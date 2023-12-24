use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct PurchaseStockRestRequestBodyDTO {
    #[validate(length(min = 3))]
    pub stock: String,

    #[validate(range(min = 1, max = 100))]
    pub shares: usize,
}

//

#[derive(Deserialize, Serialize, Debug)]
pub struct PurchaseStockRestResponseDTO {
    pub id: String,
    pub stock: String,
    pub shares: usize,
    pub price: f32,
}
