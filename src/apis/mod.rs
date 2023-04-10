use crate::catalog::avatar_catalog::CreatorType;
use crate::{Client, RoboatError};
use serde::{Deserialize, Serialize};

mod request_types;

const COLLECTIBLE_ITEM_DETAILS_API: &str =
    "https://apis.roblox.com/marketplace-items/v1/items/details";

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
            .get(0)
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
}

mod internal {
    use reqwest::header;

    use super::{
        request_types, sort_items_by_argument_order, NonTradableLimitedDetails,
        COLLECTIBLE_ITEM_DETAILS_API,
    };
    use crate::{Client, RoboatError, XCSRF_HEADER};

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
