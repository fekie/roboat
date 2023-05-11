use super::request_types::CatalogSearchQueryResponse;
#[allow(unused_imports)]
use super::{Category, CreatorType, Genre, Item, SortAggregation, SortType, Subcategory};
use crate::{Client, RoboatError};
use serde::{Deserialize, Serialize};

const AVATAR_CATALOG_SEARCH_BASE_URL: &str = "https://catalog.roblox.com/v1/search/items?";

/// We set this to thirty because it's unlikely to be anything else.
const QUERY_LIMIT: QueryLimit = QueryLimit::Thirty;

/// The allowed limits in a catalog search query.
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
enum QueryLimit {
    #[default]
    Ten,
    TwentyEight,
    Thirty,
}

impl QueryLimit {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Self::Ten => 10,
            Self::TwentyEight => 28,
            Self::Thirty => 30,
        }
    }
}

/// These are only used when making a search query.
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum QueryGenre {
    #[default]
    TownAndCity,
    Medieval,
    SciFi,
    Fighting,
    Horror,
    Naval,
    Adventure,
    Sports,
    Comedy,
    Western,
    Military,
    Building,
    FPS,
    RPG,
}

impl QueryGenre {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Self::TownAndCity => 1,
            Self::Medieval => 2,
            Self::SciFi => 3,
            Self::Fighting => 4,
            Self::Horror => 5,
            Self::Naval => 6,
            Self::Adventure => 7,
            Self::Sports => 8,
            Self::Comedy => 9,
            Self::Western => 10,
            Self::Military => 11,
            Self::Building => 13,
            Self::FPS => 14,
            Self::RPG => 15,
        }
    }
}

/// Information comes directly from here <https://create.roblox.com/docs/studio/catalog-api#marketplace-api>.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct AvatarSearchQuery {
    /// Category must be filled to query more than one page.
    pub category: Option<Category>,
    /// Search by creator name. If `creator_type` is not provided, search is for users only.
    pub creator_name: Option<String>,
    /// Corresponds to a user id or group id depending on the creator type.
    /// Must be filled if `creator_type` is filled.
    pub creator_id: Option<u64>,
    /// Must be filled if `creator_id` is filled.
    pub creator_type: Option<CreatorType>,
    /// The genres of the item; keep in mind [`QueryGenre`] is different from [`Genre`].
    pub query_genres: Vec<QueryGenre>,
    /// The keyword to search for.
    pub keyword: Option<String>,
    /// The sort aggregation is used to sort the results by a specific metric.
    /// View [`SortAggregation`] for more information.
    pub sort_aggregation: Option<SortAggregation>,
    /// The sort type is used to sort the results in a specific order.
    /// View [`SortType`] for more information.
    pub sort_type: Option<SortType>,
    /// Subcategory must be filled to query more than one page.
    pub subcategory: Option<Subcategory>,
}

impl AvatarSearchQuery {
    /// Converts the query into a url.
    pub fn to_url(&self) -> String {
        let mut url = String::from(AVATAR_CATALOG_SEARCH_BASE_URL);

        if let Some(category) = self.category {
            url.push_str(&format!("category={}&", category.as_u8()));
        }

        if let Some(creator_name) = &self.creator_name {
            url.push_str(&format!("creatorName={}&", creator_name));
        }

        if let Some(creator_id) = self.creator_id {
            url.push_str(&format!("creatorTargetId={}&", creator_id));
        }

        if let Some(creator_type) = self.creator_type {
            url.push_str(&format!("creatorType={}&", creator_type.as_u8()));
        }

        if !self.query_genres.is_empty() {
            url.push_str("genre=");
            for query_genre in &self.query_genres {
                url.push_str(&format!("{},", query_genre.as_u8()));
            }
            url.push('&');
        }

        if let Some(keyword) = &self.keyword {
            url.push_str(&format!("keyword={}&", keyword));
        }

        if let Some(sort_aggregation) = self.sort_aggregation {
            url.push_str(&format!("sortAggregation={}&", sort_aggregation.as_u8()));
        }

        if let Some(sort_type) = self.sort_type {
            url.push_str(&format!("sortType={}&", sort_type.as_u8()));
        }

        if let Some(subcategory) = self.subcategory {
            url.push_str(&format!("subcategory={}&", subcategory.as_u8()));
        }

        // Remove the last & if it exists.
        if url.ends_with('&') {
            url.pop();
        }

        url
    }
}

/// A builder for [`AvatarSearchQuery`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct AvatarSearchQueryBuilder {
    query: AvatarSearchQuery,
}

impl AvatarSearchQueryBuilder {
    /// Creates a new `AvatarSearchQueryBuilder`.
    pub fn new() -> Self {
        Self {
            query: AvatarSearchQuery::default(),
        }
    }

    /// Builds the [`AvatarSearchQuery`].
    pub fn build(self) -> AvatarSearchQuery {
        self.query
    }

    #[allow(missing_docs)]
    pub fn category(mut self, category: Category) -> Self {
        self.query.category = Some(category);
        self
    }

    #[allow(missing_docs)]
    pub fn creator_name(mut self, creator_name: String) -> Self {
        self.query.creator_name = Some(creator_name);
        self
    }

    #[allow(missing_docs)]
    pub fn creator_id(mut self, creator_id: u64) -> Self {
        self.query.creator_id = Some(creator_id);
        self
    }

    #[allow(missing_docs)]
    pub fn creator_type(mut self, creator_type: CreatorType) -> Self {
        self.query.creator_type = Some(creator_type);
        self
    }

    #[allow(missing_docs)]
    pub fn query_genres(mut self, query_genres: Vec<QueryGenre>) -> Self {
        self.query.query_genres = query_genres;
        self
    }

    #[allow(missing_docs)]
    pub fn keyword(mut self, keyword: String) -> Self {
        self.query.keyword = Some(keyword);
        self
    }

    #[allow(missing_docs)]
    pub fn sort_aggregation(mut self, sort_aggregation: SortAggregation) -> Self {
        self.query.sort_aggregation = Some(sort_aggregation);
        self
    }

    #[allow(missing_docs)]
    pub fn sort_type(mut self, sort_type: SortType) -> Self {
        self.query.sort_type = Some(sort_type);
        self
    }

    #[allow(missing_docs)]
    pub fn subcategory(mut self, subcategory: Subcategory) -> Self {
        self.query.subcategory = Some(subcategory);
        self
    }
}

impl Client {
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
    /// use roboat::catalog::avatar_catalog::{Item, Category};
    /// use roboat::catalog::avatar_catalog::search_query::AvatarSearchQueryBuilder;
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
        let raw = Self::parse_to_raw::<CatalogSearchQueryResponse>(response).await?;

        let items = raw.items;
        let next_cursor = raw.next_page_cursor;

        Ok((items, next_cursor))
    }
}
