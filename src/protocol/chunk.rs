use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AreChunksInstalled {
    #[serde(rename = "@ItemId")]
    pub item_id: String,
    #[serde(rename = "ChunkIds", default)]
    pub chunk_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreChunksInstalledResponse {
    #[serde(rename = "@ItemId")]
    pub item_id: String,
    #[serde(rename = "@Installed")]
    pub installed: bool,
    #[serde(rename = "ChunkIds", default)]
    pub chunk_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChunkState {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "QUEUED")]
    Queued,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "DOWNLOADING")]
    Downloading,
    #[serde(rename = "INSTALLING")]
    Installing,
    #[serde(rename = "INSTALLED")]
    Installed,
    #[serde(rename = "BUSY")]
    Busy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkStatus {
    #[serde(rename = "@ChunkId")]
    pub chunk_id: i32,
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@ItemId")]
    pub item_id: String,
    #[serde(rename = "@Type")]
    pub r#type: ChunkType,
    #[serde(rename = "@State")]
    pub state: ChunkState,
    #[serde(rename = "@Progress")]
    pub progress: f32,
    #[serde(rename = "@Size")]
    pub size: u64,
    #[serde(rename = "@ChunkETA")]
    pub chunk_eta: i32,
    #[serde(rename = "@TotalETA")]
    pub total_eta: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChunkType {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "REQUIRED")]
    Required,
    #[serde(rename = "RECOMMENDED")]
    Recommended,
    #[serde(rename = "NORMAL")]
    Normal,
    #[serde(rename = "ONDEMAND")]
    Ondemand,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChunk {
    #[serde(rename = "@ItemId")]
    pub item_id: String,
    #[serde(rename = "Files", default)]
    pub files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChunkResponse {
    #[serde(rename = "@ChunkId")]
    pub chunk_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetChunkPriority {
    #[serde(rename = "@ItemId")]
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetChunkPriorityResponse {
    #[serde(rename = "@ItemId")]
    pub item_id: String,
    #[serde(rename = "ChunkIds", default)]
    pub chunk_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsFileDownloaded {
    #[serde(rename = "@ItemId")]
    pub item_id: String,
    #[serde(rename = "@Filepath")]
    pub filepath: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsFileDownloadedResponse {
    #[serde(rename = "@ItemId")]
    pub item_id: String,
    #[serde(rename = "@Filepath")]
    pub filepath: String,
    #[serde(rename = "@Downloaded")]
    pub downloaded: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsProgressiveInstallationAvailable {
    #[serde(rename = "@ItemId")]
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsProgressiveInstallationAvailableResponse {
    #[serde(rename = "@ItemId")]
    pub item_id: String,
    #[serde(rename = "@Available")]
    pub available: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryChunkFiles {
    #[serde(rename = "@ItemId")]
    pub item_id: String,
    #[serde(rename = "@ChunkId")]
    pub chunk_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryChunkFilesResponse {
    #[serde(rename = "Files", default)]
    pub files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryChunkStatus {
    #[serde(rename = "@ItemId")]
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryChunkStatusResponse {
    #[serde(rename = "ChunkStatus", default)]
    pub chunk_status: Vec<ChunkStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetChunkPriority {
    #[serde(rename = "@ItemId")]
    pub item_id: String,
    #[serde(rename = "ChunkIds", default)]
    pub chunk_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetDownloaderUtilization {
    #[serde(rename = "@Utilization")]
    pub utilization: f32,
}
