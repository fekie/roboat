use serde::{Deserialize, Serialize};
use super::{FriendRequestUserInformation, FriendsUserInformation};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct FriendsListResponse {
    pub data: Vec<FriendsUserInformation>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct FriendRequestsResponse {
    pub previous_page_cursor: Option<String>,
    pub next_page_cursor: Option<String>,

    pub data: Vec<FriendRequestUserInformation>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PendingFriendRequestsResponse {
    pub count: u64,
}
