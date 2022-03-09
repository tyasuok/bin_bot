// use std::collections::HashMap;
use bin_bot::json_formats::Kline;
// use serde_json::{Result, Value};
// use serde_json;
// use std::io::Result;
//

// time is in milliseconds

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
// #[derive(Serialize, Deserialize, Debug)]
// struct APIResponse {
//     timezone: serde_json::Value,
//     server_time: serde_json::Value,
//     rate_limits: serde_json::Value,
//     exchange_filters: serde_json::Value,
//     symbols: serde_json::Value,
// }
//

// #[derive(Serialize, Deserialize, Debug)]
// struct Tmp {
//     a: Vec<Kline>,
// }
// #[derive(Serialize, Deserialize, Debug)]
// struct Test {
//     Vec<Tmp>,
// }

async fn kline(symbol: &str) -> Vec<Vec<Kline>> {
    let response = reqwest::get(format!("https://api.binance.com/api/v3/klines?symbol={}&interval=1h", {symbol}))
        .await.unwrap()
        .json::<Vec<Vec<Kline>>>()
        // .json::<Vec<Tmp>>()
        .await.unwrap();
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
    let response = kline(&symbol).await;
    // let response = kline(String::from_str(symbol));
    println!("{:#?}", response[0]);

    Ok(())
}
