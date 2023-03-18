//! An api wrapper for Roblox.com.
//!
//! This library is designed to be high-performance capable, meaning
//! that a [`Client`] is designed to work with proxies, as well as make
//! multiple requests in parallel.

#![warn(missing_docs)]

// Re-export reqwest so people can use the correct version.
pub use reqwest;

use std::sync::Mutex;

/// A client used for making requests to the Roblox API.
///
/// The client stores the roblosecurity cookie, X-CSRF-TOKEN header, and an HTTPS client to send web
/// requests.
#[derive(Debug, Default)]
pub struct Client {
    /// The cookie used for authentication.
    roblosecurity: Mutex<Option<String>>,
    /// The header X-CSRF-TOKEN used in and returned by endpoints.
    xcsrf: Mutex<Option<String>>,
    /// A Reqwest HTTP client used to send web requests.
    reqwest_client: reqwest::Client,
}

impl Client {
    /// Used to interface with Roblox.com endpoints.
    ///
    /// Contains any necessary authentication and security tokens, as well as the
    /// reqwest client.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new [`Client`] providing a custom [`reqwest::Client`].
    /// Custom [`reqwest::Client`]s are used for configuring proxies.
    pub fn with_reqwest_client(reqwest_client: reqwest::Client) -> Self {
        Self {
            roblosecurity: Mutex::new(None),
            xcsrf: Mutex::new(None),
            reqwest_client,
        }
    }

    /// Sets the Roblosecurity string for the [`Client`].
    ///
    /// # Example
    ///
    /// ```
    /// use roboat::Client;
    ///
    /// let client = Client::new();
    /// client.set_roblosecurity("my_roblosecurity".to_string());
    /// ```
    pub fn set_roblosecurity(&self, roblosecurity: String) {
        *self.roblosecurity.lock().unwrap() = Some(roblosecurity);
    }

    /// Returns the Roblosecurity stored in the [`Client`].
    ///
    /// # Example
    ///
    /// ```
    /// use roboat::Client;
    ///
    /// let client = Client::new();
    /// client.set_roblosecurity("my_roblosecurity".to_string());
    /// let roblosecurity = client.roblosecurity();
    /// assert_eq!(roblosecurity, "my_roblosecurity".to_string());
    /// ```
    pub fn roblosecurity(&self) -> String {
        self.roblosecurity.lock().unwrap().clone().unwrap()
    }
}
