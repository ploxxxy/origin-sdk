use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthCode {
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Challenge {
    #[serde(rename = "@key")]
    pub key: String,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@build")]
    pub build: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeAccepted {
    #[serde(rename = "@response")]
    pub response: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeResponse {
    #[serde(rename = "@response")]
    pub response: String,
    #[serde(rename = "@key")]
    pub key: String,
    #[serde(rename = "@version")]
    pub protocol_version: String,
    #[serde(rename = "ContentId")]
    pub content_id: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "MultiplayerId")]
    pub multiplayer_id: String,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "Version")]
    pub sdk_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAuthCode {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@ClientId")]
    pub client_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    #[serde(rename = "@IsLoggedIn")]
    pub is_logged_in: bool,
    #[serde(rename = "@UserIndex")]
    pub user_index: i32,
    #[serde(rename = "@LoginReasonCode")]
    pub login_reason_code: LoginReasonCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LoginReasonCode {
    #[serde(rename = "UNDEFINED")]
    Undefined,
    #[serde(rename = "USER_INITIATED")]
    UserInitiated,
    #[serde(rename = "ALREADY_ONLINE")]
    AlreadyOnline,
    #[serde(rename = "NETWORK_ERROR")]
    NetworkError,
    #[serde(rename = "INVALID_CREDENTIALS")]
    InvalidCredentials,
    #[serde(rename = "ACCESSTOKEN_REFRESH_ERROR")]
    AccesstokenRefreshError,
}
