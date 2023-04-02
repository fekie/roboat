use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct CurrencyResponse {
    pub robux: u64,
}

#[derive(Serialize, Deserialize)]
pub(super) struct ResellersResponse {
    #[serde(rename = "previousPageCursor")]
    pub previous_page_cursor: Option<String>,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: Option<String>,
    pub data: Vec<ListingRaw>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct ListingRaw {
    #[serde(rename = "userAssetId")]
    pub user_asset_id: u64,
    pub seller: ResellerRaw,
    pub price: u64,
    #[serde(rename = "serialNumber")]
    pub serial_number: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct ResellerRaw {
    #[serde(rename = "hasVerifiedBadge")]
    pub has_verified_badge: bool,
    pub id: u64,
    #[serde(rename = "type")]
    pub seller_type: Option<serde_json::Value>,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub(super) struct UserSalesResponse {
    #[serde(rename = "previousPageCursor")]
    pub previous_page_cursor: Option<String>,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: Option<String>,
    pub data: Vec<SaleRaw>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct SaleRaw {
    #[serde(rename = "id")]
    pub sale_id: u64,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    #[serde(rename = "agent")]
    pub user: UserRaw,
    pub details: DetailsRaw,
    pub currency: CurrencyRaw,
}

// This is what they call the user that bought the item for some reason.
#[derive(Serialize, Deserialize)]
pub(super) struct UserRaw {
    pub id: u64,
    #[serde(rename = "name")]
    pub user_display_name: String,
}

#[derive(Serialize, Deserialize)]
pub(super) struct DetailsRaw {
    pub id: u64,
    #[serde(rename = "name")]
    pub item_name: String,
}

#[derive(Serialize, Deserialize)]
pub(super) struct CurrencyRaw {
    pub amount: u64,
    #[serde(rename = "type")]
    pub currency_type: CurrencyTypeRaw,
}

#[derive(Serialize, Deserialize)]
pub(super) enum CurrencyTypeRaw {
    Robux,
}

#[derive(Serialize, Deserialize)]
pub(super) enum TransactionTypeRaw {
    Sale,
}