use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockListUpdated;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockUser {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@UserIdToBlock")]
    pub user_id_to_block: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBlockList;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBlockListResponse {
    #[serde(rename = "@Return")]
    pub r#return: String,
    #[serde(rename = "User", default)]
    pub user: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnblockUser {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@UserIdToUnblock")]
    pub user_id_to_unblock: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "@EAID")]
    pub eaid: String,
    #[serde(rename = "@MAIL")]
    pub mail: String,
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@PersonaId")]
    pub persona_id: u64,
}
