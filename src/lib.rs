//! # About
//! A high performance interface for the Roblox API.
//!
//! This library is designed to be high-performance capable, meaning
//! that a [`Client`] is designed to work with proxies, as well as make
//! multiple requests in parallel. All API calls are made through a [`Client`].
//!
//! Extensive documentation is used throughout this crate.
//! All public methods in this crate are documented and have at least one corresponding example.
//!
//! # Covered Endpoints
//! * Catalog API
//!    - Item Details - [`Client::item_details`]
//! * Economy API
//!   - Robux Balance - [`Client::robux`]
//!   - Resellers - [`Client::resellers`]
//!   - User Sales - [`Client::user_sales`]
//!   - Put Limited On Sale - [`Client::put_limited_on_sale`]
//!   - Take Limited Off Sale - [`Client::take_limited_off_sale`]
//!   - Purchase Limited - [`Client::purchase_limited`]
//! * Users API
//!   - User Details - [`Client::user_id`], [`Client::username`], and [`Client::display_name`]
//! (all of them use the same endpoint internally and cache the results)
//!   - User Search - [`Client::user_search`]
//! * Presence API
//!   - Register Presence - [`Client::register_presence`]
//! * Trades API
//!   - Trades List - [`Client::trades`]
//!
//! # Quick Start Examples
//!
//! ## Example 1
//!
//! This code snippet allows you to get your current robux, id, username, and display name.
//!
//! ```no_run
//! // Replace this value with your own roblosecurity token.
//! const ROBLOSECURITY: &str = "your-roblosecurity-token";
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = roboat::ClientBuilder::new()
//!         .roblosecurity(ROBLOSECURITY.to_string())
//!         .build();
//!
//!     let robux = client.robux().await?;
//!     let user_id = client.user_id().await?;
//!     let username = client.username().await?;
//!     let display_name = client.display_name().await?;    
//!
//!     println!("Robux: {}", robux);
//!     println!("User ID: {}", user_id);
//!     println!("Username: {}", username);
//!     println!("Display Name: {}", display_name);
//!
//!     Ok(())   
//! }
//! ```
//!
//! ## Example 2
//!
//! This code snippet allows you to view the lowest price of a limited item by
//! fetching a list of reseller listings.
//!
//! ```no_run
//! // Replace this value with your own roblosecurity token.
//! const ROBLOSECURITY: &str = "your-roblosecurity-token";
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = roboat::ClientBuilder::new()
//!         .roblosecurity(ROBLOSECURITY.to_string())
//!         .build();
//!
//!     let item_id = 1365767;
//!     let limit = roboat::Limit::Ten;
//!     let cursor = None;
//!
//!     let (resellers, _) = client.resellers(item_id, limit, cursor).await?;
//!
//!     println!("Lowest Price for Valkyrie Helm: {}", resellers[0].price);  
//!
//!     Ok(())   
//! }
//! ```
//!
//! ## Example 3
//!
//! This code snippet allows you to get the details of an item.
//!
//! ```no_run
//! use roboat::catalog::avatar_catalog::{ItemArgs, ItemType};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = roboat::ClientBuilder::new().build();
//!
//!     let item = ItemArgs {
//!         item_type: ItemType::Asset,
//!         id: 1365767,
//!     };
//!
//!     let details = &client.item_details(vec![item]).await?[0];
//!
//!     let name = &details.name;
//!     let description = &details.description;
//!     let creator_name = &details.creator_name;
//!     let price = details.price.unwrap_or(0);
//!
//!     println!("Name: {}", name);
//!     println!("Description: {}", description);
//!     println!("Creator Name: {}", creator_name);
//!     println!("Price: {}", price);
//!
//!     Ok(())   
//! }
//! ```

#![warn(missing_docs)]

// Re-export reqwest so people can use the correct version.
pub use reqwest;

pub use client::{Client, ClientBuilder};
pub use economy::PurchaseLimitedError;

/// A module for endpoints prefixed with <https://catalog.roblox.com/*>.
pub mod catalog;
mod client;
/// A module for endpoints prefixed with <https://economy.roblox.com/*>.
pub mod economy;
/// A module for endpoints prefixed with <https://presence.roblox.com/*>.
mod presence;
/// A module for endpoints prefixed with <https://trades.roblox.com/*>.
pub mod trades;
/// A module for endpoints prefixed with <https://users.roblox.com/*>.
pub mod users;
mod validation;

// todo: add manual xcsrf refresh
// todo: endpoints that require premium/robux to test: recent trades, send trade, buy limited item, buy non-limited item
// todo: inventory api, groups api, follow api
// todo: add usage to readme
// todo: every type should have an explanation of the typical means by which the user will construct or fetch it, if the answer isn't “this is a struct literal with public methods”.
// todo: figure out authtickets
// todo: add ugc limited buying
// todo: make feature that allows reqwest crate to not collide.
// todo: hide reqwest types
// todo: rename reqwest_types.rs to request_types.rs
// todo: list what errors can be returned by each method

use serde::{Deserialize, Serialize};

// Used in reqwest header keys.
const XCSRF_HEADER: &str = "x-csrf-token";
// The user agent used for fussy endpoints.
const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:101.0) Gecko/20100101 Firefox/101.0";
// The content type used for fussy endpoints.
const CONTENT_TYPE: &str = "application/json;charset=utf-8";

/// The maximum amount of instances to return from an endpoint. Used as a parameter in various methods that call
/// endpoints. This is an enum instead of an integer as these are the only values that are accepted by Roblox
/// for the limit parameter.
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

/// The universal error used in this crate. Encapsulates any sub-errors used in this crate.
#[non_exhaustive]
#[derive(thiserror::Error, Debug, Default)]
pub enum RoboatError {
    /// Used when an endpoint returns status code 429.
    #[default]
    #[error("Too Many Requests")]
    TooManyRequests,
    /// Used when an endpoint returns status code 500.
    #[error("Internal Server Error")]
    InternalServerError,
    /// Used when an endpoint returns status code 400 and does not embed an error.
    /// This is used when the server cannot process the data sent, whether
    /// it be because it is in the wrong format or it contains too much data.
    #[error("Bad Request")]
    BadRequest,
    /// Returned when the user does not have a valid roblosecurity, or
    /// does not have authorization to access the endpoint.
    ///
    /// This is also used as the backup error when an endpoint returns a 401 status code
    /// but the error cannot be parsed from the response.
    ///
    /// Roblox error code 0.
    #[error("Invalid Roblosecurity")]
    InvalidRoblosecurity,
    /// Returned when the endpoint returns a 401 status code, but the error response
    /// contains an unknown Roblox error code.
    #[error("Unknown Roblox Error Code {code}: {message}")]
    UnknownRobloxErrorCode {
        /// The error code (not status code) returned by roblox.
        code: u16,
        /// The error message returned by roblox.
        message: String,
    },
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
    /// Custom Roblox errors sometimes thrown when the user calls [`Client::purchase_limited`].
    #[error("{0}")]
    PurchaseLimitedError(PurchaseLimitedError),
    /// Used for any reqwest error that occurs.
    #[error("RequestError {0}")]
    ReqwestError(reqwest::Error),
}
