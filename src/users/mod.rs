use serde::{Deserialize, Serialize};

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

mod internal {
    use super::{UserInformation, USER_DETAILS_API};
    use super::{USERS_SEARCH_API};
    use crate::{Client, Limit, RoboatError};
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
            keyword: &str,
            limit: Limit,
            &self,
        ) -> Result<reqwest_types::UserSearchResponse, RoboatError> {


        let formatted_url = format!(
            "{}?keyword={}&limit={}",
            USERS_SEARCH_API,  keyword, limit
        );

            //todo pass parameters
            let request_result = self
                .reqwest_client
                .get(formatted_url)
                .send()
                .await;

            let response = Self::validate_request_result(request_result).await?;
            let user_search = Self::parse_to_raw::<reqwest_types::UserSearchResponse>(response).await?;
            Ok(user_search)
        }
    }
}
