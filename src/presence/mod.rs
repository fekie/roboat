use crate::{presence::request_types::UserPresenceResponse, Client, RoboatError};
use serde::{Deserialize, Serialize};
mod request_types;

const REGISTER_PRESENCE_API: &str = "https://presence.roblox.com/v1/presence/register-app-presence";
const USER_PRESENCE_API: &str = "https://presence.roblox.com/v1/presence/users";

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

impl TryFrom<u8> for PresenceType {
    type Error = RoboatError;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Offline),
            1 => Ok(Self::Online),
            2 => Ok(Self::InGame),
            3 => Ok(Self::InStudio),
            4 => Ok(Self::Invisible),
            _ => Err(RoboatError::MalformedResponse),
        }
    }
}

impl Client {
    /// Registers presence on the website (makes you appear to be online). Endpoint called is
    /// <https://presence.roblox.com/v1/presence/register-app-presence>
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    /// * Will repeat once if the x-csrf-token is invalid.
    /// * Normally repeats every 15 seconds when viewing the Roblox homepage.
    ///
    /// # Return Value Notes
    /// * Will return `Ok(())` if presence was successfully registered.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    /// * All errors under [X-CSRF-TOKEN Required Errors](#x-csrf-token-required-errors).
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
    /// match client.register_presence().await {
    ///    Ok(_) => println!("Successfully registered presence!"),
    ///    Err(e) => println!("Error: {}", e),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn register_presence(&self) -> Result<(), RoboatError> {
        match self.register_presence_internal().await {
            Ok(x) => Ok(x),
            Err(e) => match e {
                RoboatError::InvalidXcsrf(new_xcsrf) => {
                    self.set_xcsrf(new_xcsrf).await;

                    self.register_presence_internal().await
                }
                _ => Err(e),
            },
        }
    }

    /// Fetch presences of users on roblox like (Offline, Online, In Game, Last Location). Endpoint called is
    /// <https://presence.roblox.com/v1/presence/users>
    ///
    /// # Notes
    /// * valid roblosecurity is optional for more info about the game the user is in
    /// * If user is in game and either their joins are turned off or you don't have a valid cookie information like place_id will always be None
    /// * Can handle up to 50 users at once
    /// * Doesnt need xcsrf token.
    ///
    /// # Return Value Notes
    /// * Will return `Ok(UserPresenceResponse)` if presence was successfully fetched.
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
    /// let users = vec![1, 35958674918];
    /// match client.fetch_user_presence(users).await {
    ///    Ok(user_statuses) => println!("Successfully registered presence: {:?}", user_statuses),
    ///    Err(e) => println!("Error: {}", e),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_users_presence(
        &self,
        users: Vec<u64>,
    ) -> Result<UserPresenceResponse, RoboatError> {
        match self.fetch_users_presence_internal(users).await {
            Ok(x) => Ok(x),
            Err(e) => Err(e),
        }
    }
}

mod internal {
    use super::REGISTER_PRESENCE_API;
    use crate::{
        presence::{request_types::UserPresenceResponse, USER_PRESENCE_API},
        Client, RoboatError, XCSRF_HEADER,
    };
    use reqwest::header;

    impl Client {
        pub(super) async fn register_presence_internal(&self) -> Result<(), RoboatError> {
            let cookie = self.cookie_string()?;

            let json = serde_json::json!({
                "location": "Home",
            });

            let request_result = self
                .reqwest_client
                .post(REGISTER_PRESENCE_API)
                .header(header::COOKIE, cookie)
                .header(XCSRF_HEADER, self.xcsrf().await)
                .json(&json)
                .send()
                .await;

            let _ = Self::validate_request_result(request_result).await?;

            // We don't care about the response, just that it's a status code 200.
            Ok(())
        }

        pub(super) async fn fetch_users_presence_internal(
            &self,
            users: Vec<u64>,
        ) -> Result<UserPresenceResponse, RoboatError> {
            let json = serde_json::json!({
                "userIds": users,
            });

            // NOTE: Cookie is optional here
            let request = self
                .cookie_string()
                .map(|cookie| {
                    self.reqwest_client
                        .post(USER_PRESENCE_API)
                        .json(&json)
                        .header(header::COOKIE, cookie)
                })
                .unwrap_or_else(|_| self.reqwest_client.post(USER_PRESENCE_API).json(&json));

            let request_result = request.send().await;

            let response = Self::validate_request_result(request_result).await?;
            let presense_json = Self::parse_to_raw::<UserPresenceResponse>(response).await?;
            Ok(presense_json)
        }
    }
}
