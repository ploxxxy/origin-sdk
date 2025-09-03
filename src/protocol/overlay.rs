use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IgoEvent {
    #[serde(rename = "@State")]
    pub state: IgoState,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IgoState {
    #[serde(rename = "DOWN")]
    Down,
    #[serde(rename = "UP")]
    Up,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IgoUnavailable {
    #[serde(rename = "@Reason")]
    pub reason: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IgoWindow {
    #[serde(rename = "LOGIN")]
    Login = 1,
    #[serde(rename = "PROFILE")]
    Profile,
    #[serde(rename = "RECENT")]
    Recent,
    #[serde(rename = "FEEDBACK")]
    Feedback,
    #[serde(rename = "FRIENDS")]
    Friends,
    #[serde(rename = "FRIEND_REQUEST")]
    FriendRequest,
    #[serde(rename = "CHAT")]
    Chat,
    #[serde(rename = "COMPOSE_CHAT")]
    ComposeChat,
    #[serde(rename = "INVITE")]
    Invite,
    #[serde(rename = "ACHIEVEMENTS")]
    Achievements,
    #[serde(rename = "STORE")]
    Store,
    #[serde(rename = "CODE_REDEMPTION")]
    CodeRedemption,
    #[serde(rename = "CHECKOUT")]
    Checkout,
    #[serde(rename = "BLOCKED")]
    Blocked,
    #[serde(rename = "BROWSER")]
    Browser,
    #[serde(rename = "FIND_FRIENDS")]
    FindFriends,
    #[serde(rename = "CHANGE_AVATAR")]
    ChangeAvatar,
    #[serde(rename = "GAMEDETAILS")]
    Gamedetails,
    #[serde(rename = "BROADCAST")]
    Broadcast,
    #[serde(rename = "UPSELL")]
    Upsell,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimizeRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowIgo {
    #[serde(rename = "@bShow")]
    pub b_show: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowIgoWindow {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@WindowId")]
    pub window_id: IgoWindow,
    #[serde(rename = "@Show", default = "ShowIgoWindow::default_show")]
    pub show: bool,
    #[serde(rename = "@Flags")]
    pub flags: i32,
    #[serde(rename = "@ContentId")]
    pub content_id: String,
    #[serde(rename = "TargetId", default)]
    pub target_id: Vec<u64>,
    #[serde(rename = "String")]
    pub string: String,
    #[serde(rename = "Args", default)]
    pub args: Vec<String>,
    #[serde(rename = "MasterTitleIds", default)]
    pub master_title_ids: Vec<String>,
    #[serde(rename = "Categories", default)]
    pub categories: Vec<String>,
    #[serde(rename = "Offers", default)]
    pub offers: Vec<String>,
}

impl ShowIgoWindow {
    #[must_use]
    pub fn default_show() -> bool {
        true
    }
}
