use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetMetaData {
    pub locations: Vec<AssetLocation>,
    pub request_id: String,
    #[serde(rename = "IsHashDynamic")]
    pub is_hash_dynamic: bool,
    #[serde(rename = "IsCopyrightProtected")]
    pub is_copyright_protected: bool,
    pub is_archived: bool,
    pub asset_type_id: u8,
    pub is_recordable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetLocation {
    pub asset_format: String,
    pub location: String,
    pub asset_metadatas: Vec<AssetMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetMetadata {
    pub metadata_type: u8,
    pub value: String,
}
