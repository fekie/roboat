use super::{
    CreatorType, Genre, ItemParameters, ItemRestriction, ItemStatus, ItemType, PremiumPricing,
    PriceStatus,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ItemDetailsResponse {
    pub data: Vec<ItemDetailsRaw>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ItemDetailsRaw {
    pub id: Option<u64>,
    #[serde(rename = "itemType")]
    pub item_type: Option<ItemType>,
    #[serde(rename = "bundleType")]
    pub bundle_type: Option<u64>,
    #[serde(rename = "assetType")]
    pub asset_type: Option<u64>,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "productId")]
    pub product_id: Option<u64>,
    pub genres: Option<Vec<Genre>>,
    #[serde(rename = "itemStatus")]
    pub item_statuses: Option<Vec<ItemStatus>>,
    #[serde(rename = "itemRestrictions")]
    pub item_restrictions: Option<Vec<ItemRestriction>>,
    #[serde(rename = "creatorHasVerifiedBadge")]
    pub creator_has_verified_badge: Option<bool>,
    #[serde(rename = "creatorType")]
    pub creator_type: Option<CreatorType>,
    #[serde(rename = "creatorTargetId")]
    pub creator_user_id: Option<u64>,
    #[serde(rename = "creatorName")]
    pub creator_name: Option<String>,
    /// Exists instead of lowest_price if the item is non-limited.
    pub price: Option<u64>,
    /// Exists instead of price if the item is limited.
    #[serde(rename = "lowestPrice")]
    pub lowest_price: Option<u64>,
    #[serde(rename = "favoriteCount")]
    pub favorite_count: Option<u64>,
    #[serde(rename = "premiumPricing")]
    pub premium_pricing: Option<PremiumPricing>,
    #[serde(rename = "priceStatus")]
    pub price_status: Option<PriceStatus>,
    /// It is unknown as to what type this value is.
    /// The farthest it can be tracked by reverse engineering is that the value
    /// is fed into a `new Date()` constructor in js.
    ///
    /// Because of this, it is not included in the public struct until
    #[serde(rename = "offSaleDeadline")]
    pub off_sale_deadline: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ItemDetailsReqBody {
    pub(crate) items: Vec<ItemParametersReq>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ItemParametersReq {
    #[serde(rename = "itemType")]
    pub item_type: ItemType,
    pub id: u64,
}

impl From<ItemParameters> for ItemParametersReq {
    fn from(item: ItemParameters) -> Self {
        Self {
            item_type: item.item_type,
            id: item.id,
        }
    }
}
