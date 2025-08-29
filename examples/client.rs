use origin_sdk::{
    protocol::model::{GetConfig, GetInternetConnectedState, GetProfile},
    sdk::OriginSdk,
};
use std::error::Error;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    fmt()
        .with_env_filter(EnvFilter::new("trace"))
        .try_init()
        .unwrap();

    let (client, mut event_rx) = OriginSdk::connect("127.0.0.1:3216").await.unwrap();

    let _event_handle = tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            info!("Received event from channel");
            info!("{:#?}", event);
        }
    });

    let state = client
        .send_request(GetInternetConnectedState {})
        .await
        .unwrap();
    info!("Connected to the internet?: {}", state.connected);

    let profile = client.send_request(GetProfile { index: 0 }).await.unwrap();
    info!("Profile: {:#?}", profile);

    let config = client.send_request(GetConfig {}).await.unwrap();
    info!("Config: {:#?}", config);

    tokio::signal::ctrl_c().await?;

    Ok(())
}
