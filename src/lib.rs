//! # About
//! An api wrapper for Roblox.com.
//!
//! This library is designed to be high-performance capable, meaning
//! that a [`Client`] is designed to work with proxies, as well as make
//! multiple requests in parallel. All API calls are made through a [`Client`].
//!
//! # Implementation Details
//! All modules corresponding to groups of endpoints are structured
//! to where they consist of an "internal" and "external" module.
//! The "external" modules contain public facing client methods, which
//! call methods from "internal" modules which make the actual endpoint calls.
//! This is done as some endpoints are designed to possibly have more than one call made.
//! However, Rust does not allow async recursion by default. One possible solution, using #[async_recursion],
//! complicates the type signature of methods, requiring much repitive documentation to explain
//! parameters and return values that would have been self-explanatory otherwise.

#![warn(missing_docs)]

// Re-export reqwest so people can use the correct version.
pub use reqwest;

/// A module for endpoints prefixed with https://catalog.roblox.com/*
pub mod catalog;
/// A module for endpoints prefixed with https://economy.roblox.com/*
pub mod economy;

use std::sync::Mutex;

/// The universal error used in this crate.
#[derive(thiserror::Error, Debug, Default)]
pub enum Error {
    /// Used when an endpoint returns status code 429.
    #[default]
    #[error("Too Many Requests")]
    TooManyRequests,
    /// Used when an endpoint returns status code 500.
    #[error("Internal Server Error")]
    InternalServerError,
    /// Used for any status codes that do not fit any enum variants of this error.
    /// If you encounter this enum variant, please submit an issue so a variant can be
    /// made or the crate can be fixed.
    #[error("Unidentified Status Code {0}")]
    UnidentifiedStatusCode(u16),
    /// Used when the response from an API endpoint is malformed.
    #[error("Malformed Response")]
    MalformedResponse,
    /// Used when an endpoint rejects a request due to an invalid xcsrf.
    /// An invalid xcsrf is returned due to the fact that rust does not
    /// allow async recursion without making a type signature extremely messy.
    #[error("Invalid Xcsrf. New Xcsrf Contained In Error.")]
    InvalidXcsrf(String),
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
            reqwest_client,
        }
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
