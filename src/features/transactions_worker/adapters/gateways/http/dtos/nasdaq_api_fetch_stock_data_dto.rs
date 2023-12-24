use serde::{Deserialize, Serialize};

// Only partial data from API response
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(non_snake_case)]
pub struct NasdaqAPIFetchStockDataResultDataPrimaryDataDTO {
    pub lastSalePrice: String,
    pub netChange: String,
    pub percentageChange: String,
    pub deltaIndicator: String,
    pub lastTradeTimestamp: String,
    pub isRealTime: String,
    pub bidPrice: String,
    pub askPrice: String,
    pub bidSize: String,
    pub askSize: String,
    pub volume: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NasdaqAPIFetchStockDataResultDataKeyStatsStatDTO {
    pub label: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(non_snake_case)]
pub struct NasdaqAPIFetchStockDataResultDataKeyStatsDTO {
    pub fiftyTwoWeekHighLow: NasdaqAPIFetchStockDataResultDataKeyStatsStatDTO,
    pub dayrange: NasdaqAPIFetchStockDataResultDataKeyStatsStatDTO,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(non_snake_case)]
pub struct NasdaqAPIFetchStockDataResultDataDTO {
    pub symbol: String,
    pub companyName: String,
    pub primaryData: NasdaqAPIFetchStockDataResultDataPrimaryDataDTO,
    pub keyStats: NasdaqAPIFetchStockDataResultDataKeyStatsDTO,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NasdaqAPIFetchStockDataResultDTO {
    pub data: NasdaqAPIFetchStockDataResultDataDTO,
}
