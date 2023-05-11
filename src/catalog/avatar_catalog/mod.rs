use crate::{Client, RoboatError};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

mod request_types;
/// Structs and methods related to avatar catalog search.
pub mod search_query;

// A useful link for the encodings for item types: https://create.roblox.com/docs/studio/catalog-api#avatar-catalog-api

const ITEM_DETAILS_API: &str = "https://catalog.roblox.com/v1/catalog/items/details";

/// An enum representing the overall high level type of the item (Asset or Bundle)
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum ItemType {
    /// An individual asset.
    #[default]
    Asset,
    /// A bundle (such as an animation package).
    Bundle,
}

/// An enum representing the type of the asset (hat, shirt, gear).
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum AssetType {
    #[default]
    TShirt,
    Hat,
    Shirt,
    Pants,
    Head,
    Face,
    Gear,
    Arms,
    Legs,
    Torso,
    RightArm,
    LeftArm,
    LeftLeg,
    RightLeg,
    HairAccessory,
    FaceAccessory,
    NeckAccessory,
    ShoulderAccessory,
    FrontAccessory,
    BackAccessory,
    WaistAccessory,
    ClimbAnimation,
    DeathAnimation,
    FallAnimation,
    IdleAnimation,
    JumpAnimation,
    RunAnimation,
    SwimAnimation,
    WalkAnimation,
    PoseAnimation,
    EmoteAnimation,
}

/// An enum representing the type of bundle (BodyParts or AvatarAnimations).
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[allow(missing_docs)]
pub enum BundleType {
    #[default]
    BodyParts,
    AvatarAnimations,
}

/// An enum representing the genre of an item (war, funny). Only used when returning
/// info from item_details.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[allow(missing_docs)]
pub enum Genre {
    #[default]
    All,
    Tutorial,
    Scary,
    TownAndCity,
    War,
    Funny,
    Fantasy,
    Adventure,
    SciFi,
    Pirate,
    FPS,
    RPG,
    Sports,
    Ninja,
    WildWest,
}

/// The status of an item (Sale, Exclusive).
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum ItemStatus {
    #[default]
    New,
    Sale,
    XboxExclusive,
    AmazonExclusive,
    GooglePlayExclusive,
    IosExclusive,
    SaleTimer,
}

/// Restriction on an item (ThirteenPlus, Limited). Not sure why limited
/// is here but I guess that's how they do it.
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum ItemRestriction {
    #[default]
    ThirteenPlus,
    LimitedUnique,
    Limited,
    Rthro,
    /// Appears to be used only for "new" limiteds (including ugc limiteds).
    Collectible,
}

/// Type of creator that created the item (User or Group)
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[allow(missing_docs)]
pub enum CreatorType {
    #[default]
    User,
    Group,
}

impl CreatorType {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Self::User => 1,
            Self::Group => 2,
        }
    }
}

/// The price status of an item. Only applies to items not on sale (Free, Offsale).
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[allow(missing_docs)]
pub enum PriceStatus {
    #[default]
    Free,
    #[serde(alias = "Off Sale")]
    Offsale,
    #[serde(alias = "No Resellers")]
    NoResellers,
}

/// The broad category of an item for use in search (Clothing, Collectables).
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[allow(missing_docs)]
pub enum Category {
    #[default]
    Featured,
    All,
    Collectibles,
    Clothing,
    BodyParts,
    Gear,
    Accessories,
    AvatarAnimations,
    CommunityCreations,
}

impl Category {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Self::Featured => 0,
            Self::All => 1,
            Self::Collectibles => 2,
            Self::Clothing => 3,
            Self::BodyParts => 4,
            Self::Gear => 5,
            Self::Accessories => 11,
            Self::AvatarAnimations => 12,
            Self::CommunityCreations => 13,
        }
    }
}

/// A time period for when a sort applies.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[allow(missing_docs)]
pub enum SortAggregation {
    #[default]
    PastDay,
    PastWeek,
    PastMonth,
    AllTime,
}

impl SortAggregation {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Self::PastDay => 0,
            Self::PastWeek => 1,
            Self::PastMonth => 2,
            Self::AllTime => 3,
        }
    }
}

/// Sorting types that can be used in an item search.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[allow(missing_docs)]
pub enum SortType {
    #[default]
    Relevance,
    Favorited,
    Sales,
    Updated,
    PriceAsc,
    PriceDesc,
}

impl SortType {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Self::Relevance => 0,
            Self::Favorited => 1,
            Self::Sales => 2,
            Self::Updated => 3,
            Self::PriceAsc => 4,
            Self::PriceDesc => 5,
        }
    }
}

/// A subcategory for items, used when searching.
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum Subcategory {
    Featured,
    #[default]
    All,
    Collectibles,
    Clothing,
    BodyParts,
    Gear,
    Hats,
    Faces,
    Shirts,
    TShirts,
    Pants,
    Heads,
    Accessories,
    HairAccessories,
    FaceAccessories,
    NeckAccessories,
    ShoulderAccessories,
    FrontAccessories,
    BackAccessories,
    WaistAccessories,
    AvatarAnimations,
    Bundles,
    AnimationBundles,
    EmoteAnimations,
    CommunityCreations,
    Melee,
    Ranged,
    Explosive,
    PowerUp,
    Navigation,
    Musical,
    Social,
    Building,
    Transport,
}

impl Subcategory {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Self::Featured => 0,
            Self::All => 1,
            Self::Collectibles => 2,
            Self::Clothing => 3,
            Self::BodyParts => 4,
            Self::Gear => 5,
            Self::Hats => 9,
            Self::Faces => 10,
            Self::Shirts => 12,
            Self::TShirts => 13,
            Self::Pants => 14,
            Self::Heads => 15,
            Self::Accessories => 19,
            Self::HairAccessories => 20,
            Self::FaceAccessories => 21,
            Self::NeckAccessories => 22,
            Self::ShoulderAccessories => 23,
            Self::FrontAccessories => 24,
            Self::BackAccessories => 25,
            Self::WaistAccessories => 26,
            Self::AvatarAnimations => 27,
            Self::Bundles => 37,
            Self::AnimationBundles => 38,
            Self::EmoteAnimations => 39,
            Self::CommunityCreations => 40,
            Self::Melee => 41,
            Self::Ranged => 42,
            Self::Explosive => 43,
            Self::PowerUp => 44,
            Self::Navigation => 45,
            Self::Musical => 46,
            Self::Social => 47,
            Self::Building => 48,
            Self::Transport => 49,
        }
    }
}

/// Additional details for premium pricing.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub struct PremiumPricing {
    /// The discount percentage in the form of a value from 0-100.
    #[serde(alias = "premiumDiscountPercentage")]
    pub premium_discount_percentage: u64,
    /// The price of the item for premium users.
    #[serde(alias = "premiumPriceInRobux")]
    pub premium_price_in_robux: u64,
}

/// A struct containing (mostly) all the fields possibly returned from <https://catalog.roblox.com/v1/catalog/items/details>.
///
/// Returned from [`Client::item_details`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct ItemDetails {
    /// Either the asset id, or the bundle id, depending on the [`Self::item_type`].
    pub id: u64,
    /// The type of item (Asset or Bundle).
    pub item_type: ItemType,
    /// Exclusive with [`ItemDetails::bundle_type`].
    pub asset_type: Option<AssetType>,
    /// Exclusive with [`ItemDetails::asset_type`].
    pub bundle_type: Option<BundleType>,
    /// The name of the item.
    pub name: String,
    /// The description of the item.
    pub description: String,
    /// The product id of the item. This is different from the asset/bundle id.
    /// This is most notably used when buying limiteds. Is not used for "new" limiteds.
    pub product_id: Option<u64>,
    /// Only exists if the [`ItemDetails::item_type`] is a [`ItemType::Asset`].
    pub genres: Option<Vec<Genre>>,
    /// The statuses of an item (e.g., New, Sale). Does not exist on "new" limiteds.
    pub item_statuses: Option<Vec<ItemStatus>>,
    /// The restrictions on an item (e.g., ThirteenPlus, Limited).
    ///
    /// If there are none and the item is a non-tradable limited, then the field does not exist.
    /// Otherwise, the vector exists but the length is zero.
    pub item_restrictions: Option<Vec<ItemRestriction>>,
    /// Whether the creator is verified by Roblox.
    pub creator_has_verified_badge: bool,
    /// The type of creator that created the item (User or Group).
    pub creator_type: CreatorType,
    /// The id (group or user) of the creator. The value is 1 if the creator is Roblox.
    pub creator_id: u64,
    /// The name of the creator. The value is "Roblox" if the creator is Roblox.
    pub creator_name: String,
    /// Coincides with price if the item is a non-limited,
    /// and lowest price if item is a limited.
    ///
    /// If the item is offsale, the price is 0.
    /// However, if the price is a limited and no resellers exist,
    /// then the price does not exist.
    pub price: Option<u64>,
    /// For some reason, if details for multiple items are requested from
    /// the item details endpoint, this field is not present.
    pub favorite_count: Option<u64>,
    /// Only exists if the item has a special price status.
    pub price_status: Option<PriceStatus>,
    /// Only exists if the item has special premium pricing.
    pub premium_pricing: Option<PremiumPricing>,
    /// The remaining stock of an item. Only applies to "new" limiteds.
    pub remaining_stock: Option<u64>,
    /// The total stock of an item. Only applies to "new" limiteds.
    pub total_stock: Option<u64>,
    /// The id needed to purchase a "new" limited. This replaces the
    /// product id. Although this is an id, this is a String instead of a u64.
    pub collectible_item_id: Option<String>,
}

/// Contains an item id and its type. Used as part of a parameter in [`Client::item_details`], and used as
/// part of a response in [`Client::avatar_catalog_search`].
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub struct Item {
    /// The type of the item (Asset or Bundle).
    #[serde(alias = "itemType")]
    pub item_type: ItemType,
    /// The id of the item, or of the bundle.
    /// In the [`Client::item_details`] endpoint, it acts as both, depending on the [`Self::item_type`].
    pub id: u64,
}

impl TryFrom<u64> for AssetType {
    type Error = RoboatError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(AssetType::TShirt),
            8 => Ok(AssetType::Hat),
            11 => Ok(AssetType::Shirt),
            12 => Ok(AssetType::Pants),
            17 => Ok(AssetType::Head),
            18 => Ok(AssetType::Face),
            19 => Ok(AssetType::Gear),
            25 => Ok(AssetType::Arms),
            26 => Ok(AssetType::Legs),
            27 => Ok(AssetType::Torso),
            28 => Ok(AssetType::RightArm),
            29 => Ok(AssetType::LeftArm),
            30 => Ok(AssetType::LeftLeg),
            31 => Ok(AssetType::RightLeg),
            41 => Ok(AssetType::HairAccessory),
            42 => Ok(AssetType::FaceAccessory),
            43 => Ok(AssetType::NeckAccessory),
            44 => Ok(AssetType::ShoulderAccessory),
            45 => Ok(AssetType::FrontAccessory),
            46 => Ok(AssetType::BackAccessory),
            47 => Ok(AssetType::WaistAccessory),
            48 => Ok(AssetType::ClimbAnimation),
            49 => Ok(AssetType::DeathAnimation),
            50 => Ok(AssetType::FallAnimation),
            51 => Ok(AssetType::IdleAnimation),
            52 => Ok(AssetType::JumpAnimation),
            53 => Ok(AssetType::RunAnimation),
            54 => Ok(AssetType::SwimAnimation),
            55 => Ok(AssetType::WalkAnimation),
            56 => Ok(AssetType::PoseAnimation),
            61 => Ok(AssetType::EmoteAnimation),
            _ => Err(RoboatError::MalformedResponse),
        }
    }
}

impl TryFrom<u64> for BundleType {
    type Error = RoboatError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(BundleType::BodyParts),
            2 => Ok(BundleType::AvatarAnimations),
            _ => Err(RoboatError::MalformedResponse),
        }
    }
}

impl TryFrom<request_types::ItemDetailsRaw> for ItemDetails {
    type Error = RoboatError;

    fn try_from(value: request_types::ItemDetailsRaw) -> Result<Self, Self::Error> {
        let asset_type = match value.asset_type {
            Some(asset_type_id) => {
                let asset_type = AssetType::try_from(asset_type_id)?;
                Some(asset_type)
            }
            None => None,
        };

        let bundle_type = match value.bundle_type {
            Some(bundle_type_id) => {
                let bundle_type = BundleType::try_from(bundle_type_id)?;
                Some(bundle_type)
            }
            None => None,
        };

        let id = value.id.ok_or(RoboatError::MalformedResponse)?;
        let item_type = value.item_type.ok_or(RoboatError::MalformedResponse)?;
        let name = value.name.ok_or(RoboatError::MalformedResponse)?;
        let description = value.description.ok_or(RoboatError::MalformedResponse)?;
        let product_id = value.product_id;
        let creator_type = value.creator_type.ok_or(RoboatError::MalformedResponse)?;
        let item_statuses = value.item_status;

        let item_restrictions = value.item_restrictions;

        let creator_has_verified_badge = value
            .creator_has_verified_badge
            .ok_or(RoboatError::MalformedResponse)?;

        let creator_id = value
            .creator_target_id
            .ok_or(RoboatError::MalformedResponse)?;

        let creator_name = value
            .creator_name
            .clone()
            .ok_or(RoboatError::MalformedResponse)?;

        let genres = value.genres;
        let favorite_count = value.favorite_count;
        let price_status = value.price_status;
        let premium_pricing = value.premium_pricing;

        // If the price is None, use the lowest price (used for limiteds).
        // If neither exists, the item has no resellers and the price
        // does not exist.
        let price = match value.price {
            Some(x) => Some(x),
            None => value.lowest_price,
        };

        let remaining_stock = value.units_available_for_consumption;
        let total_stock = value.total_quantity;
        let collectible_item_id = value.collectible_item_id;

        Ok(Self {
            id,
            item_type,
            asset_type,
            bundle_type,
            name,
            description,
            product_id,
            genres,
            item_statuses,
            item_restrictions,
            creator_has_verified_badge,
            creator_type,
            creator_id,
            creator_name,
            price,
            favorite_count,
            price_status,
            premium_pricing,
            remaining_stock,
            total_stock,
            collectible_item_id,
        })
    }
}

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
    /// use roboat::catalog::avatar_catalog::{ItemType, Item};
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
