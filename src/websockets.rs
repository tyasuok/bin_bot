use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use native_tls::TlsConnector;
use std::io::{Read, Write};
use futures_util::{future, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std;

pub async fn test(a: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (ws_stream, _) = connect_async(format!("wss://stream.binance.com:9443/ws/{}", a)).await?;
    let (write, mut read) = ws_stream.split();
    loop {
        let message = read.next().await.unwrap()?;
        match message {
            Message::Text(msg) => {println!("{}", msg)}
            // Message::Ping(_) | Message::Pong(_) | Message::Binary(_) | Message::Frame(_) => {}
            Message::Ping(_) => {
                println!("Ping");
                // write
            }
            Message::Pong(_) => {println!("Pong")}
            Message::Binary(_) => {println!("Binary")}
            Message::Frame(_) => {println!("Frame")}
            Message::Close(e) => {println!("{:?}", e)}
        }
        // std::thread::sleep(std::time::Duration::from_secs(5));
        // tokio::time::sleep(core::time::Duration::from_secs(5));
    }
    Ok(())
}
