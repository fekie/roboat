use crate::{Client, Limit, RoboatError};
use reqwest::header;
use serde::{Deserialize, Serialize};

mod reqwest_types;

const USER_DETAILS_API: &str = "https://users.roblox.com/v1/users/authenticated";
const USERS_SEARCH_API: &str = "https://users.roblox.com/v1/users/search";

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
pub struct User {
    pub user_id: u64,
    pub username: String,
    pub display_name: String,
    pub has_verified_badge: bool,
    pub previous_usernames: Vec<String>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct UserList {
    pub previous_page_cursor: String,
    pub next_page_cursor: String,
    pub data: Vec<User>,
}

impl Client {
    /// Grabs information about the user from <https://catalog.roblox.com/v1/catalog/items/details> using the
    /// Roblosecurity inside the client.
    ///
    /// This is only for internal use. Use [`Client::user_id`], [`Client::username`], and [`Client::display_name`] instead.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    pub(crate) async fn user_information_internal(&self) -> Result<UserInformation, RoboatError> {
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

    // todo: make external example
    // todo: make it use roblosecurity if available
    // todo: write docs with doc example
    // todo: note the previous todos are for this one shark guy and should be resolved within a couple of days (or ill handle it)
    pub async fn user_search(
        &self,
        keyword: String,
        limit: Limit,
        cursor: Option<String>,
    ) -> Result<UserList, RoboatError> {
        let limit = limit.to_u64();
        let cursor = cursor.unwrap_or_default();

        let formatted_url = format!(
            "{}?keyword={}&limit={}&cursor={}",
            USERS_SEARCH_API, keyword, limit, cursor
        );

        let request_result = self.reqwest_client.get(formatted_url).send().await;

        let response = Self::validate_request_result(request_result).await?;

        let raw = Self::parse_to_raw::<reqwest_types::UserSearchResponse>(response).await?;

        let mut users = Vec::new();

        for user in raw.data {
            let user_data = User {
                user_id: user.user_id,
                username: user.username,
                display_name: user.display_name,
                has_verified_badge: user.has_verified_badge,
                previous_usernames: user.previous_usernames,
            };

            users.push(user_data);
        }

        let result = UserList {
            previous_page_cursor: raw.previous_page_cursor,
            next_page_cursor: raw.next_page_cursor,
            data: users,
        };

        Ok(result)
    }
}
