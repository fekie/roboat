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
//! # Coverage
//! * Auth API
//!   - Force Refresh Xcsrf - [`Client::force_refresh_xcsrf`]
//! * BEDEV2 API
//!   - Fetch Non-Tradable Limited Details - [`Client::non_tradable_limited_details`]
//!   - Fetch Collectible Product ID - [`Client::collectible_product_id`]
//!   - Fetch Collectible Product ID Bulk - [`Client::collectible_product_id_bulk`]
//!   - Fetch Collectible Creator ID - [`Client::collectible_creator_id`]
//!   - Purchase Non-Tradable Limited - [`Client::purchase_non_tradable_limited`]
//! * Catalog API
//!   - Fetch Item Details - [`Client::item_details`]
//!   - Fetch Product ID - [`Client::product_id`]  
//!   - Fetch Product ID Bulk - [`Client::product_id_bulk`]
//!   - Fetch Collectible Item ID - [`Client::collectible_item_id`]
//!   - Fetch Collectible Item ID Bulk - [`Client::collectible_item_id_bulk`]
//!   - Avatar Catalog Search - [`Client::avatar_catalog_search`]
//! * Chat API
//!   - Fetch Unread Conversation Count - [`Client::unread_conversation_count`]
//! * Economy API
//!   - Fetch Robux Balance - [`Client::robux`]
//!   - Fetch Resellers - [`Client::resellers`]
//!   - Fetch User Sales - [`Client::user_sales`]
//!   - Put Limited On Sale - [`Client::put_limited_on_sale`]
//!   - Take Limited Off Sale - [`Client::take_limited_off_sale`]
//!   - Purchase Tradable Limited - [`Client::purchase_tradable_limited`]
//! * Group API
//!   - Fetch Group Roles - [`Client::group_roles`]
//!   - Fetch Group Role Members - [`Client::group_role_members`]
//!   - Set Group Member Role - [`Client::set_group_member_role`]
//! * Presence API
//!   - Register Presence - [`Client::register_presence`]
//!   - Fetch Users Presence - [`Client::fetch_users_presence`]
//! * Private Messages API
//!   - Fetch Messages - [`Client::messages`]
//! * Thumbnails API
//!   - Fetch Thumbnail Url Bulk - [`Client::thumbnail_url_bulk`]
//!   - Fetch Thumbnail Url - [`Client::thumbnail_url`]
//! * Trades API
//!   - Accept Trade - [`Client::accept_trade`]
//!   - Decline Trade - [`Client::decline_trade`]
//!   - Send Trade - [`Client::send_trade`]
//!   - Fetch Trade Details - [`Client::trade_details`]
//!   - Fetch Trades List - [`Client::trades`]
//!   - Fetch Trade Count - [`Client::trade_count`]
//! * Users API
//!   - Fetch User ID - [`Client::user_id`]
//!   - Fetch Username - [`Client::username`]
//!   - Fetch Display Name - [`Client::display_name`]
//!   - User Search - [`Client::user_search`]
//!   - Username User Details - [`Client::username_user_details`]
//!   - Fetch User Details - [`Client::user_details`]
//! * Friends API
//!   - Fetch Count of Pending Friend Requests - [`Client::pending_friend_requests`]
//!   - Fetch Friend Requests - [`Client::friend_requests`]
//!   - Fetch Friends List - [`Client::friends_list`]
//!   - Accept Friend Request - [`Client::accept_friend_request`]
//!   - Decline Friend Request - [`Client::decline_friend_request`]
//!   - Send Friend Request - [`Client::send_friend_request`]
//!   - Unfriend - [`Client::unfriend`]
//! * Assetdelivery API
//!   - Fetch Asset Data - [`Client::fetch_asset_data`]
//! * IDE API (Animations)
//!   - Upload New Animation - [`Client::upload_new_animation`]
//! * UNDER CONSTRUCTION
//!   - Upload Classic Clothing To Group - [`Client::upload_classic_clothing_to_group`]
//!
//! # Quick Start Examples
//!
//! ## Example 1 - Purchase Free UGC Limited
//! This code snippet allows you to purchase a free ugc limited.
//!
//! It can be modified to purchase a non-free ugc limited by changing the price.
//!
//! ```no_run
//! // Replace this value with your own roblosecurity token.
//! const ROBLOSECURITY: &str = "your-roblosecurity-token";
//! // Replace this value with the item id of the item you want to purchase.
//! const ITEM_ID: u64 = 13119979433;
//! // Replace this value if you want to purchase a non-free item.
//! const PRICE: u64 = 0;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = roboat::ClientBuilder::new()
//!         .roblosecurity(ROBLOSECURITY.to_string())
//!         .build();
//!
//!     let collectible_item_id = client.collectible_item_id(ITEM_ID).await?;
//!
//!     let collectible_product_id = client
//!         .collectible_product_id(collectible_item_id.clone())
//!         .await?;
//!
//!     let collectible_creator_id = client
//!         .collectible_creator_id(collectible_item_id.clone())
//!         .await?;
//!
//!     client
//!         .purchase_non_tradable_limited(
//!             collectible_item_id,
//!             collectible_product_id,
//!             collectible_creator_id,
//!             PRICE,
//!         )
//!         .await?;
//!
//!     println!("Purchased item {} for {} robux!", ITEM_ID, PRICE);
//!
//!     Ok(())   
//! }
//! ```
//!
//! ## Example 2 - Fetch User Info
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
//! ## Example 3 - Fetch Price of Tradable Limited
//!
//! This code snippet allows you to view the lowest price of a tradable limited item by
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
//! ## Example 4 - Fetch Item Details
//!
//! This code snippet allows you to get the details of an item.
//!
//! ```no_run
//! use roboat::catalog::{Item, ItemType};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = roboat::ClientBuilder::new().build();
//!
//!     let item = Item {
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
use serde::{Deserialize, Serialize};

pub use bedev2::PurchaseNonTradableLimitedError;
pub use client::{Client, ClientBuilder};
pub use economy::PurchaseTradableLimitedError;

///
/// A module for endpoints prefixed with <https://assetdelivery.roblox.com/*>
pub mod assetdelivery;
/// A module for endpoints prefixed with <https://auth.roblox.com/*>.
mod auth;
/// A module for endpoints prefixed with <https://apis.roblox.com/*>.
pub mod bedev2;
/// A module for endpoints prefixed with <https://catalog.roblox.com/*>.
pub mod catalog;
/// A module for endpoints prefixed with <https://chat.roblox.com/*>.
mod chat;
/// A module related to the [`Client`] struct.
mod client;
/// A module for endpoints prefixed with <https://economy.roblox.com/*>.
pub mod economy;
/// A module for endpoints prefixed with <https://friends.roblox.com/*>.
pub mod friends;
/// A module for endpoints prefixed with <https://groups.roblox.com/*>.
pub mod groups;

/// A module for endpoints prefixed with <https://games.roblox.com/*>
pub mod games;

/// A module for endpoints prefixed with <https://www.roblox.com/ide/*>
// This is used for private APIs like ide/uploadnewanimation and ide/places/createV2
pub mod ide;
/// A module for endpoints prefixed with <https://presence.roblox.com/*>.
pub mod presence;
/// A module for endpoints prefixed with <https://privatemessages.roblox.com/*>.
pub mod private_messages;
/// A module for endpoints prefixed with <https://thumbnails.roblox.com/*>.
pub mod thumbnails;
/// A module for endpoints prefixed with <https://trades.roblox.com/*>.
pub mod trades;
/// A module for endpoints prefixed with <https://users.roblox.com/*>.
pub mod users;
/// A module related to validating requests.
mod validation;
// todo: figure out authtickets
// todo: maybe respect cookies returned
// todo: maybe add stronger types for stuff like cursors? stuff that can be returned basically and is unlikely to cbe created by the user.
// todo: add doc example and example count somewhere
// todo: the roblox api docs show the roblox error codes, maybe a custom sub error can be made
// todo: add a "2 step not implemented for this endpoint" error

// Used in request header keys.
const XCSRF_HEADER: &str = "x-csrf-token";
// The user agent used for fussy endpoints.
const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:101.0) Gecko/20100101 Firefox/101.0";
// The content type used for fussy endpoints.
const CONTENT_TYPE: &str = "application/json;charset=utf-8";

/// The maximum amount of instances to return from an endpoint. Used as a parameter in various methods that call
/// endpoints.
///
/// This is an enum instead of an integer as these are usually the only values that are accepted by Roblox
/// for the limit parameter.
///
/// This is the most common limit used on Roblox endpoints. However, not all endpoints use this limit.
/// Some alternative limits are as follows:
/// * [`catalog::CatalogQueryLimit`]
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
    #[error("Malformed Response. If this occurs often it may be a bug. Please report it to the issues page."
    )]
    MalformedResponse,
    /// Used when an endpoint rejects a request due to an invalid xcsrf.
    /// Mostly used internally invalid xcsrf is returned due to the fact that rust does not
    /// allow async recursion without making a type signature extremely messy.
    #[error("Invalid Xcsrf. New Xcsrf Contained In Error.")]
    InvalidXcsrf(String),
    /// Used when an endpoint returns a 403 status code, doesn't need a challenge, but the response does not contain
    /// a new xcsrf.
    #[error("Missing Xcsrf")]
    XcsrfNotReturned,
    /// Used when an endpoint returns a 403 status code, but not because of an invalid xcsrf.
    /// The string inside this error variant is a challenge id, which can be used to complete the challenge
    /// (which can be either a captcha or a two step verification code).
    #[error("Challenge Required. A captcha or two step authentication must be completed using challenge id {0}."
    )]
    ChallengeRequired(String),
    /// Used when an endpoint returns a 403 status code, can be parsed into a roblox error,
    /// but the error message is incorrect or the challenge id is not returned. This also means that no xcsrf was returned.
    #[error("Unknown Status Code 403 Format. If this occurs often it may be a bug. Please report it to the issues page."
    )]
    UnknownStatus403Format,
    /// Custom Roblox errors sometimes thrown when the user calls [`Client::purchase_tradable_limited`].
    #[error("{0}")]
    PurchaseTradableLimitedError(PurchaseTradableLimitedError),
    /// Custom Roblox errors sometimes thrown when the user calls [`Client::purchase_non_tradable_limited`].
    #[error("{0}")]
    PurchaseNonTradableLimitedError(PurchaseNonTradableLimitedError),
    /// Used for any reqwest error that occurs.
    #[error("RequestError {0}")]
    ReqwestError(reqwest::Error),
    /// Used when an io error occurs.
    #[error("IoError {0}")]
    IoError(#[from] std::io::Error),
    /// Used when a file system path passed to a method is invalid.
    #[error("Invalid Path {0}")]
    InvalidPath(String),
}

/// The type of the challenge required to complete a request.
/// This can be either a captcha or a two step verification code (can be an authenticator or an email).
#[non_exhaustive]
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub enum ChallengeType {
    #[default]
    TwoStep,
}

impl TryFrom<String> for ChallengeType {
    type Error = RoboatError;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        match raw.as_str() {
            "twostepverification" => Ok(ChallengeType::TwoStep),
            _ => Err(RoboatError::MalformedResponse),
        }
    }
}

/// The challenge info returned by Roblox when a challenge is required to complete a request.
/// This challenge can be either a two step verification code or a captcha. This is specified by the `challenge_type` field.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct ChallengeInfo {
    /// The string in the returned `rblx-challenge-id` header.
    pub challenge_id: String,
    /// The string in the returned `rblx-challenge-metadata` header.
    ///
    /// This is encoded in base64 and can be decoded using the [`base64`] crate.
    pub challenge_metadata: String,
    /// The type of challenge parsed from the `rblx-challenge-type` header.
    pub challenge_type: ChallengeType,
}

/// The universal struct for a Roblox user in this crate.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct User {
    pub user_id: u64,
    pub username: String,
    pub display_name: String,
}
