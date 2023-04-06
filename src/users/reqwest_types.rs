use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UserSearchResponse {
    pub previous_page_cursor: Option<String>,
    pub next_page_cursor: String,
    pub data: Vec<UserSearchUserInformationRaw>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UserSearchUserInformationRaw {
    pub id: u64,
    pub name: String,
    pub has_verified_badge: bool,
    pub previous_usernames: Vec<String>,
    pub display_name: String,
}
