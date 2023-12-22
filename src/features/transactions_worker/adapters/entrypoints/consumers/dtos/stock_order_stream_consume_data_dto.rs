use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct StockOrderStreamConsumeDataDTO {
    #[validate(length(min = 1))]
    pub stock: String,

    #[validate(range(min = 1))]
    pub shares: usize,
}
