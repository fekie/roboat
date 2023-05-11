use crate::{Client, RoboatError};
use reqwest::header;

mod request_types;

const UNREAD_CONVERSATION_COUNT_API: &str =
    "https://chat.roblox.com/v2/get-unread-conversation-count";

impl Client {
    /// Fetches the number of unread chats/conversations using <https://chat.roblox.com/v2/get-unread-conversation-count>.
    /// Keep in mind that these are not the same as "messages".
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    ///
    /// # Example
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// let count = client.unread_conversation_count().await?;
    ///
    /// println!("Unread message count: {}", count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn unread_conversation_count(&self) -> Result<u64, RoboatError> {
        let cookie_string = self.cookie_string()?;

        let request_result = self
            .reqwest_client
            .get(UNREAD_CONVERSATION_COUNT_API)
            .header(header::COOKIE, cookie_string)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;
        let raw = Self::parse_to_raw::<request_types::UnreadMessageCountResponse>(response).await?;

        Ok(raw.count)
    }
}
