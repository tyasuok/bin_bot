// use bin_bot::json_formats;
// mod json_formats;
use crate::json_formats;
use std::error;

pub async fn kline(symbol: &str) -> Result<Vec<Vec<json_formats::Kline>>, Box<dyn error::Error>> {
    let response = reqwest::get(format!("https://api.binance.com/api/v3/klines?symbol={}&interval=1h", {symbol}))
        .await?
        .json::<Vec<Vec<json_formats::Kline>>>()
        .await?;
    Ok(response)
}

pub async fn exc_info() -> Result<json_formats::APIResponse, Box<dyn error::Error>> {
    let response = reqwest::get("https://www.binance.com/api/v3/exchangeInfo")
        .await?
        .json::<json_formats::APIResponse>()
        .await?;
    Ok(response)
}
