use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum StockOrderTransactionStatus {
    SUCCESS,
    FAIL,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum StockOrderTransactionOperation {
    PURCHASE,
    SELL,
}
impl FromStr for StockOrderTransactionOperation {
    type Err = ();
    fn from_str(input: &str) -> Result<StockOrderTransactionOperation, Self::Err> {
        match input {
            "PURCHASE" => Ok(StockOrderTransactionOperation::PURCHASE),
            "SELL" => Ok(StockOrderTransactionOperation::SELL),
            _ => Err(()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StockOrderTransaction {
    pub user_id: String,
    pub status: StockOrderTransactionStatus,
    pub operation: StockOrderTransactionOperation,
    pub stock: String,
    pub shares: usize,
    pub price: f32,
}
