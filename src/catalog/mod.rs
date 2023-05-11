use crate::{Client, RoboatError};
use request_types::AvatarSearchQueryResponse;

use catalog_types::QueryLimit;

/// re export all types
pub use catalog_types::{
    AssetType, AvatarSearchQuery, AvatarSearchQueryBuilder, BundleType, Category, CreatorType,
    Genre, Item, ItemDetails, ItemRestriction, ItemStatus, ItemType, PriceStatus, QueryGenre,
    SortAggregation, SortType, Subcategory,
};

/// Types related to the avatar catalog.
/// They are in this module because there are so many.
pub mod catalog_types;
mod request_types;

// A useful link for the encodings for item types: https://create.roblox.com/docs/studio/catalog-api#avatar-catalog-api

const ITEM_DETAILS_API: &str = "https://catalog.roblox.com/v1/catalog/items/details";

/// We set this to thirty because it's unlikely to be anything else.
const QUERY_LIMIT: QueryLimit = QueryLimit::Thirty;

impl Client {
    /// Grabs details of one or more items from <https://catalog.roblox.com/v1/catalog/items/details>.
    /// This now supports "new" limiteds (which include ugc limiteds). Note that this is a messy,
    /// all-encompassing endpoint that should only be used directly when necessary.
    ///
    /// Specialized endpoints that use this internally include: [`Client::product_id`], [`Client::product_id_bulk`],
    /// [`Client::collectible_item_id`], and [`Client::collectible_item_id_bulk`].
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * This endpoint will accept up to 120 items at a time.
    /// * Will repeat once if the x-csrf-token is invalid.
    ///
    /// # Argument Notes
    /// * The `id` parameter is that acts differently for this endpoint than others.
    /// If the `item_type` is [`ItemType::Asset`], then `id` is the item ID.
    /// Otherwise, if the `item_type` is [`ItemType::Bundle`], then `id` is the bundle ID.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [X-CSRF-TOKEN Required Errors](#x-csrf-token-required-errors).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use roboat::catalog::{ItemType, Item};
    /// use roboat::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let asset = Item {
    ///     item_type: ItemType::Asset,
    ///     id: 1365767,
    /// };
    ///
    /// let bundle = Item {
    ///    item_type: ItemType::Bundle,
    ///    id: 39,
    /// };
    ///
    /// let ugc_limited = Item {
    ///    item_type: ItemType::Asset,
    ///    id: 13032232281,
    /// };
    ///
    /// let items = vec![asset, bundle];
    /// let details = client.item_details(items).await?;
    ///
    /// println!("Item Name: {}", details[0].name);
    /// println!("Bundle Name: {}", details[1].name);
    /// println!("UGC Limited Name: {} / UGC Limited Collectible ID: {}", details[2].name,
    ///     details[2].collectible_item_id.as_ref().ok_or("No collectible ID")?);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn item_details(&self, items: Vec<Item>) -> Result<Vec<ItemDetails>, RoboatError> {
        match self.item_details_internal(items.clone()).await {
            Ok(x) => Ok(x),
            Err(e) => match e {
                RoboatError::InvalidXcsrf(new_xcsrf) => {
                    self.set_xcsrf(new_xcsrf).await;

                    self.item_details_internal(items).await
                }
                _ => Err(e),
            },
        }
    }

    /// Fetches the product ID of an item (must be an asset). Uses [`Client::item_details`] internally
    /// (which fetches from <https://catalog.roblox.com/v1/catalog/items/details>)
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * Will repeat once if the x-csrf-token is invalid.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [X-CSRF-TOKEN Required Errors](#x-csrf-token-required-errors).
    ///
    /// # Example
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let item_id = 12345679;
    ///
    /// let product_id = client.product_id(item_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn product_id(&self, item_id: u64) -> Result<u64, RoboatError> {
        let item = Item {
            item_type: ItemType::Asset,
            id: item_id,
        };

        let details = self.item_details(vec![item]).await?;

        details
            .get(0)
            .ok_or(RoboatError::MalformedResponse)?
            .product_id
            .ok_or(RoboatError::MalformedResponse)
    }

    /// Fetches the product ID of multiple items (must be an asset). More efficient than calling [`Client::product_id`] repeatedly.
    /// Uses [`Client::item_details`] internally
    /// (which fetches from <https://catalog.roblox.com/v1/catalog/items/details>).
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * This endpoint will accept up to 120 items at a time.
    /// * Will repeat once if the x-csrf-token is invalid.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [X-CSRF-TOKEN Required Errors](#x-csrf-token-required-errors).
    ///
    /// # Example
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let item_id_1 = 12345679;
    /// let item_id_2 = 987654321;
    ///
    /// let product_ids = client.product_id_bulk(vec![item_id_1, item_id_2]).await?;
    ///
    /// let product_id_1 = product_ids.get(0).ok_or("No product ID 1")?;
    /// let product_id_2 = product_ids.get(1).ok_or("No product ID 2")?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn product_id_bulk(&self, item_ids: Vec<u64>) -> Result<Vec<u64>, RoboatError> {
        let item_ids_len = item_ids.len();

        let mut items = Vec::new();

        for item_id in item_ids {
            let item = Item {
                item_type: ItemType::Asset,
                id: item_id,
            };

            items.push(item);
        }

        let details = self.item_details(items).await?;

        let product_ids = details
            .iter()
            .filter_map(|x| x.product_id)
            .collect::<Vec<u64>>();

        if product_ids.len() != item_ids_len {
            return Err(RoboatError::MalformedResponse);
        }

        Ok(product_ids)
    }

    /// Fetches the collectible item id of a multiple non-tradeable limited (including ugc limiteds).
    /// More efficient than calling [`Client::product_id`] repeatedly.
    /// Uses [`Client::item_details`] internally
    /// (which fetches from <https://catalog.roblox.com/v1/catalog/items/details>).
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * Will repeat once if the x-csrf-token is invalid.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [X-CSRF-TOKEN Required Errors](#x-csrf-token-required-errors).
    ///
    /// # Example
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let item_id = 12345679;
    ///
    /// let collectible_item_id = client.collectible_item_id(item_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn collectible_item_id(&self, item_id: u64) -> Result<String, RoboatError> {
        let item = Item {
            item_type: ItemType::Asset,
            id: item_id,
        };

        let details = self.item_details(vec![item]).await?;

        details
            .get(0)
            .ok_or(RoboatError::MalformedResponse)?
            .collectible_item_id
            .clone()
            .ok_or(RoboatError::MalformedResponse)
    }

    /// Fetches the collectible item ids of multiple non-tradeable limiteds (including ugc limiteds).
    /// More efficient than calling [`Client::collectible_item_id`] repeatedly.
    /// Uses [`Client::item_details`] internally
    /// (which fetches from <https://catalog.roblox.com/v1/catalog/items/details>).
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * This endpoint will accept up to 120 items at a time.
    /// * Will repeat once if the x-csrf-token is invalid.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [X-CSRF-TOKEN Required Errors](#x-csrf-token-required-errors).
    ///
    /// # Example
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let item_id_1 = 12345679;
    /// let item_id_2 = 987654321;
    ///
    /// let collectible_item_ids = client.collectible_item_id_bulk(vec![item_id_1, item_id_2]).await?;
    ///
    /// let collectible_item_id_1 = collectible_item_ids.get(0).ok_or("No collectible item ID 1")?;
    /// let collectible_item_id_2 = collectible_item_ids.get(1).ok_or("No collectible item ID 2")?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn collectible_item_id_bulk(
        &self,
        item_ids: Vec<u64>,
    ) -> Result<Vec<String>, RoboatError> {
        let item_ids_len = item_ids.len();

        let mut items = Vec::new();

        for item_id in item_ids {
            let item = Item {
                item_type: ItemType::Asset,
                id: item_id,
            };

            items.push(item);
        }

        let details = self.item_details(items).await?;

        let collectible_item_ids = details
            .iter()
            .filter_map(|x| x.collectible_item_id.clone())
            .collect::<Vec<String>>();

        if collectible_item_ids.len() != item_ids_len {
            return Err(RoboatError::MalformedResponse);
        }

        Ok(collectible_item_ids)
    }

    /// Performs a search query using <https://catalog.roblox.com/v1/search/items>.
    /// Query parameters are specified using the [`AvatarSearchQuery`] struct, which can be built using the [`AvatarSearchQueryBuilder`].
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    ///
    /// # Argument Notes
    /// * Query parameters are specified using the [`AvatarSearchQuery`] struct, which can be built using the [`AvatarSearchQueryBuilder`].
    /// * If the Query is empty, no next page cursor will be returned.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use roboat::catalog::{Item, Category, AvatarSearchQueryBuilder};
    /// use roboat::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let query = AvatarSearchQueryBuilder::new()
    ///     .keyword("cute".to_owned())
    ///     .category(Category::Accessories)
    ///     .build();
    ///
    /// let next_cursor = None;
    ///
    /// // Fetch the first page of results.
    /// let (items, next_cursor) = client.avatar_catalog_search(&query, next_cursor).await?;
    /// println!("Found {} items.", items.len());
    /// println!("Next cursor: {}", next_cursor.clone().unwrap_or_default());
    ///
    /// // Fetch the next page of results.
    /// let (items, next_cursor) = client.avatar_catalog_search(&query, next_cursor).await?;
    /// println!("Found {} items.", items.len());
    /// println!("Next cursor: {}", &next_cursor.clone().unwrap_or_default());
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn avatar_catalog_search(
        &self,
        query: &AvatarSearchQuery,
        cursor: Option<String>,
    ) -> Result<(Vec<Item>, Option<String>), RoboatError> {
        let formatted_url = format!(
            "{}&limit={}&cursor={}",
            query.to_url(),
            QUERY_LIMIT.as_u8(),
            cursor.unwrap_or_default()
        );

        let request_result = self.reqwest_client.get(formatted_url).send().await;

        let response = Self::validate_request_result(request_result).await?;
        let raw = Self::parse_to_raw::<AvatarSearchQueryResponse>(response).await?;

        let items = raw.items;
        let next_cursor = raw.next_page_cursor;

        Ok((items, next_cursor))
    }
}

mod internal {
    use super::{request_types, sort_items_by_argument_order, Item, ItemDetails, ITEM_DETAILS_API};
    use crate::XCSRF_HEADER;
    use crate::{Client, RoboatError};

    impl Client {
        /// Used internally to fetch the details of one or more items from <https://catalog.roblox.com/v1/catalog/items/details>.
        pub(super) async fn item_details_internal(
            &self,
            items: Vec<Item>,
        ) -> Result<Vec<ItemDetails>, RoboatError> {
            let request_body = request_types::ItemDetailsReqBody {
                // Convert the ItemParameters to te reqwest ItemParametersReq
                items: items
                    .iter()
                    .map(|x| request_types::ItemReq::from(*x))
                    .collect(),
            };

            let request_result = self
                .reqwest_client
                .post(ITEM_DETAILS_API)
                .header(XCSRF_HEADER, self.xcsrf().await)
                .json(&request_body)
                .send()
                .await;

            let response = Self::validate_request_result(request_result).await?;
            let raw = Self::parse_to_raw::<request_types::ItemDetailsResponse>(response).await?;

            let mut item_details = Vec::new();

            for raw_details in raw.data {
                let details = ItemDetails::try_from(raw_details)?;
                item_details.push(details);
            }

            sort_items_by_argument_order(&mut item_details, &items);

            Ok(item_details)
        }
    }
}

/// Makes sure that the items are in the same order as the arguments.
///
/// For example, if the arguments are `[1, 2, 3]` and the resulting items are `[2, 1, 3]`,
/// then the resulting items will be `[1, 2, 3]`.
fn sort_items_by_argument_order(items: &mut [ItemDetails], arguments: &[Item]) {
    items.sort_by(|a, b| {
        let a_index = arguments
            .iter()
            .position(|item_args| item_args.id == a.id)
            .unwrap_or(usize::MAX);

        let b_index = arguments
            .iter()
            .position(|item_args| item_args.id == b.id)
            .unwrap_or(usize::MAX);

        a_index.cmp(&b_index)
    });
}
