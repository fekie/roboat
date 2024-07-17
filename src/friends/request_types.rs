use serde::{Deserialize, Serialize};

/// Model, representing user information that also contains select presence information
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendUserInformationRaw {
    #[serde(alias = "id")]
    pub id: u64,
    #[serde(alias = "name")]
    pub username: String,
    pub display_name: String,
    pub external_app_display_name: Option<String>,

    pub description: Option<String>,
    pub created: String,

    pub is_online: bool,
    pub is_deleted: bool,
    pub is_banned: bool,
    pub presence_type: Option<i32>,

    pub friend_frequent_score: i64,
    pub friend_frequent_rank: i64,

    pub has_verified_badge: bool,
}

/// Model, representing a friend request.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendRequestRaw {
    #[serde(alias = "id")]
    pub user_id: u64,

    #[serde(alias = "name")]
    pub username: String,

    pub display_name: String,

    pub external_app_display_name: Option<String>,

    pub has_verified_badge: bool,

    pub description: Option<String>,

    pub created: String,

    /// Whether the user is banned/terminated.
    #[serde(alias = "isBanned")]
    pub is_terminated: bool,

    pub mutual_friends_list: Vec<String>,

    pub friend_request: FriendRequestDetailsRaw,
}

/// Model, representing a friend request details.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendRequestDetailsRaw {
    /// The sender user Id.
    pub sender_id: u64,

    /// The source universe Id which the request was sent in.
    pub source_universe_id: u64,

    /// When the friend request was sent.
    pub sent_at: String,

    /// The origin source type associated with the friend request.
    /// ['Unknown' = 0, 'PlayerSearch' = 1, 'QrCode' = 2, 'InGame' = 3, 'UserProfile' = 4, 'QqContactImporter' = 5, 'WeChatContactImporter' = 6, 'ProfileShare' = 7, 'PhoneContactImporter' = 8, 'FriendRecommendations' = 9]
    pub origin_source_type: String,

    /// The contact name associated with the friend request.
    pub contact_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct FriendsListResponse {
    pub data: Vec<FriendUserInformationRaw>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct FriendRequestsResponse {
    pub previous_page_cursor: Option<String>,
    pub next_page_cursor: Option<String>,

    pub data: Vec<FriendRequestRaw>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PendingFriendRequestsResponse {
    pub count: u64,
}
