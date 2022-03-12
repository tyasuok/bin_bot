use bin_bot::endpoints;
use bin_bot::websockets;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    websockets::test().await?;

    Ok(())
}
