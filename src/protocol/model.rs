use serde::{Deserialize, Serialize};

use crate::xml;

xml!(Challenge {
    #[serde(rename = "@build")]
    pub build: String,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@key")]
    pub key: String,
});

xml!(ChallengeResponse {
    #[serde(rename = "@response")]
    pub response: String,
    #[serde(rename = "@key")]
    pub key: String,
    #[serde(rename = "@version")]
    pub protocol_version: String,
    #[serde(rename = "ContentId")]
    pub content_id: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "MultiplayerId")]
    pub multiplayer_id: String,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "Version")]
    pub sdk_version: String,
});

xml!(ChallengeAccepted {
    #[serde(rename = "@response")]
    pub response: String,
});

xml!(GetInternetConnectedState {});

xml!(InternetConnectedState {
    #[serde(rename = "@connected")]
    pub connected: bool,
});

xml!(QueryFriends {
    #[serde(rename = "@UserId")]
    pub user_id: String,
});

xml!(PresenceVisibilityEvent {
    #[serde(rename = "@Visible")]
    pub visible: bool,
});

xml!(Service {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Facility")]
    pub facility: Facility,
});

xml!(GetConfig {});

xml!(GetConfigResponse {
    #[serde(rename = "Service")]
    pub services: Vec<Service>,
});

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Facility {
    Achievement,
    AchievementEvent,
    ChatEvent,
    InviteEvent,
    BlockedUsers,
    BlockedUserEvent,
    Commerce,
    CommerceEvent,
    Content,
    DownloadEvent,
    Friends,
    FriendsEvent,
    Igo,
    IgoEvent,
    LoginEvent,
    OnlineStatusEvent,
    Permission,
    Presence,
    PresenceEvent,
    ProfileEvent,
    ProgressiveInstallation,
    ProgressiveInstallationEvent,
    Profile,
    Resources,
    Sdk,
    Utility,
    GetUserid,
    Xmpp,
}

xml!(GetProfile {
    #[serde(rename = "@index")]
    pub index: u32,
});

xml!(GetProfileResponse {
    #[serde(rename = "@Country")]
    pub country: String,
    #[serde(rename = "@IsTrialSubscriber")]
    pub is_trial_subscriber: bool,
    #[serde(rename = "@Persona")]
    pub persona: String,
    #[serde(rename = "@IsUnderAge")]
    pub is_under_age: bool,
    #[serde(rename = "@IsSteamSubscriber")]
    pub is_steam_subscriber: bool,
    #[serde(rename = "@CommerceCountry")]
    pub commerce_country: String,
    #[serde(rename = "@PersonaId")]
    pub persona_id: u64,
    #[serde(rename = "@AvatarId")]
    pub avatar_id: String,
    #[serde(rename = "@GeoCountry")]
    pub geo_country: String,
    #[serde(rename = "@CommerceCurrency")]
    pub commerce_currency: String,
    #[serde(rename = "@UserIndex")]
    pub user_index: u32,
    #[serde(rename = "@IsSubscriber")]
    pub is_subscriber: bool,
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@SubscriberLevel")]
    pub subscriber_level: u32,
});

xml!(CurrentUserPresenceEvent {
    #[serde(rename = "@Presence")]
    pub presence: String,
    #[serde(rename = "@GroupId")]
    pub group_id: String,
    #[serde(rename = "@TitleId")]
    pub title_id: String,
    #[serde(rename = "@SessionId")]
    pub session_id: String,
    #[serde(rename = "@Title")]
    pub title: String,
    #[serde(rename = "@Group")]
    pub group: String,
    #[serde(rename = "@GamePresence")]
    pub game_presence: String,
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@RichPresence")]
    pub rich_presence: String,
    #[serde(rename = "@MultiplayerId")]
    pub multiplayer_id: String,
});
