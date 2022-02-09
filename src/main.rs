// use std::collections::HashMap;
use serde::{Deserialize, Serialize};
// use serde_json::{Result, Value};
// use serde_json;
// use std::io::Result;
//

// time is in milliseconds

#[derive(Serialize, Deserialize, Debug)]
struct Timezone {
    current_zone: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct ServerTime {
    time: u32,
}
#[derive(Serialize, Deserialize, Debug)]
struct RateLimit {
    rateLimitType: String,
    interval: String,
    intervalNum: u32,
    limit: u32,
}
// #[derive(Serialize, Deserialize, Debug)]
// struct RateLimits {
//     tmp_rate: Vec<RateLimit>,
// }
#[derive(Serialize, Deserialize, Debug)]
struct ExchangeFilters {
    filters: Vec<u32>,
}
// Put the Filter inside the filters in Symbol, when u make it work
// #[derive(Serialize, Deserialize, Debug)]
// struct PriceFilter {
//     filter_type: String,
//     min_price: String,
//     max_price: String,
//     tick_size: String,
// }
#[derive(Serialize, Deserialize, Debug)]
struct PriceFilter {
    filterType: String,
    minPrice: String,
    maxPrice: String,
    tickSize: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct PercentPrice {
    filterType: String,
    multiplierUp: String,
    multiplierDown: String,
    avgPriceMins: u32,
}
#[derive(Serialize, Deserialize, Debug)]
struct LotSize {
    filterType: String,
    minQty: String,
    maxQty: String,
    stepSize: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct MinNotional {
    filterType: String,
    minNotional: String,
    applyToMarket: bool,
    avgPriceMins: u32,
}
#[derive(Serialize, Deserialize, Debug)]
struct IcebergParts {
    filterType: String,
    limit: u32,
}
#[derive(Serialize, Deserialize, Debug)]
struct MarketLotSize {
    filterType: String,
    minQty: String,
    maxQty: String,
    stepSize: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct MaxNumOrders {
    filterType: String,
    maxNumOrders: u32,
}
#[derive(Serialize, Deserialize, Debug)]
struct MaxNumAlgoOrders {
    filterType: String,
    maxNumAlgoOrders: u32,
}
#[derive(Serialize, Deserialize, Debug)]
struct MaxPosition {
    filterType: String,
    maxPosition: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Filters {
    PriceFilter(PriceFilter),
    PercentPrice(PercentPrice),
    LotSize(LotSize),
    MinNotional(MinNotional),
    IcebergParts(IcebergParts),
    MarketLotSize(MarketLotSize),
    MaxNumOrders(MaxNumOrders),
    MaxNumAlgoOrders(MaxNumAlgoOrders),
    MaxPosition(MaxPosition),
}
#[derive(Serialize, Deserialize, Debug)]
struct Symbol {
    symbol: String,
    status: String,
    baseAsset: String,
    baseAssetPrecision: u32,
    quoteAsset: String,
    quotePrecision: u32,
    quoteAssetPrecision: u32,
    baseCommissionPrecision: u32,
    quoteCommissionPrecision: u32,
    orderTypes: Vec<String>,
    icebergAllowed: bool,
    ocoAllowed: bool,
    quoteOrderQtyMarketAllowed: bool,
    isSpotTradingAllowed: bool,
    isMarginTradingAllowed: bool,
    filters: Vec<Filters>,
    permissions: Vec<String>,
}
// #[derive(Serialize, Deserialize, Debug)]
// struct Symbols {
//     tmp_symb: String,
// }
// #[derive(Serialize, Deserialize, Debug)]
// struct APIResponse {
//     timezone: Timezone,
//     server_time: ServerTime,
//     rate_limits: RateLimits,
//     exchange_filters: ExchangeFilters,
//     symbols: Symbols,
// }
#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    timezone: String,
    serverTime: u64,
    rateLimits: Vec<RateLimit>, // RateLimits,
    exchangeFilters: Vec<u32>,
    symbols: Vec<Symbol>,
}
// #[derive(Serialize, Deserialize, Debug)]
// struct APIResponse {
//     timezone: serde_json::Value,
//     server_time: serde_json::Value,
//     rate_limits: serde_json::Value,
//     exchange_filters: serde_json::Value,
//     symbols: serde_json::Value,
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://www.binance.com/api/v3/exchangeInfo")
        .await?
        .json::<APIResponse>()
        .await?;
    println!("{:#?}", response.timezone);

    Ok(())
}
