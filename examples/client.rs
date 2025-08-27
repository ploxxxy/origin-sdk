use origin_sdk::sdk::OriginSdk;
use std::error::Error;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    fmt().with_env_filter(EnvFilter::new("trace")).try_init().unwrap();

    let (client, mut event_rx) = OriginSdk::connect("127.0.0.1:3216").await.unwrap();

    let _event_handle = tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            info!("Received event from channel");
            info!("{:#?}", event);
        }
    });

    let state = client.get_internet_connected_state().await.unwrap();

    info!("Connected?: {}", state.connected);

    tokio::signal::ctrl_c().await?;

    Ok(())
}
