use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct CurrencyResponse {
    pub robux: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ResellersResponse {
    pub previous_page_cursor: Option<String>,
    pub next_page_cursor: Option<String>,
    pub data: Vec<ListingRaw>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ListingRaw {
    pub user_asset_id: u64,
    pub seller: ResellerRaw,
    pub price: u64,
    pub serial_number: Option<u64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ResellerRaw {
    pub has_verified_badge: bool,
    pub id: u64,
    #[serde(rename = "type")]
    pub seller_type: Option<serde_json::Value>,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UserSalesResponse {
    pub previous_page_cursor: Option<String>,
    pub next_page_cursor: Option<String>,
    pub data: Vec<SaleRaw>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct SaleRaw {
    pub id: u64,
    pub is_pending: bool,
    pub agent: UserRaw,
    pub details: DetailsRaw,
    pub currency: CurrencyRaw,
}

// This is what they call the user that bought the item for some reason.
#[derive(Serialize, Deserialize)]
pub(super) struct UserRaw {
    pub id: u64,
    /// This is the user's display name.
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub(super) struct DetailsRaw {
    pub id: u64,
    /// The name of the item.
    pub name: String,
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseLimitedResponse {
    pub purchased: bool,
    pub error_msg: String,
}
