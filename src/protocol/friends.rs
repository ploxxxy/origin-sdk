use serde::{Deserialize, Serialize};

use crate::protocol::presence::Presence;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddRecentPlayers {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "Player")]
    pub player: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Friend {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@PersonaId")]
    pub persona_id: u64,
    #[serde(rename = "@Persona")]
    pub persona: String,
    #[serde(rename = "@AvatarId")]
    pub avatar_id: String,
    #[serde(rename = "@Group")]
    pub group: String,
    #[serde(rename = "@GroupId")]
    pub group_id: String,
    #[serde(rename = "@Presence")]
    pub presence: Presence,
    #[serde(rename = "@State")]
    pub state: FriendState,
    #[serde(rename = "@TitleId")]
    pub title_id: String,
    #[serde(rename = "@Title")]
    pub title: String,
    #[serde(rename = "@MultiplayerId")]
    pub multiplayer_id: String,
    #[serde(rename = "@RichPresence")]
    pub rich_presence: String,
    #[serde(rename = "@GamePresence")]
    pub game_presence: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FriendState {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "MUTUAL")]
    Mutual,
    #[serde(rename = "INVITED")]
    Invited,
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "REQUEST")]
    Request,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FriendStatus {
    #[serde(rename = "@FriendId")]
    pub friend_id: u64,
    #[serde(rename = "@State")]
    pub state: FriendState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FriendsEvent {
    #[serde(rename = "@value")]
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryAreFriends {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "Friends", default)]
    pub friends: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryAreFriendsResponse {
    #[serde(rename = "FriendStatus", default)]
    pub users: Vec<FriendStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryFriends {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryFriendsResponse {
    #[serde(rename = "Friend", default)]
    pub friends: Vec<Friend>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveFriend {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@UserToRemove")]
    pub user_to_remove: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestFriend {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@UserToAdd")]
    pub user_to_add: u64,
}
