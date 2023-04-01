use crate::{Client, RoboatError, ROBLOSECURITY_COOKIE_STR};
use reqwest::header;

mod reqwest_types;

const ROBUX_API_PART_1: &str = "https://economy.roblox.com/v1/users/";
const ROBUX_API_PART_2: &str = "/currency";

impl Client {
    /// Grabs robux count of the current account from <https://economy.roblox.com/v1/users/{user_id}/currency>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    ///
    /// # Example
    /// ```no_run
    /// use roboat::catalog::avatar_catalog::ItemArgs;
    /// use roboat::catalog::avatar_catalog::ItemType;
    /// use roboat::Client;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new();
    /// client.set_roblosecurity("my_roblosecurity".to_string());
    ///
    /// let robux = client.robux_balance().await?;
    /// println!("Robux: {}", robux);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn robux_balance(&self) -> Result<u64, RoboatError> {
        let user_id = self.user_id().await?;
        let formatted_url = format!("{}{}{}", ROBUX_API_PART_1, user_id, ROBUX_API_PART_2);
        let roblosecurity = match self.roblosecurity() {
            Some(roblosecurity) => roblosecurity,
            None => return Err(RoboatError::RoblosecurityNotSet),
        };
        let cookie = format!("{}={}", ROBLOSECURITY_COOKIE_STR, roblosecurity);

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .header(header::COOKIE, cookie)
            .send()
            .await;

        match request_result {
            Ok(response) => {
                let status_code = response.status().as_u16();

                match status_code {
                    200 => {
                        let raw = match response.json::<reqwest_types::CurrencyResponse>().await {
                            Ok(x) => x,
                            Err(_) => return Err(RoboatError::MalformedResponse),
                        };
                        let robux = raw.robux;
                        Ok(robux)
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
