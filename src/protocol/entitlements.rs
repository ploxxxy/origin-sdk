use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumeEntitlement {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@Uses")]
    pub uses: i32,
    #[serde(rename = "@bOveruse")]
    pub b_overuse: bool,
    #[serde(rename = "Entitlement")]
    pub entitlement: Entitlement,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumeEntitlementResponse {
    #[serde(rename = "Entitlement")]
    pub entitlement: Entitlement,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entitlement {
    #[serde(rename = "@Type")]
    pub r#type: String,
    #[serde(rename = "@ItemId")]
    pub item_id: String,
    #[serde(rename = "@EntitlementId")]
    pub entitlement_id: String,
    #[serde(rename = "@EntitlementTag")]
    pub entitlement_tag: String,
    #[serde(rename = "@Group")]
    pub group: String,
    #[serde(rename = "@ResourceId")]
    pub resource_id: String,
    #[serde(rename = "@UseCount")]
    pub use_count: i32,
    #[serde(rename = "@Expiration")]
    pub expiration: String,
    #[serde(rename = "@GrantDate")]
    pub grant_date: String,
    #[serde(rename = "@LastModifiedDate")]
    pub last_modified_date: String,
    #[serde(rename = "@Version")]
    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendTrial {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@RequestTicket")]
    pub request_ticket: String,
    #[serde(rename = "@TicketEngine")]
    pub ticket_engine: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendTrialResponse {
    #[serde(rename = "@Code")]
    pub code: i32,
    #[serde(rename = "@TotalTimeRemaining")]
    pub total_time_remaining: i32,
    #[serde(rename = "@TimeGranted")]
    pub time_granted: i32,
    #[serde(rename = "@ResponseTicket")]
    pub response_ticket: String,
    #[serde(
        rename = "@RetryCount",
        default = "ExtendTrialResponse::default_retry_count"
    )]
    pub retry_count: i32,
    #[serde(
        rename = "@RetryAfterFailSec",
        default = "ExtendTrialResponse::default_retry_after_fail_sec"
    )]
    pub retry_after_fail_sec: i32,
    #[serde(
        rename = "@ExtendBeforeExpireSec",
        default = "ExtendTrialResponse::default_extend_before_expire_sec"
    )]
    pub extend_before_expire_sec: i32,
    #[serde(
        rename = "@SleepBeforeNukeSec",
        default = "ExtendTrialResponse::default_sleep_before_nuke_sec"
    )]
    pub sleep_before_nuke_sec: i32,
}

impl ExtendTrialResponse {
    #[must_use]
    pub fn default_retry_count() -> i32 {
        5i32
    }

    #[must_use]
    pub fn default_retry_after_fail_sec() -> i32 {
        10i32
    }

    #[must_use]
    pub fn default_extend_before_expire_sec() -> i32 {
        60i32
    }

    #[must_use]
    pub fn default_sleep_before_nuke_sec() -> i32 {
        20i32
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryEntitlements {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@OfferId")]
    pub offer_id: String,
    #[serde(rename = "@ItemId")]
    pub item_id: String,
    #[serde(rename = "@Group")]
    pub group: String,
    #[serde(rename = "@includeChildGroups")]
    pub include_child_groups: bool,
    #[serde(rename = "@includeExpiredTrialDLC")]
    pub include_expired_trial_dlc: bool,
    #[serde(rename = "FilterOffers", default)]
    pub filter_offers: Vec<String>,
    #[serde(rename = "FilterItems", default)]
    pub filter_items: Vec<String>,
    #[serde(rename = "FilterGroups", default)]
    pub filter_groups: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryEntitlementsResponse {
    #[serde(rename = "Entitlement", default)]
    pub entitlements: Vec<Entitlement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryManifest {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@Manifest")]
    pub manifest: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryManifestResponse {
    #[serde(rename = "Entitlement", default)]
    pub entitlements: Vec<Entitlement>,
}
