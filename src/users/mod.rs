use serde::{Deserialize, Serialize};

const USER_DETAILS_API: &str = "https://users.roblox.com/v1/users/authenticated";
const USERS_SEARCH_API: &str = "https://users.roblox.com/v1/users/search"

/// Basic information about the account of the Roblosecurity. Retrieved
/// from <https://users.roblox.com/v1/users/authenticated>.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub(crate) struct UserInformation {
    #[serde(rename(deserialize = "id"))]
    #[serde(rename(deserialize = "user_id"))]
    pub user_id: u64,
    #[serde(rename(deserialize = "name"))]
    #[serde(rename(deserialize = "username"))]
    pub username: String,
    #[serde(rename(deserialize = "displayName"))]
    #[serde(rename(deserialize = "display_name"))]
    pub display_name: String,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub(crate) struct UserSearchUserInformation {
    #[serde(rename(deserialize = "id"))]
    #[serde(rename(deserialize = "user_id"))]
    pub user_id: u64,
    #[serde(rename(deserialize = "name"))]
    #[serde(rename(deserialize = "username"))]
    pub username: String,
    #[serde(rename(deserialize = "hasVerifiedBadge"))]
    #[serde(rename(deserialize = "has_verified_badge"))]
    pub has_verified_badge: bool,
    #[serde(rename(deserialize = "previousUsernames"))]
    #[serde(rename(deserialize = "previous_usernames"))]
    pub previous_usernames: Vec<String>,
    #[serde(rename(deserialize = "displayName"))]
    #[serde(rename(deserialize = "display_name"))]
    pub display_name: String,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub (crate) struct UserSearch {
    #[serde(rename(deserialize = "previousPageCursor"))]
    #[serde(rename(deserialize = "previous_page_cursor"))]
    pub previous_page_cursor: String,
    #[serde(rename(deserialize = "nextPageCursor"))]
    #[serde(rename(deserialize = "next_page_cursor"))]
    pub next_page_cursor: String,
    #[serde(rename(deserialize = "data"))]
    #[serde(rename(deserialize = "data"))]
    pub data: Vec<UserSearchUserInformation>,
}

mod internal {
    use super::{UserInformation, USER_DETAILS_API};
    use super::{UserSearch, USERS_SEARCH_API};
    use crate::{Client, RoboatError};
    use reqwest::header;

    impl Client {
        /// Grabs information about the user from <https://catalog.roblox.com/v1/catalog/items/details> using the
        /// Roblosecurity inside the client.
        ///
        /// This is only for internal use. Use [`Client::user_id`], [`Client::username`], and [`Client::display_name`] instead.
        ///
        /// # Notes
        /// * Requires a valid roblosecurity.
        pub(crate) async fn user_information_internal(
            &self,
        ) -> Result<UserInformation, RoboatError> {
            let cookie = self.create_cookie_string()?;

            let request_result = self
                .reqwest_client
                .get(USER_DETAILS_API)
                .header(header::COOKIE, cookie)
                .send()
                .await;

            let response = Self::validate_request_result(request_result).await?;
            let user_information = Self::parse_to_raw::<UserInformation>(response).await?;

            // Cache results.
            *self.user_id.lock().unwrap() = Some(user_information.user_id as u64);
            *self.username.lock().unwrap() = Some(user_information.username.clone());
            *self.display_name.lock().unwrap() = Some(user_information.display_name.clone());

            Ok(user_information)
        }

        pub(crate) async fn users_search_internal(
            &self,
        ) -> Result<UserSearch, RoboatError> {

            //todo pass parameters
            let request_result = self
                .reqwest_client
                .get(USERS_SEARCH_API)
                .send()
                .await;

            let response = Self::validate_request_result(request_result).await?;
            let user_search = Self::parse_to_raw::<UserSearch>(response).await?;

            // Cache results.
            *self.previous_page_cursor.lock().unwrap() = Some(user_search.previous_page_cursor.clone());
            *self.next_page_cursor.lock().unwrap() = Some(user_search.next_page_cursor.clone());
            *self.data.lock().unwrap() = Some(user_search.data.clone());

            Ok(user_search)
        }
    }
}
