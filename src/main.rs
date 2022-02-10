// use std::collections::HashMap;
use serde::{Deserialize, Serialize};
// use serde_json::{Result, Value};
// use serde_json;
// use std::io::Result;
//

// time is in milliseconds

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Timezone {
    current_zone: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerTime {
    time: u32,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RateLimit {
    rate_limit_type: String,
    interval: String,
    interval_num: u32,
    limit: u32,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ExchangeFilters {
    filters: Vec<u32>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PriceFilter {
    filter_type: String,
    min_price: String,
    max_price: String,
    tick_size: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PercentPrice {
    filter_type: String,
    multiplier_up: String,
    multiplier_down: String,
    avg_price_mins: u32,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LotSize {
    filter_type: String,
    min_qty: String,
    max_qty: String,
    step_size: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MinNotional {
    filter_type: String,
    min_notional: String,
    apply_to_market: bool,
    avg_price_mins: u32,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct IcebergParts {
    filter_type: String,
    limit: u32,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MarketLotSize {
    filter_type: String,
    min_qty: String,
    max_qty: String,
    step_size: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MaxNumOrders {
    filter_type: String,
    max_num_orders: u32,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MaxNumAlgoOrders {
    filter_type: String,
    max_num_algo_orders: u32,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MaxPosition {
    filter_type: String,
    max_position: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
struct Symbol {
    symbol: String,
    status: String,
    base_asset: String,
    base_asset_precision: u32,
    quote_asset: String,
    quote_precision: u32,
    quote_asset_precision: u32,
    base_commission_precision: u32,
    quote_commission_precision: u32,
    order_types: Vec<String>,
    iceberg_allowed: bool,
    oco_allowed: bool,
    quote_order_qty_market_allowed: bool,
    is_spot_trading_allowed: bool,
    is_margin_trading_allowed: bool,
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
#[serde(rename_all = "camelCase")]
struct APIResponse {
    timezone: String,
    server_time: u64,
    rate_limits: Vec<RateLimit>, // RateLimits,
    exchange_filters: Vec<u32>,
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
//
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Kline {
    Why(String),
    Yes(u64),
}

// #[derive(Serialize, Deserialize, Debug)]
// struct Tmp {
//     Vec<Kline>,
// }
// #[derive(Serialize, Deserialize, Debug)]
// struct Test {
//     Vec<Tmp>,
// }

async fn kline(symbol) -> Vec<Vec<Kline>> {
    let response = reqwest::get(format!("https://api.binance.com/api/v3/klines?symbol={}&interval=1h", {symbol}))
        .await?
        // .json::<Vec<Vec<Kline>>>()
        .json::Test()
        .await?;
    response
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // post request example
    // let client = reqwest::Client::new();
    // let response = client.post("https://www.binance.com/api/v3/exchangeInfo")
    //     .send()
    //     .await?
    //     .json::<APIResponse>()
    //     .await?;
    //
    // let response = reqwest::get("https://www.binance.com/api/v3/exchangeInfo")
    //     .await?
    //     .json::<APIResponse>()
    //     .await?;
    // println!("{:#?}", response.symbols);

    let symbol = "BNBBTC";
    // let response = reqwest::get(format!("https://api.binance.com/api/v3/klines?symbol={}&interval=1h", {symbol}))
    //     .await?
    //     .json::<Vec<Vec<Kline>>>()
    //     .await?;
    let response = kline(&symbol);
    println!("{:#?}", response);

    Ok(())
}
