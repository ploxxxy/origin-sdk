use serde::{Deserialize, Serialize};

use crate::protocol::friends::Friend;

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentUserPresenceEvent {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@Presence")]
    pub presence: Presence,
    #[serde(rename = "@Title")]
    pub title: String,
    #[serde(rename = "@TitleId")]
    pub title_id: String,
    #[serde(rename = "@MultiplayerId")]
    pub multiplayer_id: String,
    #[serde(rename = "@RichPresence")]
    pub rich_presence: String,
    #[serde(rename = "@GamePresence")]
    pub game_presence: String,
    #[serde(rename = "@SessionId")]
    pub session_id: String,
    #[serde(rename = "@Group")]
    pub group: String,
    #[serde(rename = "@GroupId")]
    pub group_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPresence {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPresenceResponse {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@Presence")]
    pub presence: Presence,
    #[serde(rename = "@Title")]
    pub title: String,
    #[serde(rename = "@TitleId")]
    pub title_id: String,
    #[serde(rename = "@MultiplayerId")]
    pub multiplayer_id: String,
    #[serde(rename = "@RichPresence")]
    pub rich_presence: String,
    #[serde(rename = "@GamePresence")]
    pub game_presence: String,
    #[serde(rename = "@SessionId")]
    pub session_id: String,
    #[serde(rename = "@Group")]
    pub group: String,
    #[serde(rename = "@GroupId")]
    pub group_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPresenceVisibility {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPresenceVisibilityResponse {
    #[serde(rename = "@Visible")]
    pub visible: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoOnline;

#[derive(Debug, Serialize, Deserialize)]
pub struct OnlineStatusEvent {
    #[serde(rename = "@isOnline")]
    pub is_online: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Presence {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "OFFLINE")]
    Offline,
    #[serde(rename = "ONLINE")]
    Online,
    #[serde(rename = "INGAME")]
    Ingame,
    #[serde(rename = "BUSY")]
    Busy,
    #[serde(rename = "IDLE")]
    Idle,
    #[serde(rename = "JOINABLE")]
    Joinable,
    #[serde(rename = "JOINABLE_INVITE_ONLY")]
    JoinableInviteOnly,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PresenceEvent {
    #[serde(rename = "@userid")]
    pub userid: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PresenceVisibilityEvent {
    #[serde(rename = "@Visible")]
    pub visible: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryPresence {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "Users", default)]
    pub users: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryPresenceResponse {
    #[serde(rename = "Friend", default)]
    pub friends: Vec<Friend>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetPresence {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@Presence")]
    pub presence: Presence,
    #[serde(rename = "@RichPresence")]
    pub rich_presence: String,
    #[serde(rename = "@GamePresence")]
    pub game_presence: String,
    #[serde(rename = "@SessionId")]
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetPresenceVisibility {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@Visible")]
    pub visible: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribePresence {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "Users")]
    pub users: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnsubscribePresence {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "Users")]
    pub users: Vec<u64>,
}
