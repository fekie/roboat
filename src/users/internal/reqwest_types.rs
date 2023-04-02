use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(super) struct UserSearchUserInformationRaw {
    #[serde(rename(deserialize = "id"))]
    pub user_id: u64,
    #[serde(rename(deserialize = "name"))]
    pub username: String,
    #[serde(rename(deserialize = "hasVerifiedBadge"))]
    pub has_verified_badge: bool,
    #[serde(rename(deserialize = "previousUsernames"))]
    pub previous_usernames: Vec<String>,
    #[serde(rename(deserialize = "displayName"))]
    pub display_name: String,
}

#[derive(Serialize, Deserialize)]
pub (super) struct UserSearchResponse {
    #[serde(rename(deserialize = "previousPageCursor"))]
    #[serde(rename(deserialize = "previous_page_cursor"))]
    pub previous_page_cursor: String,
    #[serde(rename(deserialize = "nextPageCursor"))]
    #[serde(rename(deserialize = "next_page_cursor"))]
    pub next_page_cursor: String,
    #[serde(rename(deserialize = "data"))]
    #[serde(rename(deserialize = "data"))]
    pub data: Vec<UserSearchUserInformationRaw>,
}
