// use std::collections::HashMap;
// use bin_bot::json_formats;
use bin_bot::endpoints;
// use serde_json::{Result, Value};
// use serde_json;
// use std::io::Result;

// async fn exc_info() -> json_formats::APIResponse {
// 
// }

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
    //
    
    let response = endpoints::exc_info().await?;
    println!("{:#?}", response);

    // let symbol = "BNBBTC";
    // let response = endpoints::kline(&symbol).await?;
    // // let response = kline(String::from_str(symbol));
    // println!("{:#?}", response[0]);

    Ok(())
}
