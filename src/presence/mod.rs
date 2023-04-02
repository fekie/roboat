use crate::{Client, RoboatError};

const REGISTER_PRESENCE_API: &str = "https://presence.roblox.com/v1/presence/register-app-presence";

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
    /// # Example
    /// ```no_run
    /// use roboat::Client;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_roblosecurity("roblosecurity".to_string());
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
                    self.set_xcsrf(new_xcsrf);

                    self.register_presence_internal().await
                }
                _ => Err(e),
            },
        }
    }
}

mod internal {
    use super::REGISTER_PRESENCE_API;
    use crate::{Client, RoboatError, XCSRF_HEADER};
    use reqwest::header;

    impl Client {
        pub(super) async fn register_presence_internal(&self) -> Result<(), RoboatError> {
            let cookie = self.create_cookie_string()?;

            let json = serde_json::json!({
                "location": "Home",
            });

            let request_result = self
                .reqwest_client
                .post(REGISTER_PRESENCE_API)
                .header(header::COOKIE, cookie)
                .header(XCSRF_HEADER, self.xcsrf())
                .json(&json)
                .send()
                .await;

            let _ = Self::validate_request_result(request_result).await?;

            // We don't care about the response, just that it's a status code 200.
            Ok(())
        }
    }
}
