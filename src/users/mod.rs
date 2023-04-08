use crate::{Client, RoboatError};
use reqwest::header::{self, HeaderValue};
use serde::{Deserialize, Serialize};

mod request_types;

const USER_DETAILS_API: &str = "https://users.roblox.com/v1/users/authenticated";
const USERS_SEARCH_API: &str = "https://users.roblox.com/v1/users/search";

/// Basic information about the account of the Roblosecurity. Retrieved
/// from <https://users.roblox.com/v1/users/authenticated>.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub(crate) struct ClientUserInformation {
    #[serde(alias = "id")]
    pub user_id: u64,
    #[serde(alias = "name")]
    pub username: String,
    #[serde(alias = "displayName")]
    pub display_name: String,
}

/// The details of a user. Fetched from <https://users.roblox.com/v1/users/search?keyword={keyword}>.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct User {
    pub user_id: u64,
    pub username: String,
    pub display_name: String,
    pub has_verified_badge: bool,
    pub previous_usernames: Vec<String>,
}

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
    ) -> Result<ClientUserInformation, RoboatError> {
        let cookie = self.cookie_string()?;

        let request_result = self
            .reqwest_client
            .get(USER_DETAILS_API)
            .header(header::COOKIE, cookie)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;
        let user_information = Self::parse_to_raw::<ClientUserInformation>(response).await?;

        // Cache results.
        self.set_user_information(user_information.clone()).await;

        Ok(user_information)
    }

    /// Searches for a user using <https://users.roblox.com/v1/users/search>.
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * HOWEVER, if a valid roblosecurity is not provided then there will be a very low rate limit.
    /// * The cursors in this response are not used as using them is currently broken.
    /// * Limits are not used for the same reason (the endpoint does not respect them).
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
    /// const KEYWORD: &str = "linkmon";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// let keyword = "linkmon".to_string();
    /// let users = client.user_search(keyword).await?;
    ///
    /// println!("Found {} users.", users.len());
    ///
    /// for user in users {
    ///     println!("{}: {}", user.username, user.user_id);
    /// }
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn user_search(&self, keyword: String) -> Result<Vec<User>, RoboatError> {
        let formatted_url = format!("{}?keyword={}", USERS_SEARCH_API, keyword);

        let cookie_string = self.cookie_string().unwrap_or(HeaderValue::from_static(""));

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .header(header::COOKIE, cookie_string)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;
        let raw = Self::parse_to_raw::<request_types::UserSearchResponse>(response).await?;

        let mut users = Vec::new();

        for user in raw.data {
            let user_data = User {
                user_id: user.id,
                username: user.name,
                display_name: user.display_name,
                has_verified_badge: user.has_verified_badge,
                previous_usernames: user.previous_usernames,
            };

            users.push(user_data);
        }

        Ok(users)
    }
}
