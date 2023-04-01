//! # About
//! An API wrapper for Roblox.com.
//!
//! This library is designed to be high-performance capable, meaning
//! that a [`Client`] is designed to work with proxies, as well as make
//! multiple requests in parallel. All API calls are made through a [`Client`].
//!
//! # Covered Endpoints
//! * Catalog API
//!    - Item Details - [`Client::item_details`]

#![warn(missing_docs)]

// Re-export reqwest so people can use the correct version.
pub use reqwest;

/// A module for endpoints prefixed with <https://catalog.roblox.com/*>.
pub mod catalog;

use std::sync::Mutex;

const XCSRF_HEADER: &str = "x-csrf-token";

/// The universal error used in this crate.
#[derive(thiserror::Error, Debug, Default)]
pub enum RoboatError {
    /// Used when an endpoint returns status code 429.
    #[default]
    #[error("Too Many Requests")]
    TooManyRequests,
    /// Used when an endpoint returns status code 500.
    #[error("Internal Server Error")]
    InternalServerError,
    /// Used when an endpoint returns status code 400.
    /// This is used when the server cannot process the data sent, whether
    /// it be because it is in the wrong format or it contains too much data.
    #[error("Bad Request")]
    BadRequest,
    /// Used when an endpoint returns status code 401. This can mean that
    /// the roblosecurity is set but that it is either invalid, or
    /// the user does not have authorization to access the endpoint.
    #[error("Invalid Roblosecurity")]
    InvalidRoblosecurity,
    /// Used when no roblosecurity is set, on an endpoint that requires it.
    #[error("Roblosecurity Not Set")]
    RoblosecurityNotSet,
    /// Used for any status codes that do not fit any enum variants of this error.
    /// If you encounter this enum variant, please submit an issue so a variant can be
    /// made or the crate can be fixed.
    #[error("Unidentified Status Code {0}")]
    UnidentifiedStatusCode(u16),
    /// Used when the response from an API endpoint is malformed.
    #[error("Malformed Response")]
    MalformedResponse,
    /// Used when an endpoint rejects a request due to an invalid xcsrf.
    /// Mostly used internally invalid xcsrf is returned due to the fact that rust does not
    /// allow async recursion without making a type signature extremely messy.
    #[error("Invalid Xcsrf. New Xcsrf Contained In Error.")]
    InvalidXcsrf(String),
    /// Used when an endpoint returns a 403 status code, but the response does not contain
    /// a new xcsrf.
    #[error("Missing Xcsrf")]
    XcsrfNotReturned,
    /// Used for any reqwest error that occurs.
    #[error("RequestError {0}")]
    ReqwestError(reqwest::Error),
}

/// A client used for making requests to the Roblox API.
///
/// The client stores the roblosecurity cookie, X-CSRF-TOKEN header, and an HTTPS client to send web
/// requests.
#[derive(Debug, Default)]
pub struct Client {
    /// The cookie used for authentication.
    roblosecurity: Mutex<Option<String>>,
    /// The field holding the value for the X-CSRF-TOKEN header used in and returned by endpoints.
    xcsrf: Mutex<String>,
    /// The user id of the user. Not modifiable by user.
    user_id: Mutex<Option<u64>>,
    /// The username of the user. Not modifiable by user.
    username: Mutex<Option<String>>,
    /// The display name of the user. Not modifiable by user.
    display_name: Mutex<Option<String>>,
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
            xcsrf: Mutex::new(String::new()),
            user_id: Mutex::new(None),
            username: Mutex::new(None),
            display_name: Mutex::new(None),
            reqwest_client,
        }
    }

    /// Returns the cached user id if it exists.
    pub fn cached_user_id(&self) -> Option<u64> {
        *self.user_id.lock().unwrap()
    }

    /// Returns the cached username if it exists.
    pub fn cached_username(&self) -> Option<String> {
        self.username.lock().unwrap().clone()
    }

    /// Returns the cached display name if it exists.
    pub fn cached_display_name(&self) -> Option<String> {
        self.display_name.lock().unwrap().clone()
    }

    /// Sets the Roblosecurity string for the client.
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

    /// Returns a copy of the Roblosecurity stored in the client.
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

    /// Sets the xcsrf token of the client.
    ///
    /// # Example
    ///
    /// ```
    /// use roboat::Client;
    ///
    /// let client = Client::new();
    /// client.set_xcsrf("my_xcsrf".to_string());
    /// ```
    pub fn set_xcsrf(&self, xcsrf: String) {
        *self.xcsrf.lock().unwrap() = xcsrf;
    }

    /// Returns a copy of the xcsrf stored in the client.
    ///
    /// # Example
    ///
    /// ```
    /// use roboat::Client;
    ///
    /// let client = Client::new();
    /// client.set_xcsrf("my_xcsrf".to_string());
    /// let xcsrf = client.xcsrf();
    /// assert_eq!(xcsrf, "my_xcsrf".to_string());
    /// ```
    pub fn xcsrf(&self) -> String {
        self.xcsrf.lock().unwrap().clone()
    }
}
