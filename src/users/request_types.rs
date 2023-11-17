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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UsernameUserDetailsRequest {
    pub usernames: Vec<String>,
    pub exclude_banned_users: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UsernameUserInformationRaw {
    pub requested_username: String,
    pub has_verified_badge: bool,
    pub id: u64,
    pub name: String,
    pub display_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UsernameUserDetailsResponse {
    pub data: Vec<UsernameUserInformationRaw>,
}
