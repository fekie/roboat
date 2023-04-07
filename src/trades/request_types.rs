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
