use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcastEvent {
    #[serde(rename = "@State")]
    pub state: BroadcastState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcastStart;

#[derive(Debug, Serialize, Deserialize)]
pub enum BroadcastState {
    #[serde(rename = "DIALOG_OPEN")]
    DialogOpen,
    #[serde(rename = "DIALOG_CLOSED")]
    DialogClosed,
    #[serde(rename = "ACCOUNTLINKDIALOG_OPEN")]
    AccountlinkdialogOpen,
    #[serde(rename = "ACCOUNT_DISCONNECTED")]
    AccountDisconnected,
    #[serde(rename = "STARTED")]
    Started,
    #[serde(rename = "STOPPED")]
    Stopped,
    #[serde(rename = "BLOCKED")]
    Blocked,
    #[serde(rename = "START_PENDING")]
    StartPending,
    #[serde(rename = "ERROR")]
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcastStatus {
    #[serde(rename = "@status")]
    pub status: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcastStop;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBroadcastStatus;
