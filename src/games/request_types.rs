use serde::{Deserialize, Serialize};

use crate::catalog::{AssetType, CreatorType};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CreatorInformation {
    pub id: u64,
    #[serde(rename = "type")]
    pub creator_type: CreatorType,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RootPlaceInformation {
    pub id: u64,
    #[serde(rename = "type")]
    pub root_place_type: AssetType,
}
