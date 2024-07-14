use reqwest::header;
use serde::{Deserialize, Serialize};

use crate::{Client, RoboatError};

mod request_types;

const FRIENDS_LIST: &str = "https://friends.roblox.com/v1/users/{user_id}/friends";
const FRIEND_REQUESTS: &str = "https://friends.roblox.com/v1/my/friends/requests";
const PENDING_FRIEND_REQUESTS: &str = "https://friends.roblox.com/v1/user/friend-requests/count";

// TODO: take out this enum to presence
/// Presence of user
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub enum PresenceType {
    #[default]
    Offline,
    Online,
    InGame,
    InStudio,
    Invisible,
}

impl TryFrom<i32> for PresenceType {
    type Error = RoboatError;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Offline),
            1 => Ok(Self::Online),
            2 => Ok(Self::InGame),
            3 => Ok(Self::InStudio),
            4 => Ok(Self::Invisible),
            _ => Err(RoboatError::MalformedResponse)
        }
    }
}

/// Model, representing user information that also contains select presence information
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct FriendUserInformation {
    #[serde(alias = "id")]
    pub user_id: u64,

    #[serde(alias = "name")]
    pub username: String,

    #[serde(alias = "displayName")]
    pub display_name: String,

    pub description: Option<String>,

    pub created: String,

    pub presence_type: PresenceType,

    /// Whether the user is banned/terminated.
    #[serde(alias = "isBanned")]
    pub is_terminated: bool,

    /// The user's verified badge status.
    pub has_verified_badge: bool,
}

/// Model, representing a friend request.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct FriendRequest {
    #[serde(alias = "id")]
    pub user_id: u64,

    #[serde(alias = "name")]
    pub username: String,

    #[serde(alias = "displayName")]
    pub display_name: String,

    pub description: Option<String>,

    pub created: String,

    #[serde(alias = "isBanned")]
    pub is_terminated: bool,

    #[serde(alias = "hasVerifiedBadge")]
    pub has_verified_badge: bool,

    #[serde(alias = "mutualFriendsList")]
    pub mutual_friends_list: Vec<String>,

    /// The origin source type associated with the friend request.
    pub origin_source_type: String,

    /// The source universe id which the request was sent in.
    /// # Note
    ///  * Default universe id: `0`
    pub source_universe_id: u64,

    /// When the friend request was sent.
    pub sent_at: String,
}


impl Client {
    /// Get list of all friends for the specified user using <https://friends.roblox.com/v1/users/{userId}/friends>.
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    /// const USER_ID: u64 = 1692828498;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// let friends = client.friends_list(USER_ID).await?;
    ///
    /// println!("Found {} friends.", friends.len());
    ///
    /// for friend in friends {
    ///     println!("{}: {}", friend.username, friend.user_id);
    /// }
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn friends_list(&self, user_id: u64) -> Result<Vec<FriendUserInformation>, RoboatError> {
        let formatted_url = FRIENDS_LIST.replace("{user_id}", &user_id.to_string());

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;

        let raw = Self::parse_to_raw::<request_types::FriendsListResponse>(response).await?;

        let mut friends = Vec::new();

        for friend_raw in raw.data {
            let friend = FriendUserInformation {
                user_id: friend_raw.id,
                username: friend_raw.username,
                display_name: friend_raw.display_name,

                description: friend_raw.description,
                created: friend_raw.created,

                presence_type: PresenceType::try_from(friend_raw.presence_type.unwrap_or(0))
                    .unwrap_or(PresenceType::Offline),
                is_terminated: friend_raw.is_banned,

                has_verified_badge: friend_raw.has_verified_badge,
            };

            friends.push(friend);
        }

        Ok(friends)
    }

    /// Get list of friend requests with cursor using <https://friends.roblox.com/v1/my/friends/requests>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// let (friend_requests, next_cursor) = client.friend_requests(None).await?;
    ///
    /// for user in friend_requests {
    ///     println!("{} from {}: {}", user.username, user.origin_source_type,  user.user_id);
    /// }
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn friend_requests(
        &self,
        cursor: Option<String>,
    ) -> Result<(Vec<FriendRequest>, Option<String>), RoboatError> {
        let cookie = self.cookie_string()?;
        let mut formatted_url = format!("{}?limit={}", FRIEND_REQUESTS, 10);

        if let Some(cursor) = cursor {
            formatted_url = format!("{}&cursor={}", formatted_url, cursor)
        }

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .header(header::COOKIE, cookie)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;

        let raw = Self::parse_to_raw::<request_types::FriendRequestsResponse>(response).await?;

        let mut friend_requests = Vec::new();

        for friend_request_raw in raw.data {
            let friend_request = FriendRequest {
                user_id: friend_request_raw.user_id,
                username: friend_request_raw.username,
                display_name: friend_request_raw.display_name,
                description: friend_request_raw.description,
                created: friend_request_raw.created,
                is_terminated: friend_request_raw.is_terminated,
                has_verified_badge: friend_request_raw.has_verified_badge,
                mutual_friends_list: friend_request_raw.mutual_friends_list,
                origin_source_type: friend_request_raw.friend_request.origin_source_type,
                source_universe_id: friend_request_raw.friend_request.source_universe_id,
                sent_at: friend_request_raw.friend_request.sent_at,
            };

            friend_requests.push(friend_request);
        }

        Ok((friend_requests, raw.next_page_cursor))
    }

    /// Get count of pending friend requests using <https://friends.roblox.com/v1/user/friend-requests/count>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// let count_of_friend_requests = client.pending_friend_requests().await?;
    ///
    /// println!("There's a {} pending friend requests!", count_of_friend_requests);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn pending_friend_requests(
        &self,
    ) -> Result<u64, RoboatError> {
        let cookie = self.cookie_string()?;
        let formatted_url = PENDING_FRIEND_REQUESTS;

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .header(header::COOKIE, cookie)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;

        let raw = Self::parse_to_raw::<request_types::PendingFriendRequestsResponse>(response).await?;

        Ok(raw.count)
    }
}
