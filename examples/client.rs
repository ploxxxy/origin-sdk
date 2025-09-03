use origin_sdk::{
    protocol::{
        profile::GetProfile,
        system::{GetConfig, GetInternetConnectedState},
    },
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

    // Connect to the Origin SDK server at the given address
    // Returns a client handle for sending requests and a channel for receiving events
    let (client, mut event_rx) = OriginSdk::connect("127.0.0.1:3216").await.unwrap();

    // Spawn a background task that continuously listens for server events
    // and logs them. Keeps the main thread free to send requests
    let _event_handle = tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            info!("Received event from channel");
            info!("{:#?}", event);
        }
    });

    // Query whether the SDK reports an active internet connection
    let state = client
        .send_request(GetInternetConnectedState {})
        .await
        .unwrap();
    info!("Connected to the internet?: {}", state.connected);

    // Fetch the first user profile (index 0 = current user)
    let profile = client.send_request(GetProfile { index: 0 }).await.unwrap();
    info!("Profile: {:#?}", profile);

    // Fetch Service -> Facility configuration.
    // This would be used to determine which facilities to send requests to,
    // but EA Desktop uses "EbisuSDK" for all its services
    let config = client.send_request(GetConfig {}).await.unwrap();
    info!("Config: {:#?}", config);

    // Block until Ctrl+C is pressed. This keeps the process alive
    // so background event handling continues until user termination
    tokio::signal::ctrl_c().await?;

    Ok(())
}
