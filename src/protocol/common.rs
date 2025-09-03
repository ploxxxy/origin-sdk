use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorSuccess {
    #[serde(rename = "@Code")]
    pub code: i32,
    #[serde(rename = "@Description")]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "@EventId")]
    pub event_id: String,
    #[serde(rename = "EventParam", default)]
    pub attributes: Vec<EventParam>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventParam {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Facility {
    #[serde(rename = "SDK")]
    Sdk,
    #[serde(rename = "PROFILE")]
    Profile,
    #[serde(rename = "PRESENCE")]
    Presence,
    #[serde(rename = "FRIENDS")]
    Friends,
    #[serde(rename = "COMMERCE")]
    Commerce,
    #[serde(rename = "RECENTPLAYER")]
    RecentPlayer,
    #[serde(rename = "IGO")]
    Igo,
    #[serde(rename = "MISC")]
    Misc,
    #[serde(rename = "LOGIN")]
    Login,
    #[serde(rename = "UTILITY")]
    Utility,
    #[serde(rename = "XMPP")]
    Xmpp,
    #[serde(rename = "CHAT")]
    Chat,
    #[serde(rename = "IGO_EVENT")]
    IgoEvent,
    #[serde(rename = "EALS_EVENTS")]
    EalsEvents,
    #[serde(rename = "LOGIN_EVENT")]
    LoginEvent,
    #[serde(rename = "INVITE_EVENT")]
    InviteEvent,
    #[serde(rename = "PROFILE_EVENT")]
    ProfileEvent,
    #[serde(rename = "PRESENCE_EVENT")]
    PresenceEvent,
    #[serde(rename = "FRIENDS_EVENT")]
    FriendsEvent,
    #[serde(rename = "COMMERCE_EVENT")]
    CommerceEvent,
    #[serde(rename = "CHAT_EVENT")]
    ChatEvent,
    #[serde(rename = "DOWNLOAD_EVENT")]
    DownloadEvent,
    #[serde(rename = "PERMISSION")]
    Permission,
    #[serde(rename = "RESOURCES")]
    Resources,
    #[serde(rename = "BLOCKED_USERS")]
    BlockedUsers,
    #[serde(rename = "BLOCKED_USER_EVENT")]
    BlockedUserEvent,
    #[serde(rename = "GET_USERID")]
    GetUserId,
    #[serde(rename = "ONLINE_STATUS_EVENT")]
    OnlineStatusEvent,
    #[serde(rename = "ACHIEVEMENT")]
    Achievement,
    #[serde(rename = "ACHIEVEMENT_EVENT")]
    AchievementEvent,
    #[serde(rename = "BROADCAST_EVENT")]
    BroadcastEvent,
    #[serde(rename = "PROGRESSIVE_INSTALLATION")]
    ProgressiveInstallation,
    #[serde(rename = "PROGRESSIVE_INSTALLATION_EVENT")]
    ProgressiveInstallationEvent,
    #[serde(rename = "CONTENT")]
    Content,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "@ImageId")]
    pub image_id: String,
    #[serde(rename = "@Width")]
    pub width: i32,
    #[serde(rename = "@Height")]
    pub height: i32,
    #[serde(rename = "@ResourcePath")]
    pub resource_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryImage {
    #[serde(rename = "@ImageId")]
    pub image_id: String,
    #[serde(rename = "@Width")]
    pub width: i32,
    #[serde(rename = "@Height")]
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryImageResponse {
    #[serde(rename = "@Result")]
    pub result: i32,
    #[serde(rename = "Image", default)]
    pub images: Vec<Image>,
}
