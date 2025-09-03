use serde::{Deserialize, Serialize};

use crate::protocol::friends::Friend;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGroup {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@GroupName")]
    pub group_name: String,
    #[serde(rename = "@GroupType")]
    pub group_type: GroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnterGroup {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@GroupId")]
    pub group_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetGroupInfo {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@GroupId")]
    pub group_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupEnterEvent {
    #[serde(rename = "GroupInfo")]
    pub group_info: GroupInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupEvent {
    #[serde(rename = "Friend", default)]
    pub members: Vec<Friend>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupInfo {
    #[serde(rename = "@GroupName")]
    pub group_name: String,
    #[serde(rename = "@GroupId")]
    pub group_id: String,
    #[serde(rename = "@GroupType")]
    pub group_type: GroupType,
    #[serde(rename = "@CanInviteNewMembers")]
    pub can_invite_new_members: bool,
    #[serde(rename = "@CanRemoveMembers")]
    pub can_remove_members: bool,
    #[serde(rename = "@CanSendGameInvites")]
    pub can_send_game_invites: bool,
    #[serde(
        rename = "@MaxGroupSize",
        default = "GroupInfo::default_max_group_size"
    )]
    pub max_group_size: i32,
}

impl GroupInfo {
    #[must_use]
    pub fn default_max_group_size() -> i32 {
        16i32
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupLeaveEvent {
    #[serde(rename = "@GroupId")]
    pub group_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GroupType {
    #[serde(rename = "PUBLIC")]
    Public,
    #[serde(rename = "PRIVATE")]
    Private,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaveGroup {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryGroup {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@GroupId")]
    pub group_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryGroupResponse {
    #[serde(rename = "Friend", default)]
    pub members: Vec<Friend>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveUsersFromGroup {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "FriendId", default)]
    pub friend_id: Vec<u64>,
}
