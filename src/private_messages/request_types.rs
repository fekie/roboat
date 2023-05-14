use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct MessagesResponse {
    pub collection: Vec<MessageRaw>,
    pub total_collection_size: i64,
    pub total_pages: i64,
    pub page_number: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct MessageRaw {
    pub id: i64,
    pub sender: Sender,
    pub recipient: Recipient,
    pub subject: String,
    pub body: String,
    pub created: String,
    pub updated: String,
    pub is_read: bool,
    pub is_system_message: bool,
    pub is_report_abuse_displayed: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Sender {
    pub has_verified_badge: bool,
    pub id: i64,
    pub name: String,
    pub display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Recipient {
    pub has_verified_badge: bool,
    pub id: i64,
    pub name: String,
    pub display_name: String,
}
