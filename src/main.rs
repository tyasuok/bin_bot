use bin_bot::endpoints;
use bin_bot::websockets;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use hex_literal::hex;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // websockets::test("btcusdt@bookTicker").await?;
//     // println!("{:?}", endpoints::kline("BTCUSDT").await?);
//     // println!("{:?}", endpoints::sub_assets().await?);
//     // println!("{:?}", endpoints::test().await?);
//     //
//     type ko = Hmac<Sha256>;
//     let mut t = ko::new_from_slice(b"my secret and secure key").await?;
//     t.update(b"input message");
//     let result = t.finalize();
// 
//     Ok(())
// }
//
fn main() {
    type ko = Hmac<Sha256>;
    let mut t = ko::new_from_slice(b"my secret and secure key").expect("huh");
    t.update(b"input message");
    let result = t.finalize();
    println!("{:?}", result.into_bytes());
    let expected = hex!("
        97d2a569059bbcd8ead4444ff99071f4
        c01d005bcefe0d3567e1be628e5fdcd9
    ");
    println!("{:?}", expected);
}
