use super::TradeStatus;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct InboundTradesResponse {
    pub previous_page_cursor: Option<String>,
    pub next_page_cursor: Option<String>,
    pub data: Vec<TradeRaw>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct TradeRaw {
    pub id: i64,
    pub user: TradeUserRaw,
    pub created: String,
    pub expiration: String,
    pub is_active: bool,
    pub status: TradeStatus,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct TradeUserRaw {
    pub id: i64,
    pub name: String,
    pub display_name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct TradeDetailsResponse {
    /// The account's user is the first "offer", and the trade partner is the next.
    /// This is true for inbound, outbound, and completed trades.
    pub offers: Vec<Offer>,
    pub id: i64,
    pub user: User,
    pub created: String,
    pub expiration: Option<String>,
    pub is_active: bool,
    pub status: String,
}

/// Each offer is a side of a trade
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Offer {
    pub user: User,
    pub user_assets: Vec<UserAsset>,
    pub robux: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct User {
    pub id: i64,
    pub name: String,
    pub display_name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UserAsset {
    pub id: i64,
    pub serial_number: Option<i64>,
    pub asset_id: i64,
    pub name: String,
    pub recent_average_price: i64,
    pub original_price: Option<i64>,
    pub asset_stock: Option<i64>,
    pub membership_type: Option<String>,
}
