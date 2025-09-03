use serde::{Deserialize, Serialize};

use crate::protocol::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetProfile {
    #[serde(rename = "@index")]
    pub index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetProfileResponse {
    #[serde(rename = "@UserIndex")]
    pub user_index: i32,
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@PersonaId")]
    pub persona_id: u64,
    #[serde(rename = "@Persona")]
    pub persona: String,
    #[serde(rename = "@AvatarId")]
    pub avatar_id: String,
    #[serde(rename = "@Country")]
    pub country: String,
    #[serde(rename = "@IsUnderAge")]
    pub is_under_age: bool,
    #[serde(rename = "@IsSubscriber")]
    pub is_subscriber: bool,
    #[serde(rename = "@IsTrialSubscriber")]
    pub is_trial_subscriber: bool,
    #[serde(rename = "@SubscriberLevel")]
    pub subscriber_level: i32,
    #[serde(rename = "@GeoCountry")]
    pub geo_country: String,
    #[serde(rename = "@CommerceCountry")]
    pub commerce_country: String,
    #[serde(rename = "@CommerceCurrency")]
    pub commerce_currency: String,
    #[serde(rename = "@IsSteamSubscriber")]
    pub is_steam_subscriber: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserProfileByEmailorEaid {
    #[serde(rename = "@KeyWord")]
    pub key_word: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserProfileByEmailorEaidResponse {
    #[serde(rename = "@Return")]
    pub r#return: String,
    #[serde(rename = "User", default)]
    pub user: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileEvent {
    #[serde(rename = "@Changed")]
    pub changed: ProfileStateChange,
    #[serde(rename = "@UserId")]
    pub user_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProfileStateChange {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "EAID")]
    Eaid,
    #[serde(rename = "AVATAR")]
    Avatar,
    #[serde(rename = "SUBSCRIPTION")]
    Subscription,
}
