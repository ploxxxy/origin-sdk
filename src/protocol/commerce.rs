use serde::{Deserialize, Serialize};

use crate::protocol::entitlements::Entitlement;

#[derive(Debug, Serialize, Deserialize)]
pub struct Catalog {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Status")]
    pub status: String,
    #[serde(rename = "@Currency")]
    pub currency_type: String,
    #[serde(rename = "@Group")]
    pub group: String,
    #[serde(rename = "@CatalogId")]
    pub catalog_id: u64,
    #[serde(rename = "Category", default)]
    pub categories: Vec<Category>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(rename = "@Type")]
    pub r#type: String,
    #[serde(rename = "@CategoryId")]
    pub category_id: String,
    #[serde(rename = "@ParentId")]
    pub parent_id: String,
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Description")]
    pub description: String,
    #[serde(rename = "@MostPopular")]
    pub most_popular: i32,
    #[serde(rename = "@ImageId")]
    pub image_id: String,
    #[serde(rename = "Category", default)]
    pub categories: Vec<Category>,
    #[serde(rename = "Offer", default)]
    pub offers: Vec<Offer>,
}

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
pub struct DetermineCommerceCurrency {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    #[serde(rename = "Catalog", default)]
    pub catalogs: Vec<Catalog>,
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Title")]
    pub title: String,
    #[serde(rename = "@Group")]
    pub group: String,
    #[serde(rename = "@Status")]
    pub status: String,
    #[serde(rename = "@DefaultCurrency")]
    pub default_currency: String,
    #[serde(rename = "@StoreId")]
    pub store_id: u64,
    #[serde(rename = "@IsDemoStore")]
    pub is_demo_store: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectStore {
    #[serde(rename = "@StoreId")]
    pub store_id: u64,
    #[serde(rename = "@CatalogId")]
    pub catalog_id: u64,
    #[serde(rename = "@EWalletCategoryId")]
    pub e_wallet_category_id: u64,
    #[serde(rename = "@VirtualCurrency")]
    pub virtual_currency: String,
    #[serde(rename = "@LockboxUrl")]
    pub lockbox_url: String,
    #[serde(rename = "@SuccessUrl")]
    pub success_url: String,
    #[serde(rename = "@FailedUrl")]
    pub failed_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCatalog {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCatalogResponse {
    #[serde(rename = "Catalog", default)]
    pub catalogs: Vec<Catalog>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStore {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "@StoreId")]
    pub store_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStoreResponse {
    #[serde(rename = "Store", default)]
    pub stores: Vec<Store>,
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
pub struct QueryCategories {
    #[serde(rename = "@UserId")]
    pub user_id: u64,
    #[serde(rename = "FilterCategories", default)]
    pub filter_categories: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryCategoriesResponse {
    #[serde(rename = "Categories", default)]
    pub categories: Vec<Category>,
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
