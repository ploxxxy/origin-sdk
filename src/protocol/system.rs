use serde::{Deserialize, Serialize};

use crate::protocol::common::Facility;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetConfigResponse {
    #[serde(rename = "Service", default)]
    pub services: Vec<Service>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInternetConnectedState;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSettings;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSettingsResponse {
    #[serde(rename = "@Language")]
    pub language: String,
    #[serde(rename = "@Environment")]
    pub environment: String,
    #[serde(rename = "@IsIGOAvailable")]
    pub is_igo_available: bool,
    #[serde(rename = "@IsIGOEnabled")]
    pub is_igo_enabled: bool,
    #[serde(rename = "@IsTelemetryEnabled")]
    pub is_telemetry_enabled: bool,
    #[serde(rename = "@IsManualOffline")]
    pub is_manual_offline: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUtcTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUtcTimeResponse {
    #[serde(rename = "@Time")]
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InternetConnectedState {
    #[serde(rename = "@connected")]
    pub connected: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Facility")]
    pub facility: Facility,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Setting {
    #[serde(rename = "LANGUAGE")]
    Language,
    #[serde(rename = "ENVIRONMENT")]
    Environment,
    #[serde(rename = "IS_IGO_AVAILABLE")]
    IsIgoAvailable,
    #[serde(rename = "IS_IGO_ENABLED")]
    IsIgoEnabled,
    #[serde(rename = "IS_TELEMETRY_ENABLED")]
    IsTelemetryEnabled,
    #[serde(rename = "IS_MANUAL_OFFLINE")]
    IsManualOffline,
}
