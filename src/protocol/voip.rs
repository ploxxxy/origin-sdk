use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EnableVoip {
    #[serde(rename = "@Enable")]
    pub enable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EnumMuteState {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "UNMUTED")]
    Unmuted,
    #[serde(rename = "MUTED_LOCALLY")]
    MutedLocally,
    #[serde(rename = "MUTED_REMOTELY")]
    MutedRemotely,
    #[serde(rename = "MUTED_LOCALLY_AND_REMOTELY")]
    MutedLocallyAndRemotely,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVoipStatus;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVoipStatusResponse {
    #[serde(rename = "@Available")]
    pub available: bool,
    #[serde(rename = "@Active")]
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MuteState {
    #[serde(rename = "@State")]
    pub state: EnumMuteState,
    #[serde(rename = "@UserId")]
    pub user_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MuteUser {
    #[serde(rename = "@bMute")]
    pub b_mute: bool,
    #[serde(rename = "@GroupId")]
    pub group_id: String,
    #[serde(rename = "@UserId")]
    pub user_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryMuteState {
    #[serde(rename = "@GroupId")]
    pub group_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryMuteStateResponse {
    #[serde(rename = "MuteState", default)]
    pub mute_state_array: Vec<MuteState>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VoipStatus {
    #[serde(rename = "UNKNOWN")]
    Unknown = -1,
    #[serde(rename = "CHANNEL_DISCONNECTED")]
    ChannelDisconnected,
    #[serde(rename = "CHANNEL_CONNECTING")]
    ChannelConnecting,
    #[serde(rename = "CHANNEL_CONNECTED")]
    ChannelConnected,
    #[serde(rename = "USER_TALKING_START")]
    UserTalkingStart,
    #[serde(rename = "USER_TALKING_END")]
    UserTalkingEnd,
    #[serde(rename = "USER_MUTED_LOCALLY")]
    UserMutedLocally,
    #[serde(rename = "USER_MUTED_REMOTELY")]
    UserMutedRemotely,
    #[serde(rename = "USER_UNMUTED_LOCALLY")]
    UserUnmutedLocally,
    #[serde(rename = "USER_UNMUTED_REMOTELY")]
    UserUnmutedRemotely,
    #[serde(rename = "USER_JOINED")]
    UserJoined,
    #[serde(rename = "USER_LEFT")]
    UserLeft,
    #[serde(rename = "UNAVAILABLE")]
    Unavailable,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoipStatusEvent {
    #[serde(rename = "@Status")]
    pub status: VoipStatus,
    #[serde(rename = "@UserId")]
    pub user_id: u64,
}
