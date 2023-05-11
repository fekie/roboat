#[allow(unused_imports)]
use super::{Category, CreatorType, Genre, SortAggregation, SortType, Subcategory};
use crate::{Client, RoboatError};
use serde::{Deserialize, Serialize};

const AVATAR_CATALOG_SEARCH_BASE_URL: &str = "https://catalog.roblox.com/v1/search/items?";

/// The allowed limits in a search query.
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum QueryLimit {
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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize)]
pub struct AvatarSearchQuery {
    pub category: Option<Category>,
    pub creator_name: Option<String>,
    /// Corresponds to a user id or group id depending on the creator type.
    /// Must be filled if `creator_type` is filled.
    pub creator_id: Option<u64>,
    /// Must be filled if `creator_id` is filled.
    pub creator_type: Option<CreatorType>,
    /// The genres of the item; keep in mind [`QueryGenre`] is different from [`Genre`].
    pub query_genres: Vec<QueryGenre>,
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
        url.pop();
        url
    }
}

pub struct AvatarSearchQueryBuilder {
    query: AvatarSearchQuery,
}

impl AvatarSearchQueryBuilder {
    pub fn new() -> Self {
        Self {
            query: AvatarSearchQuery::default(),
        }
    }

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
    pub async fn avatar_catalog_search(&self, query: AvatarSearchQuery) -> Result<(), RoboatError> {
        let url = query.to_url();

        dbg!(url);

        todo!()
    }
}
