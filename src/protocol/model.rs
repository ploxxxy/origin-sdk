use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Challenge {
    #[serde(rename = "@build")]
    pub build: String,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@key")]
    pub key: String,
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
pub struct ChallengeAccepted {
    #[serde(rename = "@response")]
    pub response: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInternetConnectedState {
    // #[serde(rename = "@version")]
    // pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InternetConnectedState {
    #[serde(rename = "@connected")]
    pub connected: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryFriends {
    #[serde(rename = "@UserId")]
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PresenceVisibilityEvent {
    #[serde(rename = "@Visible")]
    pub visible: bool,
}
