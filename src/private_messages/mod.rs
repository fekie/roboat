use crate::{Client, RoboatError};
use reqwest::header;
use serde::{Deserialize, Serialize};

mod request_types;

/// Fun fact, pageSize doesn't actually do anything. It's always 20.
const PRIVATE_MESSAGES_API: &str =
    "https://privatemessages.roblox.com/v1/messages?messageTab={message_tab_type}&pageNumber={page_number}&pageSize=20";

/// An enum that corresponds to the different message tabs.
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum MessageTabType {
    #[default]
    Inbox,
    Sent,
    Archive,
}

impl std::fmt::Display for MessageTabType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inbox => write!(f, "inbox"),
            Self::Sent => write!(f, "sent"),
            Self::Archive => write!(f, "archive"),
        }
    }
}

/// A private message. This can be from the user's inbox, sent, or archive tab.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct Message {
    pub message_id: u64,
    pub sender_id: u64,
    pub sender_username: String,
    pub sender_display_name: String,
    pub receiver_id: u64,
    pub receiver_username: String,
    pub receiver_display_name: String,
    pub subject: String,
    pub body: String,
    /// ISO 8601 timestamp of when the message was created.
    pub created: String,
    /// Whether the message has been read.
    pub is_read: bool,
    /// Whether the message is from Roblox.
    pub is_system_message: bool,
}

/// Contains the metadata of the messages. This includes the total amount of messages
/// and the total amount of 20 message pages.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub struct MessagesMetadata {
    /// The total amount of messages in that message tab.
    pub total_message_count: u64,
    /// The total amount of pages in that message tab. The page numbers go from 0..total_pages.
    pub total_pages: u64,
}

impl Client {
    /// Page starts at 0
    ///
    /// Fetches private messages from the specified message tab using <https://privatemessages.roblox.com/v1/messages>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    /// * Returns 20 messages at a time.
    ///
    /// # Argument Notes
    /// * The page starts at 0.
    ///
    /// # Return Value Notes
    /// * The first value in the tuple is a vector of messages.
    /// * The second value in the tuple is the metadata of the messages (total count and page amount).
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    ///
    /// # Example
    /// ```no_run
    /// use roboat::ClientBuilder;
    /// use roboat::private_messages::MessageTabType::Inbox;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// let inbox_type = Inbox;
    ///
    /// let (messages, messages_metadata) = client.messages(0, inbox_type).await?;
    ///
    /// println!("First Message Subject: {}", messages[0].subject);
    /// println!("Total Messages: {}", messages_metadata.total_message_count);
    /// println!("Total Pages: {}", messages_metadata.total_pages);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn messages(
        &self,
        page: u64,
        message_tab_type: MessageTabType,
    ) -> Result<(Vec<Message>, MessagesMetadata), RoboatError> {
        let cookie_string = self.cookie_string()?;

        let url = PRIVATE_MESSAGES_API
            .replace("{message_tab_type}", message_tab_type.to_string().as_str())
            .replace("{page_number}", page.to_string().as_str());

        let request_result = self
            .reqwest_client
            .get(&url)
            .header(header::COOKIE, cookie_string)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;
        let raw = Self::parse_to_raw::<request_types::MessagesResponse>(response).await?;

        let messages = raw
            .collection
            .into_iter()
            .map(|message| Message {
                message_id: message.id as u64,
                sender_id: message.sender.id as u64,
                sender_username: message.sender.name,
                sender_display_name: message.sender.display_name,
                receiver_id: message.recipient.id as u64,
                receiver_username: message.recipient.name,
                receiver_display_name: message.recipient.display_name,
                subject: message.subject,
                body: message.body,
                created: message.created,
                is_read: message.is_read,
                is_system_message: message.is_system_message,
            })
            .collect();

        let metadata = MessagesMetadata {
            total_message_count: raw.total_collection_size as u64,
            total_pages: raw.total_pages as u64,
        };

        Ok((messages, metadata))
    }
}
