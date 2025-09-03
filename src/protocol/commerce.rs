use serde::{Deserialize, Serialize};

use crate::protocol::entitlements::Entitlement;

#[derive(Debug, Serialize, Deserialize)]
pub struct Checkout {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@Currency")]
    pub currency: String,
    #[serde(rename = "Offers", default)]
    pub offers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWalletBalance {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@Currency")]
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWalletBalanceResponse {
    #[serde(rename = "@Balance")]
    pub balance: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Offer {
    #[serde(rename = "@Type")]
    pub r#type: String,
    #[serde(rename = "@OfferId")]
    pub offer_id: String,
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Description")]
    pub description: String,
    #[serde(rename = "@ImageId")]
    pub image_id: String,
    #[serde(rename = "@GameDistributionSub")]
    pub game_distribution_sub_type: String,
    #[serde(rename = "@bIsOwned")]
    pub b_is_owned: bool,
    #[serde(rename = "@bHidden")]
    pub b_hidden: bool,
    #[serde(rename = "@bCanPurchase")]
    pub b_can_purchase: bool,
    #[serde(rename = "@PurchaseDate")]
    pub purchase_date: String,
    #[serde(rename = "@DownloadDate")]
    pub download_date: String,
    #[serde(rename = "@PlayableDate")]
    pub playable_date: String,
    #[serde(rename = "@UseEndDate")]
    pub use_end_date: String,
    #[serde(rename = "@DownloadSize")]
    pub download_size: u64,
    #[serde(rename = "@Currency")]
    pub currency: String,
    #[serde(rename = "@bIsDiscounted")]
    pub b_is_discounted: bool,
    #[serde(rename = "@Price")]
    pub price: f64,
    #[serde(rename = "@LocalizedPrice")]
    pub localized_price: String,
    #[serde(rename = "@OriginalPrice")]
    pub original_price: f64,
    #[serde(rename = "@LocalizedOriginalPrice")]
    pub localized_original_price: String,
    #[serde(rename = "@InventoryCap")]
    pub inventory_cap: i32,
    #[serde(rename = "@InventorySold")]
    pub inventory_sold: i32,
    #[serde(rename = "@InventoryAvailable")]
    pub inventory_available: i32,
    #[serde(rename = "Entitlements", default)]
    pub entitlements: Vec<Entitlement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseEvent {
    #[serde(rename = "@manifest")]
    pub manifest: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryOffers {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "FilterCategories", default)]
    pub filter_categories: Vec<String>,
    #[serde(rename = "FilterMasterTitleIds", default)]
    pub filter_master_title_ids: Vec<String>,
    #[serde(rename = "FilterOffers", default)]
    pub filter_offers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryOffersResponse {
    #[serde(rename = "Offer", default)]
    pub offers: Vec<Offer>,
}
