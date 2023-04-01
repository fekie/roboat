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
//! * Economy API
//!   - Robux Balance - [`Client::robux`]
//!   - Resellers - [`Client::resellers`]
//! * Users API
//!   - User Details - [`Client::user_id`], [`Client::username`], and [`Client::display_name`]
//! (all of them use the same endpoint internally and cache the results)

#![warn(missing_docs)]

// Re-export reqwest so people can use the correct version.
pub use reqwest;

pub use client::Client;

/// A module for endpoints prefixed with <https://catalog.roblox.com/*>.
pub mod catalog;
mod client;
/// A module for endpoints prefixed with <https://economy.roblox.com/*>.
pub mod economy;
/// A module for endpoints prefixed with <https://users.roblox.com/*>.
pub mod users;

use serde::{Deserialize, Serialize};

// Used in reqwest header keys.
const XCSRF_HEADER: &str = "x-csrf-token";
// Used in the cookie header.
const ROBLOSECURITY_COOKIE_STR: &str = ".ROBLOSECURITY";

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

/// The maximum amount of instances to return from an endpoint.
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum Limit {
    #[default]
    Ten,
    TwentyFive,
    Fifty,
    Hundred,
}

impl Limit {
    fn to_u64(self) -> u64 {
        match self {
            Limit::Ten => 10,
            Limit::TwentyFive => 25,
            Limit::Fifty => 50,
            Limit::Hundred => 100,
        }
    }
}
