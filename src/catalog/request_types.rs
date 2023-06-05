use super::catalog_types::{
    CreatorType, Genre, Item, ItemRestriction, ItemStatus, ItemType, PremiumPricing, PriceStatus,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct ItemDetailsResponse {
    pub data: Vec<ItemDetailsRaw>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct ItemDetailsRaw {
    pub id: Option<u64>,
    pub item_type: Option<ItemType>,
    pub bundle_type: Option<u64>,
    pub asset_type: Option<u64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub product_id: Option<u64>,
    pub genres: Option<Vec<Genre>>,
    pub bundled_items: Option<serde_json::Value>,
    pub item_status: Option<Vec<ItemStatus>>,
    pub item_restrictions: Option<Vec<ItemRestriction>>,
    pub creator_has_verified_badge: Option<bool>,
    pub creator_type: Option<CreatorType>,
    pub creator_target_id: Option<u64>,
    pub creator_name: Option<String>,
    /// Exists instead of lowest_price if the item is non-limited.
    /// This only occurs in some cases.
    pub price: Option<u64>,
    /// Exists instead of price if the item is limited.
    /// This only occurs in some cases.
    pub lowest_price: Option<u64>,
    pub lowest_resale_price: Option<u64>,
    pub favorite_count: Option<u64>,
    pub purchase_count: Option<u64>,
    pub premium_pricing: Option<PremiumPricing>,
    pub price_status: Option<PriceStatus>,
    /// It is unknown as to what type this value is.
    /// The farthest it can be tracked by reverse engineering is that the value
    /// is fed into a `new Date()` constructor in js.
    ///
    /// Because of this, it is not included in the public struct until
    /// we know what it is.
    pub off_sale_deadline: Option<serde_json::Value>,
    /// Only exists for "new" limiteds.
    pub collectible_item_id: Option<String>,
    /// Only exists for "new" limiteds.
    pub total_quantity: Option<u64>,
    pub has_resellers: Option<bool>,
    pub is_off_sale: Option<bool>,
    pub remaining_stock: Option<u64>,
    pub total_stock: Option<u64>,
    pub quantity_limit_per_user: Option<u64>,
    pub sale_location_type: Option<String>,
    /// Only exists for "new" limiteds.
    pub units_available_for_consumption: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct ItemDetailsReqBody {
    pub(crate) items: Vec<ItemReq>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct ItemReq {
    pub item_type: ItemType,
    pub id: u64,
}

impl From<Item> for ItemReq {
    fn from(item: Item) -> Self {
        Self {
            item_type: item.item_type,
            id: item.id,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) struct AvatarSearchQueryResponse {
    #[serde(alias = "previousPageCursor")]
    pub previous_page_cursor: Option<String>,
    #[serde(alias = "nextPageCursor")]
    pub next_page_cursor: Option<String>,
    #[serde(alias = "data")]
    pub items: Vec<Item>,
}
