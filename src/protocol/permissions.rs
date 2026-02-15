use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckPermission {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@PermissionId")]
    pub permission_id: Permission,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckPermissionResponse {
    #[serde(rename = "@Access")]
    pub access: Access,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Access {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "DENIED")]
    Denied,
    #[serde(rename = "GRANTED")]
    Granted,
    #[serde(rename = "FRIENDS_ONLY")]
    FriendsOnly,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Permission {
    #[serde(rename = "MULTIPLAYER")]
    Multiplayer,
    #[serde(rename = "PURCHASE")]
    Purchase,
    #[serde(rename = "TRIAL")]
    Trial,
}
