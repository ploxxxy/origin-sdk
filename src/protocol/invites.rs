use serde::{Deserialize, Serialize};

use crate::protocol::groups::GroupType;

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptFriendInvite {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@OtherId")]
    pub other_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptInvite {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@OtherId")]
    pub other_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupInviteEvent {
    #[serde(rename = "@GroupName")]
    pub group_name: String,
    #[serde(rename = "@GroupId")]
    pub group_id: String,
    #[serde(rename = "@GroupType")]
    pub group_type: GroupType,
    #[serde(rename = "@FromId")]
    pub from_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteUsersToGroup {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "FriendId", default)]
    pub friend_id: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiplayerInvite {
    #[serde(rename = "@GroupId")]
    pub group_id: String,
    #[serde(rename = "@GroupName")]
    pub group_name: String,
    #[serde(rename = "@multiplayerId")]
    pub multiplayer_id: String,
    #[serde(rename = "@initial")]
    pub initial: bool,
    #[serde(rename = "@from")]
    pub from: u64,
    #[serde(rename = "SessionInformation")]
    pub session_information: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiplayerInvitePending {
    #[serde(rename = "@MultiplayerId")]
    pub multiplayer_id: String,
    #[serde(rename = "@GroupName")]
    pub group_name: String,
    #[serde(rename = "@GroupId")]
    pub group_id: String,
    #[serde(rename = "@from")]
    pub from: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendGroupGameInvite {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@Message")]
    pub message: String,
    #[serde(rename = "Invitees", default)]
    pub invitees: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendInvite {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@Invitation")]
    pub invitation: String,
    #[serde(rename = "Invitees", default)]
    pub invitees: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInvitedEvent {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
}
