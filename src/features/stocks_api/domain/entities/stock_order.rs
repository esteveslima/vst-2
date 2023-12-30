use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct StockOrder {
    pub id: String,
    pub user_id: String,
    pub operation: StockOrderOperation,
    pub date: DateTime<Utc>,
    pub stock: String,
    pub shares: usize,
}
