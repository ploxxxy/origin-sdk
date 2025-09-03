use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessageEvent {
    #[serde(rename = "@FromId")]
    pub from_id: u64,
    #[serde(rename = "@GroupId")]
    pub group_id: String,
    #[serde(rename = "@Thread")]
    pub thread: String,
    #[serde(rename = "@Message")]
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChatState {
    #[serde(rename = "USER_WRITING_START")]
    UserWritingStart,
    #[serde(rename = "USER_WRITING_END")]
    UserWritingEnd,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatStateUpdateEvent {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@State")]
    pub state: ChatState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendChatMessage {
    #[serde(rename = "@FromId")]
    pub from_id: u64,
    #[serde(rename = "@ToId")]
    pub to_id: u64,
    #[serde(rename = "@Thread")]
    pub thread: String,
    #[serde(rename = "@Message")]
    pub message: String,
    #[serde(rename = "@GroupId")]
    pub group_id: String,
}
