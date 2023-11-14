use serde::{Deserialize, Serialize};
use std;

#[derive(Deserialize, Serialize, Debug)]
pub struct PurchaseStockParametersDTO {
    pub stock: String,
    pub shares: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PurchaseStockResultDTO {
    pub stock: String,
    pub shares: usize,
    pub price: f32,
}

//

pub async fn use_case(
    params: PurchaseStockParametersDTO,
) -> Result<PurchaseStockResultDTO, Box<dyn std::error::Error>> {
    let result = PurchaseStockResultDTO {
        stock: params.stock,
        shares: params.shares,
        price: 123.45,
    };

    return Ok(result);
}
