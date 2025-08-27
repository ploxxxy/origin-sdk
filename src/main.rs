use std::error::Error;

use origin_sdk::sdk::OriginSdk;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (client, mut event_rx) = OriginSdk::connect("127.0.0.1:3216").await.unwrap();

    let _event_handle = tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            println!("Received event from channel");
            println!("{:#?}", event);
        }
    });

    let state = client.get_internet_connected_state().await.unwrap();

    println!("Connected: {}", state.connected);

    tokio::signal::ctrl_c().await?;

    Ok(())
}
