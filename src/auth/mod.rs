use crate::{Client, RoboatError, XCSRF_HEADER};
use reqwest::header;

const AUTH_BASE_URL: &str = "https://auth.roblox.com/";

impl Client {
    /// Used to force refresh the xcsrf. This does not invalid the current xcsrf, it just
    /// makes sure that the current xcsrf is valid and adds a new one if it is not.
    ///
    /// Uses the endpoint <https://auth.roblox.com/>
    ///
    /// # Notes
    /// * Works with or without a Roblosecurity. If a Roblosecurity is added, it will be used.
    ///
    /// # Return Value Notes
    /// * Will return `Ok(())` if everything was successful.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    ///
    /// # Examples
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let _ = client.force_refresh_xcsrf().await?;
    /// println!("Successfully Refreshed Xcsrf!");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn force_refresh_xcsrf(&self) -> Result<(), RoboatError> {
        let builder = self
            .reqwest_client
            .post(AUTH_BASE_URL)
            .header(XCSRF_HEADER, self.xcsrf().await);

        // Add the roblosecurity if it exists.
        let builder = match self.cookie_string() {
            Ok(cookie_string) => builder.header(header::COOKIE, cookie_string),
            Err(_) => builder,
        };

        let request_result = builder.send().await;

        // We want to take the xcsrf from here.
        match Self::validate_request_result(request_result).await {
            // This just means the xcsrf is valid.
            Ok(_) => Ok(()),
            Err(e) => match e {
                RoboatError::InvalidXcsrf(xcsrf) => {
                    self.set_xcsrf(xcsrf).await;
                    Ok(())
                }
                _ => Err(e),
            },
        }
    }
}
