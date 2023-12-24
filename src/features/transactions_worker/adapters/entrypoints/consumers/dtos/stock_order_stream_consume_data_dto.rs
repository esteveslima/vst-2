use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

lazy_static! {
    static ref ORDER_OPERATION_ENUM: Regex = Regex::new(r"^(PURCHASE|SELL)$").unwrap();
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct StockOrderStreamConsumeDataDTO {
    #[validate(regex(path = "ORDER_OPERATION_ENUM"))]
    pub operation: String,

    #[validate(length(min = 1))]
    pub stock: String,

    #[validate(range(min = 1))]
    pub shares: usize,
}
