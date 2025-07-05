use crate::bedev2::request_types::AssetInfo;
use crate::catalog::CreatorType;
use crate::{Client, RoboatError};
use serde::{Deserialize, Serialize};

mod request_types;

const COLLECTIBLE_ITEM_DETAILS_API: &str =
    "https://apis.roblox.com/marketplace-items/v1/items/details";

const PURCHASE_NON_TRADEABLE_LIMITED_API_PART_1: &str =
    "https://apis.roblox.com/marketplace-sales/v1/item/";

/// This API endpoint supports two operations:
/// - POST request with a payload to upload an asset
/// - GET request with an asset ID parameter (/{Id}) to retrieve asset information
const ASSET_API: &str = "https://apis.roblox.com/assets/user-auth/v1/assets";

const PURCHASE_NON_TRADEABLE_LIMITED_API_PART_2: &str = "/purchase-item";

/// Custom Roblox errors that occur when using [`Client::purchase_non_tradable_limited`].
#[non_exhaustive]
#[derive(
    thiserror::Error,
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum PurchaseNonTradableLimitedError {
    /// Thrown when the price of the item is not the same as the price you're trying to buy it for.
    #[default]
    #[error("Price Mismatch")]
    PriceMismatch,
    /// Thrown when the item is sold out.
    #[error("Sold Out")]
    SoldOut,
    /// Thrown when an unknown error occurs.
    #[error("Unknown Roblox Error Message: {0}")]
    UnknownRobloxErrorMsg(String),
}

/// Used to specify the type of classic clothing being uploaded.
///
/// This is used in [`Client::upload_classic_clothing_to_group`].
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum ClassicClothingType {
    #[default]
    Shirt,
    Pants,
    TShirt,
}

impl std::fmt::Display for ClassicClothingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassicClothingType::Shirt => write!(f, "Shirt"),
            ClassicClothingType::Pants => write!(f, "Pants"),
            ClassicClothingType::TShirt => write!(f, "T-Shirt"),
        }
    }
}

/// A struct containing (mostly) all the fields possibly returned from <https://apis.roblox.com/marketplace-items/v1/items/details>.
///
/// Returned from [`Client::non_tradable_limited_details`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct NonTradableLimitedDetails {
    /// The normal u64 item id.
    pub item_id: u64,
    /// The string item id. This only exists for non-tradable limiteds.
    pub collectible_item_id: String,
    /// The string used as a product id to buy the item.
    pub collectible_product_id: String,
    /// The name of the item.
    pub name: String,
    /// The description of the item.
    pub description: String,
    /// Whether the creator has a verified badge.
    pub creator_has_verified_badge: bool,
    /// The type of creator that created the item (User or Group).
    pub creator_type: CreatorType,
    /// The id (group or user) of the creator. The value is 1 if the creator is Roblox.
    pub creator_id: u64,
    /// The name of the creator. The value is "Roblox" if the creator is Roblox.
    pub creator_name: String,
    /// It's unclear which one of these to use as `price` and `lowest_price`
    /// have been the same every time I have tested this.
    pub price: u64,
    /// It's unclear which one of these to use as `price` and `lowest_price`
    /// have been the same every time I have tested this.
    pub lowest_price: u64,
    /// The stock amount left.
    pub remaining_stock: u64,
    /// The total stock amount on release.
    pub total_stock: u64,
    /// The error code if Roblox decides to throw one.
    pub error_code: Option<u64>,
}

impl TryFrom<request_types::NonTradableLimitedDetailsRaw> for NonTradableLimitedDetails {
    type Error = RoboatError;

    fn try_from(raw: request_types::NonTradableLimitedDetailsRaw) -> Result<Self, Self::Error> {
        let item_id = raw.item_target_id;
        let collectible_item_id = raw.collectible_item_id;
        let collectible_product_id = raw.collectible_product_id;
        let name = raw.name;
        let description = raw.description;
        let creator_has_verified_badge = raw.creator_has_verified_badge;
        let creator_type = raw.creator_type;
        let creator_id = raw.creator_id;
        let creator_name = raw.creator_name;
        let price = raw.price;
        let lowest_price = raw.lowest_price;
        let remaining_stock = raw.units_available_for_consumption;
        let total_stock = raw.asset_stock;
        let error_code = raw.error_code;

        Ok(Self {
            item_id,
            collectible_item_id,
            collectible_product_id,
            name,
            description,
            creator_has_verified_badge,
            creator_type,
            creator_id,
            creator_name,
            price,
            lowest_price,
            remaining_stock,
            total_stock,
            error_code,
        })
    }
}

impl Client {
    /// Grabs details of one or more non-tradable limiteds from <https://apis.roblox.com/marketplace-items/v1/items/details>.
    /// Does not work on normal items. Note that this is a messy,
    /// all-encompassing endpoint that should only be used directly when necessary.
    ///
    /// Specialized endpoints that use this internally include: [`Client::collectible_product_id`] and [`Client::collectible_product_id_bulk`].
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    /// * The amount of items that can be requested at once is unknown as not enough non-tradable limiteds exist, and the
    /// endpoint doesn't accept duplicates.
    /// * Will repeat once if the x-csrf-token is invalid.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [X-CSRF-TOKEN Required Errors](#x-csrf-token-required-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new()
    ///     .roblosecurity(ROBLOSECURITY.to_string())
    ///     .build();
    ///
    /// let collectible_item_id_1 = "a4b5cb79-5218-4ca1-93fa-1e3436f595ef".to_owned();
    /// let collectible_item_id_2 = "61f2e366-9fe6-4562-8ce3-47334083372a".to_owned();
    /// let items = vec![collectible_item_id_1, collectible_item_id_2];
    ///
    /// let details = client.non_tradable_limited_details(items).await?;
    ///
    /// println!("Item Name: {}", details[0].name);
    /// println!("Item Description: {}", details[0].description);
    /// println!("Item Price: {}", details[0].price);
    ///
    /// println!("Item Name: {}", details[1].name);
    /// println!("Item Description: {}", details[1].description);
    /// println!("Item Price: {}", details[1].price);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn non_tradable_limited_details(
        &self,
        collectible_item_ids: Vec<String>,
    ) -> Result<Vec<NonTradableLimitedDetails>, RoboatError> {
        match self
            .non_tradable_limited_details_internal(collectible_item_ids.clone())
            .await
        {
            Ok(x) => Ok(x),
            Err(e) => match e {
                RoboatError::InvalidXcsrf(new_xcsrf) => {
                    self.set_xcsrf(new_xcsrf).await;

                    self.non_tradable_limited_details_internal(collectible_item_ids)
                        .await
                }
                _ => Err(e),
            },
        }
    }

    /// Fetches the collectible product id of a non-tradeable limited. Uses [`Client::non_tradable_limited_details`] internally
    /// (which fetches from <https://apis.roblox.com/marketplace-items/v1/items/details>)
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    /// * Will repeat once if the x-csrf-token is invalid.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [X-CSRF-TOKEN Required Errors](#x-csrf-token-required-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new()
    ///     .roblosecurity(ROBLOSECURITY.to_string())
    ///     .build();
    ///
    /// let collectible_item_id = "a4b5cb79-5218-4ca1-93fa-1e3436f595ef".to_owned();
    /// let collectible_product_id = client.collectible_product_id(collectible_item_id).await?;
    ///
    /// println!("Collectible Product ID: {}", collectible_product_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn collectible_product_id(
        &self,
        collectible_item_id: String,
    ) -> Result<String, RoboatError> {
        let details = self
            .non_tradable_limited_details(vec![collectible_item_id])
            .await?;

        let collectible_product_id = details
            .first()
            .ok_or(RoboatError::MalformedResponse)?
            .collectible_product_id
            .clone();

        Ok(collectible_product_id)
    }

    /// Fetches collectible product ids of multiple non-tradeable limiteds. Uses [`Client::non_tradable_limited_details`] internally
    /// (which fetches from <https://apis.roblox.com/marketplace-items/v1/items/details>)
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    /// * The amount of items that can be requested at once is unknown as not enough non-tradable limiteds exist, and the
    /// endpoint doesn't accept duplicates.
    /// * Will repeat once if the x-csrf-token is invalid.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [X-CSRF-TOKEN Required Errors](#x-csrf-token-required-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new()
    ///     .roblosecurity(ROBLOSECURITY.to_string())
    ///     .build();
    ///
    /// let collectible_item_id_1 = "a4b5cb79-5218-4ca1-93fa-1e3436f595ef".to_owned();
    /// let collectible_item_id_2 = "61f2e366-9fe6-4562-8ce3-47334083372a".to_owned();
    /// let items = vec![collectible_item_id_1, collectible_item_id_2];
    ///
    /// let collectible_product_ids = client.collectible_product_id_bulk(items).await?;
    ///
    /// println!("Collectible Product ID 1: {}", collectible_product_ids[0]);
    /// println!("Collectible Product ID 2: {}", collectible_product_ids[1]);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn collectible_product_id_bulk(
        &self,
        collectible_item_ids: Vec<String>,
    ) -> Result<Vec<String>, RoboatError> {
        let collectible_item_ids_len = collectible_item_ids.len();

        let details = self
            .non_tradable_limited_details(collectible_item_ids)
            .await?;

        let collectible_product_ids = details
            .iter()
            .map(|x| x.collectible_product_id.clone())
            .collect::<Vec<String>>();

        if collectible_product_ids.len() != collectible_item_ids_len {
            return Err(RoboatError::MalformedResponse);
        }

        Ok(collectible_product_ids)
    }

    /// Fetches the id of the original creator of a non-tradable limited. This is used when buying stock
    /// of an item (and not resellers).
    ///
    /// Uses [`Client::non_tradable_limited_details`] internally
    /// (which fetches from <https://apis.roblox.com/marketplace-items/v1/items/details>)
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    /// * Will repeat once if the x-csrf-token is invalid.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [X-CSRF-TOKEN Required Errors](#x-csrf-token-required-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new()
    ///     .roblosecurity(ROBLOSECURITY.to_string())
    ///     .build();
    ///
    /// let collectible_item_id = "a4b5cb79-5218-4ca1-93fa-1e3436f595ef".to_owned();
    /// let collectible_creator_id = client.collectible_creator_id(collectible_item_id).await?;
    ///
    /// println!("Collectible Creator ID: {}", collectible_creator_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn collectible_creator_id(
        &self,
        collectible_item_id: String,
    ) -> Result<u64, RoboatError> {
        let details = self
            .non_tradable_limited_details(vec![collectible_item_id])
            .await?;

        let collectible_creator_id = details
            .first()
            .ok_or(RoboatError::MalformedResponse)?
            .creator_id;

        Ok(collectible_creator_id)
    }

    /// Purchases a non-tradable limited (includes ugc limiteds) using endpoint
    /// <https://apis.roblox.com/marketplace-sales/v1/item/{collectible_item_id}/purchase-item>.
    ///
    /// # Warning
    /// This endpoint and related endpoints may change as new things are discovered about this endpoint.
    /// This is because no resellers can sell items yet.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    /// * Will repeat once if the x-csrf-token is invalid.
    /// * Currently only tested to work when buying from users (as opposed to groups), and only tested
    /// when buying the items from the original seller (with original stock). This is because
    /// these are the only conditions that currently exist as of 4/14/2023.
    ///
    /// # Return Value Notes
    /// * Will return `Ok(())` if the limited was successfully purchased.
    ///
    /// # Argument Notes
    /// * `collectible_item_id` is the string id of a non-tradable limited. It can be
    /// fetched using [`Client::collectible_item_id`].
    /// * `collectible_product_id` is the string product id of a non-tradable limited. It can be
    /// fetched using [`Client::collectible_product_id`].
    /// * `collectible_seller_id` is the user id of the seller of a non-tradable limited. It can be
    /// fetched using [`Client::collectible_creator_id`] (currently it is unknown how to buy from a reseller
    /// instead of the original creator as they do not exist yet).
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    /// * All errors under [X-CSRF-TOKEN Required Errors](#x-csrf-token-required-errors).
    /// * [`RoboatError::PurchaseNonTradableLimitedError`] - Nested inside this error, all variants of [`PurchaseNonTradableLimitedError`] may be thrown.
    ///
    /// # Examples
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// let collectible_item_id = "abc".to_string();;
    /// let collectible_product_id = "xyz".to_string();
    /// let collectible_seller_id = 123456789;
    /// let price = 0;
    ///
    /// let _ = client.purchase_non_tradable_limited(collectible_item_id, collectible_product_id, collectible_seller_id, price).await?;
    /// println!("Successfully Purchased!");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn purchase_non_tradable_limited(
        &self,
        collectible_item_id: String,
        collectible_product_id: String,
        collectible_seller_id: u64,
        price: u64,
    ) -> Result<(), RoboatError> {
        match self
            .purchase_non_tradable_limited_internal(
                collectible_item_id.clone(),
                collectible_product_id.clone(),
                collectible_seller_id,
                price,
            )
            .await
        {
            Ok(x) => Ok(x),
            Err(e) => match e {
                RoboatError::InvalidXcsrf(new_xcsrf) => {
                    self.set_xcsrf(new_xcsrf).await;

                    self.purchase_non_tradable_limited_internal(
                        collectible_item_id,
                        collectible_product_id,
                        collectible_seller_id,
                        price,
                    )
                    .await
                }
                _ => Err(e),
            },
        }
    }

    /// Fetches detailed information about a specific asset using its asset ID.
    ///
    /// This function retrieves asset information such as the asset's name, description,
    /// and other related details from the Roblox API. It uses the internal `get_asset_info_internal`
    /// function to send a request to the server and parse the response.
    ///
    /// # Argument Notes
    ///
    /// * `asset_id`: The unique ID of the asset whose information is being fetched. This ID can be obtained
    ///   through various means, such as browsing the asset page on Roblox or using other API endpoints.
    ///
    /// # Notes
    /// * requires .ROBLOSECURITY cookie
    /// * Will repeat once if the x-csrf-token is invalid.

    /// # Errors
    ///
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    /// * All errors under [X-CSRF-TOKEN Required Errors](#x-csrf-token-required-errors).
    /// * [`RoboatError::MalformedResponse`] - If the response from the server is malformed and cannot be parsed.
    /// * [`RoboatError::NetworkError`] - If there is an issue with the network request.
    /// * [`RoboatError::RequestError`] - If there is any general error with the request or the server response.
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
    /// let asset_id = 123456789;
    ///
    /// let asset_info = client.get_asset_info(asset_id).await?;
    /// println!("Asset Info: {:?}", asset_info);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_asset_info(&self, asset_id: u64) -> Result<AssetInfo, RoboatError> {
        match self.get_asset_info_internal(asset_id.clone()).await {
            Ok(x) => Ok(x),
            Err(e) => match e {
                RoboatError::InvalidXcsrf(new_xcsrf) => {
                    self.set_xcsrf(new_xcsrf).await;
                    self.get_asset_info_internal(asset_id).await
                }
                _ => Err(e),
            },
        }
    }

    /// Uploads classic clothing to a group. This currently only works for classic clothing and
    /// for people uploading from groups. This is because the endpoint is not fully understood yet
    /// and reverse engineering it is expensive.
    ///
    /// Uses endpoint <https://apis.roblox.com/assets/user-auth/v1/assets>.
    ///
    /// # WARNING: UNDER CONSTRUCTION
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    /// * Will repeat once if the x-csrf-token is invalid.
    /// * The `image_path` must be a valid path to an image file.
    pub async fn upload_classic_clothing_to_group(
        &self,
        group_id: u64,
        name: String,
        description: String,
        image_path: String,
        classic_clothing_type: ClassicClothingType,
    ) -> Result<(), RoboatError> {
        match self
            .upload_classic_clothing_to_group_internal(
                group_id,
                name.clone(),
                description.clone(),
                image_path.clone(),
                classic_clothing_type,
            )
            .await
        {
            Ok(x) => Ok(x),
            Err(e) => match e {
                RoboatError::InvalidXcsrf(new_xcsrf) => {
                    self.set_xcsrf(new_xcsrf).await;

                    self.upload_classic_clothing_to_group_internal(
                        group_id,
                        name,
                        description,
                        image_path,
                        classic_clothing_type,
                    )
                    .await
                }
                _ => Err(e),
            },
        }
    }
}

mod internal {
    use std::path::Path;

    use reqwest::header;

    use super::{
        request_types, sort_items_by_argument_order, ClassicClothingType,
        NonTradableLimitedDetails, PurchaseNonTradableLimitedError, COLLECTIBLE_ITEM_DETAILS_API,
        PURCHASE_NON_TRADEABLE_LIMITED_API_PART_1, PURCHASE_NON_TRADEABLE_LIMITED_API_PART_2,
    };
    use crate::{
        bedev2::{request_types::AssetInfo, ASSET_API},
        Client, RoboatError, XCSRF_HEADER,
    };

    impl Client {
        pub(super) async fn non_tradable_limited_details_internal(
            &self,
            collectible_item_ids: Vec<String>,
        ) -> Result<Vec<NonTradableLimitedDetails>, RoboatError> {
            let request_body = serde_json::json!({
                "itemIds": collectible_item_ids,
            });

            let request_result = self
                .reqwest_client
                .post(COLLECTIBLE_ITEM_DETAILS_API)
                .header(XCSRF_HEADER, self.xcsrf().await)
                .header(header::COOKIE, self.cookie_string()?)
                .json(&request_body)
                .send()
                .await;

            let response = Self::validate_request_result(request_result).await?;
            let raw =
                Self::parse_to_raw::<Vec<request_types::NonTradableLimitedDetailsRaw>>(response)
                    .await?;

            let mut collectible_item_details = Vec::new();

            for raw_details in raw {
                let details = NonTradableLimitedDetails::try_from(raw_details)?;
                collectible_item_details.push(details);
            }

            sort_items_by_argument_order(&mut collectible_item_details, &collectible_item_ids);

            Ok(collectible_item_details)
        }

        pub(super) async fn purchase_non_tradable_limited_internal(
            &self,
            collectible_item_id: String,
            collectible_product_id: String,
            seller_id: u64,
            price: u64,
        ) -> Result<(), RoboatError> {
            let idempotency_key = uuid::Uuid::new_v4().to_string();
            let client_user_id = self.user_id().await?;

            let request_body = serde_json::json!({
                "collectibleItemId": collectible_item_id,
                "expectedCurrency": 1,
                "expectedPrice": price,
                "expectedPurchaserId":client_user_id,
                "expectedPurchaserType": "User",
                "expectedSellerId": seller_id,
                "expectedSellerType": "User",
                "idempotencyKey": idempotency_key,
                "collectibleProductId": collectible_product_id,
            });

            let formatted_url = format!(
                "{}{}{}",
                PURCHASE_NON_TRADEABLE_LIMITED_API_PART_1,
                collectible_item_id,
                PURCHASE_NON_TRADEABLE_LIMITED_API_PART_2
            );

            let request_result = self
                .reqwest_client
                .post(formatted_url)
                .header(XCSRF_HEADER, self.xcsrf().await)
                .header(header::COOKIE, self.cookie_string()?)
                .json(&request_body)
                .send()
                .await;

            let response = Self::validate_request_result(request_result).await?;
            let raw = Self::parse_to_raw::<request_types::PurchaseNonTradeableLimitedRaw>(response)
                .await?;

            if raw.purchased {
                return Ok(());
            }

            let err_msg = raw.error_message.ok_or(RoboatError::MalformedResponse)?;

            match err_msg.as_str() {
                "PriceMismatch" => Err(RoboatError::PurchaseNonTradableLimitedError(
                    PurchaseNonTradableLimitedError::PriceMismatch,
                )),
                "QuantityExhausted" => Err(RoboatError::PurchaseNonTradableLimitedError(
                    PurchaseNonTradableLimitedError::SoldOut,
                )),
                _ => Err(RoboatError::PurchaseNonTradableLimitedError(
                    PurchaseNonTradableLimitedError::UnknownRobloxErrorMsg(raw.purchase_result),
                )),
            }
        }

        pub(super) async fn get_asset_info_internal(
            &self,
            asset_id: u64,
        ) -> Result<AssetInfo, RoboatError> {
            let cookie_string = self.cookie_string()?;
            let xcsrf = self.xcsrf().await;
            let formatted_url = format!("{}/{}", ASSET_API, asset_id);

            let response_result = self
                .reqwest_client
                .get(formatted_url)
                .header(header::COOKIE, cookie_string)
                .header(XCSRF_HEADER, xcsrf)
                .send()
                .await;

            let response = Self::validate_request_result(response_result).await?;

            let asset_info = Self::parse_to_raw::<request_types::AssetInfo>(response).await?;

            Ok(asset_info)
        }

        pub(super) async fn upload_classic_clothing_to_group_internal(
            &self,
            group_id: u64,
            name: String,
            description: String,
            image_path: String,
            classic_clothing_type: ClassicClothingType,
        ) -> Result<(), RoboatError> {
            let filename = Path::new(&image_path)
                .file_name()
                .map(|x| x.to_string_lossy().to_string())
                .ok_or(RoboatError::InvalidPath(image_path.clone()))?;

            let asset_name_patch = match classic_clothing_type {
                ClassicClothingType::Shirt => "Shirt",
                ClassicClothingType::Pants => "Pants",
                ClassicClothingType::TShirt => "Tshirt",
            };

            let expected_price = match classic_clothing_type {
                ClassicClothingType::Shirt | ClassicClothingType::Pants => 10,
                ClassicClothingType::TShirt => 0,
            };

            let form = reqwest::multipart::Form::new()
                .part("fileContent", reqwest::multipart::Part::bytes(tokio::fs::read(image_path).await?).file_name(filename))
                .text("request", format!("{{\"displayName\":\"{name}\",\"description\":\"{description}\",\"assetType\":\"{asset_name_patch}\",\"creationContext\":{{\"creator\":{{\"groupId\":{group_id}}},\"expectedPrice\":{expected_price}}}}}" ));

            let cookie_string = self.cookie_string()?;
            let xcsrf = self.xcsrf().await;

            let response_result = self
                .reqwest_client
                .request(reqwest::Method::POST, ASSET_API)
                .header(header::COOKIE, cookie_string)
                .header(XCSRF_HEADER, xcsrf)
                .multipart(form)
                .send()
                .await;

            let response = Self::validate_request_result(response_result).await?;
            let _ = Self::parse_to_raw::<request_types::UploadClassicClothingRaw>(response).await?;

            Ok(())
        }
    }
}

/// Makes sure that the items are in the same order as the arguments.
///
/// For example, if the arguments are `["1", "2", "3"]` and the resulting items are `["2", "1", "3"]`,
/// then the resulting items will be `["1", "2", "3"]`.
fn sort_items_by_argument_order(items: &mut [NonTradableLimitedDetails], arguments: &[String]) {
    items.sort_by(|a, b| {
        let a_index = arguments
            .iter()
            .position(|collectible_item_id| collectible_item_id == &a.collectible_item_id)
            .unwrap_or(usize::MAX);

        let b_index = arguments
            .iter()
            .position(|collectible_item_id| collectible_item_id == &b.collectible_item_id)
            .unwrap_or(usize::MAX);

        a_index.cmp(&b_index)
    });
}
