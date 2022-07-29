use bin_bot::endpoints;
use bin_bot::websockets;
use sha2::Sha256;
use hmac::{Hmac, Mac};
// use hex_literal::hex;
use std::error;
use std::env;
use hex::encode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // websockets::test("btcusdt@bookTicker").await?;
    // println!("{:?}", endpoints::kline("BTCUSDT").await?);
    // println!("{:?}", endpoints::sub_assets().await?);
    // println!("{:?}", endpoints::test().await?);
    //
    // type ko = Hmac<Sha256>;
    // let mut t = ko::new_from_slice(b"my secret and secure key").await?;
    // t.update(b"input message");
    // let result = t.finalize();
    // let key = match env::var("BINANCE_SECRET_API") {
    //     Ok(v) => v,
    //     Err(e) => panic!("Env var BINANCE_API is not set")
    // };
    // type ko = Hmac<Sha256>;
    // let mut t = ko::new_from_slice(&key.into_bytes()).expect("huh");
    // // t.update(b"input message");
    // let key = hex::encode(t.finalize().into_bytes());
    // println!("{}", key);
    // println!("https://testnet.binance.vision/sapi/v1/capital/config/getall/?signature{:?}", key)
    // let response = reqwest::get(format!(,
    //                             key))
    //     .await?
    //     .text()
    //     // .json::<json_formats::APIResponse>()
    //     .await?;
    // println!("{:?}", response);
    sub_assets().await?;

    Ok(())
}


pub async fn sub_assets() -> Result<std::string::String, Box<dyn error::Error>> {
    let env_name = "TESTNET_SECRET_API";
    let key = match env::var(env_name) {
        Ok(v) => v,
        Err(e) => panic!("Env var {} is not set", env_name)
    };
    type ko = Hmac<Sha256>;
    let mut t = ko::new_from_slice(&key.into_bytes()).expect("huh");
    // t.update(b"input message");
    let key = hex::encode(t.finalize().into_bytes());
    println!("{}", key);
    let response = reqwest::get(format!("https://testnet.binance.vision/sapi/v1/capital/config/getall/?signature={}", key))
        .await?
        .text()
        // .json::<json_formats::APIResponse>()
        .await?;
    println!("{:?}", response);
    // Ok(String::from("yo"))
    Ok(response)
}
//
// fn main() {
//     // let a = sub_assets().await?;
//     // println!("{:?}", a);
// 
//     // let key = env!("BINANCE_API");
//     // println!("{}", key);
//     // type ko = Hmac<Sha256>;
//     // let mut t = ko::new_from_slice(b"my secret and secure key").expect("huh");
//     // t.update(b"input message");
//     // let result = t.finalize();
//     // println!("{:?}", result.into_bytes());
//     // let expected = hex!("
//     //     97d2a569059bbcd8ead4444ff99071f4
//     //     c01d005bcefe0d3567e1be628e5fdcd9
//     // ");
//     // println!("{:?}", expected);
// }
