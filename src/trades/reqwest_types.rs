use super::TradeStatus;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(super) struct InboundTradesResponse {
    #[serde(rename = "previousPageCursor")]
    pub previous_page_cursor: Option<String>,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: Option<String>,
    pub data: Vec<TradeRaw>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct TradeRaw {
    pub id: i64,
    pub user: TradeUserRaw,
    pub created: String,
    pub expiration: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub status: TradeStatus,
}

#[derive(Serialize, Deserialize)]
pub(super) struct TradeUserRaw {
    pub id: i64,
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}
