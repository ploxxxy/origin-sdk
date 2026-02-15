use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SetSteamLocale {
    #[serde(rename = "@Language")]
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SteamAchievementErrorTelemetry {
    #[serde(rename = "@validStats")]
    pub valid_stats: bool,
    #[serde(rename = "@setStat")]
    pub set_stat: bool,
    #[serde(rename = "@getStat")]
    pub get_stat: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SteamAchievementEvent {
    #[serde(rename = "@AchievementId")]
    pub achievement_id: String,
    #[serde(rename = "@Points")]
    pub points: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SteamActivateOverlayToStoreEvent {
    #[serde(rename = "@AppId")]
    pub app_id: String,
    #[serde(rename = "@OfferId")]
    pub offer_id: String,
    #[serde(rename = "@IsBaseGame")]
    pub is_base_game: bool,
    #[serde(rename = "@Flag")]
    pub flag: SteamOverlayToStoreFlag,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SteamOverlayToStoreFlag {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "ADDTOCART")]
    Addtocart,
    #[serde(rename = "ADDTOCARTANDSHOW")]
    Addtocartandshow,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SteamPurchaseConfirmation {
    #[serde(rename = "@AppId")]
    pub app_id: i32,
    #[serde(rename = "@OrderId")]
    pub order_id: u64,
    #[serde(rename = "@Authorized")]
    pub authorized: bool,
}
