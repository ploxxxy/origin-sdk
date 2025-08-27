use std::error::Error;

use origin_sdk::sdk::OriginSdk;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = OriginSdk::connect("127.0.0.1:3216").await.unwrap();
    let state = client.get_internet_connected_state().await.unwrap();

    println!("Connected: {}", state.connected);

    tokio::signal::ctrl_c().await?;

    Ok(())
}
