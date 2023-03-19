use crate::{Client, Error};
use serde::{Deserialize, Serialize};

// A useful link for the encodings for item types: https://create.roblox.com/docs/studio/catalog-api#avatar-catalog-api

/// An enum representing the overall high level type of the item (Asset or Bundle)
pub enum ItemType {
    /// An individual asset.
    Asset,
    /// A bundle (such as an animation package).
    Bundle,
}

/// An enum representing the type of the asset (hat, shirt, gear).
#[allow(missing_docs)]
pub enum AssetType {
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

// An enum representing the type of bundle (BodyParts or AvatarAnimations).
#[allow(missing_docs)]
pub enum BundleType {
    BodyParts,
    AvatarAnimations,
}

/// An enum representing the genre of an item (war, funny).
#[allow(missing_docs)]
pub enum Genre {
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

/// The status of an item (Sale, Exclusive).
#[allow(missing_docs)]
#[derive(Serialize, Deserialize)]
pub enum ItemStatus {
    New,
    Sale,
    XboxExclusive,
    AmazonExclusive,
    GooglePlayExclusive,
    IosExclusive,
    SaleTimer,
}

/// Restrictions on an item (ThirteenPlus, Limited). Not sure why limited
/// is here but I guess that's how they do it.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize)]
pub enum ItemRestrictions {
    ThirteenPlus,
    LimitedUnique,
    Limited,
    Rthro,
}

/// Type of creator that created the item (User or Group)
#[allow(missing_docs)]
pub enum CreatorType {
    User,
    Group,
}

/// The price status of an item. Only applies to items not on sale (Free, Offsale).
#[allow(missing_docs)]
pub enum PriceStatus {
    Free,
    Offsale,
    NoResellers,
}

/// The broad category of an item for use in search (Clothing, Collectables).
#[allow(missing_docs)]
pub enum Category {
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
#[allow(missing_docs)]
pub enum SortAggregation {
    PastDay,
    PastWeek,
    PastMonth,
    AllTime,
}

/// Sorting types that can be used in an item search.
#[allow(missing_docs)]
pub enum SortType {
    Relevance,
    Favorited,
    Sales,
    Updated,
    PriceAsc,
    PriceDesc,
}

/// A subcategory for items, used when searching.
#[allow(missing_docs)]
pub enum Subcategory {
    Featured,
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

#[derive(Serialize, Deserialize)]
pub struct ItemDetailsResponse {
    data: Vec<ItemDetailsRaw>,
}

#[derive(Serialize, Deserialize)]
struct ItemDetailsRaw {
    id: Option<i64>,
    #[serde(rename = "itemType")]
    item_type: Option<String>,
    #[serde(rename = "bundleType")]
    bundle_type: Option<i64>,
    name: Option<String>,
    description: Option<String>,
    #[serde(rename = "productId")]
    product_id: Option<i64>,
    #[serde(rename = "itemStatus")]
    item_status: Option<Vec<ItemStatus>>,
    #[serde(rename = "itemRestrictions")]
    item_restrictions: Option<Vec<ItemRestrictions>>,
    #[serde(rename = "creatorHasVerifiedBadge")]
    creator_has_verified_badge: Option<bool>,
    #[serde(rename = "creatorType")]
    creator_type: Option<String>,
    #[serde(rename = "creatorTargetId")]
    creator_target_id: Option<i64>,
    #[serde(rename = "creatorName")]
    creator_name: Option<String>,
    price: Option<i64>,
    #[serde(rename = "favoriteCount")]
    favorite_count: Option<i64>,
    #[serde(rename = "offSaleDeadline")]
    off_sale_deadline: Option<serde_json::Value>,
}

impl Client {
    async fn item_details_internal(&self) -> Result<u64, Error> {
        todo!()
    }
}

mod external {
    use crate::{Client, Error};

    impl Client {
        /// Grabs details of one or more items from <https://catalog.roblox.com/v1/catalog/items/details>.
        pub async fn item_details(&self) -> Result<u64, Error> {
            match self.item_details_internal().await {
                Ok(x) => Ok(x),
                Err(e) => match e {
                    Error::InvalidXcsrf(new_xcsrf) => {
                        self.set_xcsrf(new_xcsrf);

                        self.item_details_internal().await
                    }
                    _ => Err(e),
                },
            }
        }
    }
}
