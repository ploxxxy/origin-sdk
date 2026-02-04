use origin_sdk::{
    protocol::game::GetAllGameInfo,
    sdk::{ClientConfig, OriginSdk, ORIGIN_SDK_PORT},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This configuration defines the game to the Origin SDK server
    let config = ClientConfig {
        content_id: "Origin.OFR.50.0004455".to_string(),
        language: "".to_string(),
        multiplayer_id: "".to_string(),
        title: "".to_string(),
        version_override: None,
    };

    // Connect to the Origin SDK server with the given configuration and the default port
    let (client, _) = OriginSdk::connect(config, ORIGIN_SDK_PORT).await?;

    let game_info = client.request(GetAllGameInfo {}).await?;
    println!("{:#?}", game_info);
    // GetAllGameInfoResponse {
    //     up_to_date: true,
    //     languages: "de_DE,en_US,es_ES,es_MX,fr_FR,it_IT,ja_JP,ko_KR,pl_PL,pt_BR,ru_RU,zh_CN,zh_TW",
    //     free_trial: false,
    //     full_game_purchased: true,
    //     full_game_released: true,
    //     full_game_release_date: "0000-00-00T00:00:48",
    //     expiration: "0000-00-00T00:00:08",
    //     system_time: "2025-09-03T12:15:54",
    //     has_expiration: false,
    //     installed_version: "",
    //     installed_language: "en_US",
    //     available_version: "0.0.0.0",
    //     display_name: "Knockout City™",
    //     max_group_size: 16,
    //     entitlement_source: "EPIC",
    // }

    Ok(())
}
