use std::error::Error;

use origin_sdk::tcp::TcpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = TcpClient::connect("127.0.0.1:3216").await?;
    // let state = client.get_internet_connected_state().await?;

    // println!("Connected: {}", state.connected);

    tokio::signal::ctrl_c().await?;

    Ok(())
}
