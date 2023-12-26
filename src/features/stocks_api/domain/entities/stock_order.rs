use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum StockOrderOperation {
    PURCHASE,
    SELL,
}
impl FromStr for StockOrderOperation {
    type Err = ();
    fn from_str(input: &str) -> Result<StockOrderOperation, Self::Err> {
        match input {
            "PURCHASE" => Ok(StockOrderOperation::PURCHASE),
            "SELL" => Ok(StockOrderOperation::SELL),
            _ => Err(()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StockOrder {
    pub user_id: String,
    pub operation: StockOrderOperation,
    pub stock: String,
    pub shares: usize,
}
