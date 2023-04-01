use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CurrencyResponse {
    pub(crate) robux: u64,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ResellersResponse {
    #[serde(rename = "previousPageCursor")]
    pub(crate) previous_page_cursor: Option<String>,
    #[serde(rename = "nextPageCursor")]
    pub(crate) next_page_cursor: Option<String>,
    pub(crate) data: Vec<ListingRaw>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ListingRaw {
    #[serde(rename = "userAssetId")]
    pub(crate) user_asset_id: u64,
    pub(crate) seller: ResellerRaw,
    pub(crate) price: u64,
    #[serde(rename = "serialNumber")]
    pub(crate) serial_number: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ResellerRaw {
    #[serde(rename = "hasVerifiedBadge")]
    pub(crate) has_verified_badge: bool,
    pub(crate) id: u64,
    #[serde(rename = "type")]
    pub(crate) seller_type: Option<serde_json::Value>,
    pub(crate) name: String,
}
