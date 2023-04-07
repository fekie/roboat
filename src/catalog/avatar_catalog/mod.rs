use crate::{Client, RoboatError};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

mod request_types;

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

/// An enum representing the genre of an item (war, funny).
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

/// The details of an item. Retrieved from <https://catalog.roblox.com/v1/catalog/items/details>.
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
    /// This is most notably used when buying limiteds.
    pub product_id: u64,
    /// Only exists if the [`ItemDetails::item_type`] is a [`ItemType::Asset`].
    pub genres: Option<Vec<Genre>>,
    /// The statuses of an item (e.g., New, Sale)
    pub item_statuses: Vec<ItemStatus>,
    /// The restrictions on an item (e.g., ThirteenPlus, Limited).
    pub item_restrictions: Vec<ItemRestriction>,
    /// Whether the creator is verified by Roblox.
    pub creator_has_verified_badge: bool,
    /// The type of creator that created the item (User or Group).
    pub creator_type: CreatorType,
    /// The id of the creator. The value is 1 if the creator is Roblox.
    pub creator_user_id: u64,
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
}

/// Holds information used to retrieve data from the [`Client::item_details`] endpoint.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub struct ItemArgs {
    /// The type of the item (Asset or Bundle).
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
        let product_id = value.product_id.ok_or(RoboatError::MalformedResponse)?;
        let creator_type = value.creator_type.ok_or(RoboatError::MalformedResponse)?;
        let item_statuses = value.item_status.ok_or(RoboatError::MalformedResponse)?;

        let item_restrictions = value
            .item_restrictions
            .ok_or(RoboatError::MalformedResponse)?;

        let creator_has_verified_badge = value
            .creator_has_verified_badge
            .ok_or(RoboatError::MalformedResponse)?;

        let creator_user_id = value
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
            creator_user_id,
            creator_name,
            price,
            favorite_count,
            price_status,
            premium_pricing,
        })
    }
}

impl Client {
    /// Grabs details of one or more items from <https://catalog.roblox.com/v1/catalog/items/details>.
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
    /// # Example
    /// ```no_run
    /// use roboat::catalog::avatar_catalog::ItemArgs;
    /// use roboat::catalog::avatar_catalog::ItemType;
    /// use roboat::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let asset = ItemArgs {
    ///     item_type: ItemType::Asset,
    ///     id: 1365767,
    /// };
    ///
    /// let bundle = ItemArgs {
    ///    item_type: ItemType::Bundle,
    ///    id: 39,
    /// };
    ///
    /// let items = vec![asset, bundle];
    /// let details = client.item_details(items).await?;
    /// println!("Item Name: {}", details[0].name);
    /// println!("Bundle Name: {}", details[1].name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn item_details(
        &self,
        items: Vec<ItemArgs>,
    ) -> Result<Vec<ItemDetails>, RoboatError> {
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
}

mod internal {
    use super::{request_types, ItemArgs, ItemDetails, ITEM_DETAILS_API};
    use crate::XCSRF_HEADER;
    use crate::{Client, RoboatError};
    use std::convert::TryFrom;

    impl Client {
        pub(super) async fn item_details_internal(
            &self,
            items: Vec<ItemArgs>,
        ) -> Result<Vec<ItemDetails>, RoboatError> {
            let request_body = request_types::ItemDetailsReqBody {
                // Convert the ItemParameters to te reqwest ItemParametersReq
                items: items
                    .iter()
                    .map(|x| request_types::ItemArgsReq::from(*x))
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

            Ok(item_details)
        }
    }
}
