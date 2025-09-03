use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ContentState {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "READY_TO_DOWNLOAD")]
    ReadyToDownload,
    #[serde(rename = "SERVER_QUEUED")]
    ServerQueued,
    #[serde(rename = "DOWNLOADING")]
    Downloading,
    #[serde(rename = "DOWNLOAD_PAUSED")]
    DownloadPaused,
    #[serde(rename = "READY_TO_ACTIVATE")]
    ReadyToActivate,
    #[serde(rename = "READY_TO_DECRYPT")]
    ReadyToDecrypt,
    #[serde(rename = "DECRYPTING")]
    Decrypting,
    #[serde(rename = "READY_TO_UNPACK")]
    ReadyToUnpack,
    #[serde(rename = "UNPACKING")]
    Unpacking,
    #[serde(rename = "READY_TO_INSTALL")]
    ReadyToInstall,
    #[serde(rename = "INSTALLING")]
    Installing,
    #[serde(rename = "READY_TO_PLAY")]
    ReadyToPlay,
    #[serde(rename = "READY_TO_USE")]
    ReadyToUse,
    #[serde(rename = "INSTALLED")]
    Installed,
    #[serde(rename = "WAITING_TO_DOWNLOAD")]
    WaitingToDownload,
    #[serde(rename = "WAITING_TO_DECRYPT")]
    WaitingToDecrypt,
    #[serde(rename = "DOWNLOAD_EXPIRED")]
    DownloadExpired,
    #[serde(rename = "DECRYPT_EXPIRED")]
    DecryptExpired,
    #[serde(rename = "INVALID_CONTENT")]
    InvalidContent,
    #[serde(rename = "PLAYING")]
    Playing,
    #[serde(rename = "FINALIZING_DOWNLOAD")]
    FinalizingDownload,
    #[serde(rename = "PREPARING_DOWNLOAD")]
    PreparingDownload,
    #[serde(rename = "UPDATING")]
    Updating,
    #[serde(rename = "VERIFYING")]
    Verifying,
    #[serde(rename = "READY_TO_UPDATE")]
    ReadyToUpdate,
    #[serde(rename = "UPDATE_PAUSED")]
    UpdatePaused,
    #[serde(rename = "DOWNLOAD_QUEUED")]
    DownloadQueued,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreContentUpdated {
    #[serde(rename = "Game", default)]
    pub games: Vec<Game>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dlc {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "@Installed")]
    pub installed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "@contentID")]
    pub content_id: String,
    #[serde(rename = "@progressValue")]
    pub progress_value: f32,
    #[serde(rename = "@state")]
    pub state: ContentState,
    #[serde(rename = "@installedVersion")]
    pub installed_version: String,
    #[serde(rename = "@availableVersion")]
    pub available_version: String,
    #[serde(rename = "@displayName")]
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GameInfo {
    #[serde(rename = "UPTODATE")]
    Uptodate,
    #[serde(rename = "LANGUAGES")]
    Languages,
    #[serde(rename = "FREETRIAL")]
    Freetrial,
    #[serde(rename = "EXPIRATION")]
    Expiration,
    #[serde(rename = "EXPIRATION_DURATION")]
    ExpirationDuration,
    #[serde(rename = "INSTALLED_VERSION")]
    InstalledVersion,
    #[serde(rename = "INSTALLED_LANGUAGE")]
    InstalledLanguage,
    #[serde(rename = "AVAILABLE_VERSION")]
    AvailableVersion,
    #[serde(rename = "DISPLAY_NAME")]
    DisplayName,
    #[serde(rename = "FULLGAME_PURCHASED")]
    FullgamePurchased,
    #[serde(rename = "FULLGAME_IS_RELEASED")]
    FullgameIsReleased,
    #[serde(rename = "FULLGAME_RELEASE_DATE")]
    FullgameReleaseDate,
    #[serde(rename = "MAX_GROUP_SIZE")]
    MaxGroupSize,
    #[serde(rename = "ENTITLEMENT_SOURCE")]
    EntitlementSource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameMessageEvent {
    #[serde(rename = "@GameId")]
    pub game_id: String,
    #[serde(rename = "@Message")]
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllGameInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllGameInfoResponse {
    #[serde(rename = "@UpToDate")]
    pub up_to_date: bool,
    #[serde(rename = "@Languages")]
    pub languages: String,
    #[serde(rename = "@FreeTrial")]
    pub free_trial: bool,
    #[serde(rename = "@FullGamePurchased")]
    pub full_game_purchased: bool,
    #[serde(rename = "@FullGameReleased")]
    pub full_game_released: bool,
    #[serde(rename = "@FullGameReleaseDate")]
    pub full_game_release_date: String,
    #[serde(rename = "@Expiration")]
    pub expiration: String,
    #[serde(rename = "@SystemTime")]
    pub system_time: String,
    #[serde(rename = "@HasExpiration")]
    pub has_expiration: bool,
    #[serde(rename = "@InstalledVersion")]
    pub installed_version: String,
    #[serde(rename = "@InstalledLanguage")]
    pub installed_language: String,
    #[serde(rename = "@AvailableVersion")]
    pub available_version: String,
    #[serde(rename = "@DisplayName")]
    pub display_name: String,
    #[serde(rename = "@MaxGroupSize")]
    pub max_group_size: i32,
    #[serde(rename = "@EntitlementSource")]
    pub entitlement_source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryContent {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@MultiplayerId")]
    pub multiplayer_id: String,
    #[serde(rename = "@Content")]
    pub content_type: i32,
    #[serde(rename = "MasterTitleId", default)]
    pub game_id: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryContentResponse {
    #[serde(rename = "Game", default)]
    pub content: Vec<Game>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestartGame {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@Options")]
    pub options: RestartOptions,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RestartOptions {
    #[serde(rename = "NORMAL")]
    Normal,
    #[serde(rename = "FORCE_UPDATE_FOR_GAME")]
    ForceUpdateForGame,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendGameMessage {
    #[serde(rename = "@GameId")]
    pub game_id: String,
    #[serde(rename = "@Message")]
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartGame {
    #[serde(rename = "@GameId")]
    pub game_id: String,
    #[serde(rename = "@MultiplayerId")]
    pub multiplayer_id: String,
    #[serde(rename = "@CommandLine")]
    pub command_line: String,
}
