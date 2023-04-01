use crate::{Client, RoboatError, ROBLOSECURITY_COOKIE_STR};
use reqwest::header;
use serde::{Deserialize, Serialize};

const USER_DETAILS_API: &str = "https://users.roblox.com/v1/users/authenticated";

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

impl Client {
    /// Grabs information about the user from <https://catalog.roblox.com/v1/catalog/items/details> using the
    /// Roblosecurity inside the client.
    ///
    /// This is only for internal use. Use [`Client::user_id`], [`Client::username`], and [`Client::display_name`] instead.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    pub(crate) async fn user_information_internal(&self) -> Result<UserInformation, RoboatError> {
        let roblosecurity = match self.roblosecurity() {
            Some(roblosecurity) => roblosecurity,
            None => return Err(RoboatError::RoblosecurityNotSet),
        };

        let request_result = self
            .reqwest_client
            .get(USER_DETAILS_API)
            .header(
                header::COOKIE,
                format!("{}={}", ROBLOSECURITY_COOKIE_STR, roblosecurity),
            )
            .send()
            .await;

        match request_result {
            Ok(response) => {
                let status_code = response.status().as_u16();

                match status_code {
                    200 => {
                        let user_information: UserInformation = match response.json().await {
                            Ok(x) => x,
                            Err(_) => return Err(RoboatError::MalformedResponse),
                        };

                        // Cache results.
                        *self.user_id.lock().unwrap() = Some(user_information.user_id as u64);
                        *self.username.lock().unwrap() = Some(user_information.username.clone());
                        *self.display_name.lock().unwrap() =
                            Some(user_information.display_name.clone());

                        Ok(user_information)
                    }
                    400 => Err(RoboatError::BadRequest),
                    401 => Err(RoboatError::InvalidRoblosecurity),
                    429 => Err(RoboatError::TooManyRequests),
                    500 => Err(RoboatError::InternalServerError),
                    _ => Err(RoboatError::UnidentifiedStatusCode(status_code)),
                }
            }
            Err(e) => Err(RoboatError::ReqwestError(e)),
        }
    }
}
