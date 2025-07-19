use serde::{Deserialize, Serialize};

// [Friends list structs]

/// Structure that contains vector of data (FriendUserInformation) from FrindsList Response
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FriendsListResponse {
    pub data: Vec<FriendUserInformation>,
}

/// Model, representing user information that also contains select presence information
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendUserInformation {
    pub id: u64,
    pub name: String,
    pub display_name: String,
}

// [Pending request struct]

/// Struct to get the count of friend requests
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PendingFriendRequestsResponse {
    pub count: u64,
}

// [Friend requests structs]

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequestResponseData {
    #[serde(rename = "friendRequest")]
    pub request_info: RequestInfo,
    pub mutual_friends_list: Vec<String>,
    pub id: u64,
    pub name: String,
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequestInfo {
    pub sent_at: String,
    pub sender_id: u64,
    pub source_universe_id: u64,
    pub origin_source_type: OriginSource,
    // NOTE:These are always null
    // contact_name: Option<String>,
    // sender_nickname: String,
}

/// Represents the source of a friend request or friend connection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OriginSource {
    /// Source is unknown or not specified
    #[serde(rename = "Unknown")]
    Unknown = 0,

    /// Friend was found through player search
    #[serde(rename = "PlayerSearch")]
    PlayerSearch = 1,

    /// Friend was added via QR code
    #[serde(rename = "QrCode")]
    QrCode = 2,

    /// Friend was met in-game
    #[serde(rename = "InGame")]
    InGame = 3,

    /// Friend was found through user profile
    #[serde(rename = "UserProfile")]
    UserProfile = 4,

    /// Friend was imported from QQ contacts
    #[serde(rename = "QqContactImporter")]
    QqContactImporter = 5,

    /// Friend was imported from WeChat contacts
    #[serde(rename = "WeChatContactImporter")]
    WeChatContactImporter = 6,

    /// Friend was added through profile sharing
    #[serde(rename = "ProfileShare")]
    ProfileShare = 7,

    /// Friend was imported from phone contacts
    #[serde(rename = "PhoneContactImporter")]
    PhoneContactImporter = 8,

    /// Friend was suggested through recommendations
    #[serde(rename = "FriendRecommendations")]
    FriendRecommendations = 9,
}

impl Default for OriginSource {
    fn default() -> Self {
        OriginSource::Unknown
    }
}

// Optionally, implement From<u8> to convert from numeric values
impl From<u8> for OriginSource {
    fn from(value: u8) -> Self {
        match value {
            1 => OriginSource::PlayerSearch,
            2 => OriginSource::QrCode,
            3 => OriginSource::InGame,
            4 => OriginSource::UserProfile,
            5 => OriginSource::QqContactImporter,
            6 => OriginSource::WeChatContactImporter,
            7 => OriginSource::ProfileShare,
            8 => OriginSource::PhoneContactImporter,
            9 => OriginSource::FriendRecommendations,
            _ => OriginSource::Unknown,
        }
    }
}
