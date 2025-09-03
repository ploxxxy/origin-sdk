use serde::{Deserialize, Serialize};

use crate::protocol::common::Event;

#[derive(Debug, Serialize, Deserialize)]
pub struct Achievement {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Progress")]
    pub progress: i32,
    #[serde(rename = "@Total")]
    pub total: i32,
    #[serde(rename = "@Count")]
    pub count: i32,
    #[serde(rename = "@Description")]
    pub description: String,
    #[serde(rename = "@HowTo")]
    pub how_to: String,
    #[serde(rename = "@ImageId")]
    pub image_id: String,
    #[serde(rename = "@GrantDate")]
    pub grant_date: String,
    #[serde(rename = "@Expiration")]
    pub expiration: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementSet {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@GameName")]
    pub game_name: String,
    #[serde(rename = "Achievement", default)]
    pub achievement: Vec<Achievement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementSets {
    #[serde(rename = "AchievementSet", default)]
    pub achievement_set: Vec<AchievementSet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrantAchievement {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@PersonaId")]
    pub persona_id: u64,
    #[serde(rename = "@AchievementId")]
    pub achievement_id: i32,
    #[serde(rename = "@Progress")]
    pub progress: i32,
    #[serde(rename = "@AchievementCode")]
    pub achievement_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostAchievementEvents {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@PersonaId")]
    pub persona_id: u64,
    #[serde(rename = "Event", default)]
    pub events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryAchievements {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@PersonaId")]
    pub persona_id: u64,
    #[serde(rename = "@All")]
    pub all: bool,
    #[serde(rename = "GameId", default)]
    pub game_id: Vec<String>,
}
