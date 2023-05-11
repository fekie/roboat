use crate::catalog::CreatorType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct NonTradableLimitedDetailsRaw {
    pub collectible_item_id: String,
    pub name: String,
    pub description: String,
    pub collectible_product_id: String,
    pub creator_has_verified_badge: bool,
    pub creator_type: CreatorType,
    pub item_target_id: u64,
    pub creator_id: u64,
    pub creator_name: String,
    /// It's unclear which one of these to use
    pub price: u64,
    /// It's unclear which one of these to use
    pub lowest_price: u64,
    pub units_available_for_consumption: u64,
    pub off_sale_deadline: String,
    pub asset_stock: u64,
    pub error_code: Option<u64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PurchaseNonTradeableLimitedRaw {
    pub purchase_result: String,
    pub purchased: bool,
    /// Error variants: null, "PriceMismatch"
    pub error_message: Option<String>,
}
