// use bin_bot::json_formats;
// mod json_formats;
use crate::json_formats;
use std::error;
use std::env;
use sha2::Sha256;
use hmac::{Hmac, Mac};

pub async fn kline(symbol: &str) -> Result<Vec<Vec<json_formats::Kline>>, Box<dyn error::Error>> {
    // let response = reqwest::get(format!("https://api.binance.com/api/v3/klines?symbol={}&interval=1h", {symbol}))
    let response = reqwest::get(format!("https://testnet.binance.vision/api/v3/klines?symbol={}&interval=1h", {symbol}))
        .await?
        .json::<Vec<Vec<json_formats::Kline>>>()
        .await?;
    Ok(response)
}

pub async fn exc_info() -> Result<json_formats::APIResponse, Box<dyn error::Error>> {
    // let response = reqwest::get("https://www.binance.com/api/v3/exchangeInfo")
    let response = reqwest::get("https://testnet.binance.vision/api/v3/exchangeInfo")
        .await?
        .json::<json_formats::APIResponse>()
        .await?;
    Ok(response)
}

// pub async fn sub_assets() -> Result<json_formats::APIResponse, Box<dyn error::Error>> {
pub async fn sub_assets() -> Result<std::string::String, Box<dyn error::Error>> {
    // let response = reqwest::get("https://www.binance.com/api/v3/exchangeInfo")
    let response = reqwest::get("https://testnet.binance.vision/sapi/v3/sub-account/assets")
        .await?
        .text()
        // .json::<json_formats::APIResponse>()
        .await?;
    Ok(response)
}

// pub async fn test() -> Result<std::string::String, Box<dyn error::Error>> {
//     type ko = Hmac<Sha256>;
//     let mut t = ko::new_from_slice(b"my secret and secure key").await?;
//     t.update(b"input message");
//     let result = t.finalize();
//     Ok(t.into_bytes())
// 
// }
