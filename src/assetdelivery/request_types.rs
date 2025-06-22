use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

//NOTE: Asset Deliver API responses are really dynamic and can miss all fields, to show errors so
//I use option to make Rust not panic because roblox responds with nonsense.

//https://create.roblox.com/docs/cloud/legacy/assetdelivery/v2#/AssetFetchV2/get_v2_assetId__assetId_
/// Structs For MetaData /v2/assetId/{assetId} Endpoint
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetMetaData {
    pub errors: Option<Vec<RobloxError>>,
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

/// Structs for /v2/assets/batch All of these are optional to define what Asset you want information from.
//{https://create.roblox.com/docs/cloud/legacy/assetdelivery/v2#/BatchV2/post_v2_assets_batch}
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct AssetBatchPayload {
    pub asset_name: Option<String>,
    pub asset_type: Option<String>,
    pub client_insert: Option<bool>,
    pub place_id: Option<String>,
    pub request_id: Option<String>,
    pub script_insert: Option<bool>,
    pub server_place_id: Option<String>,
    pub universe_id: Option<String>,
    pub accept: Option<String>,
    pub encoding: Option<String>,
    pub hash: Option<String>,
    pub user_asset_id: Option<String>,
    pub asset_id: Option<String>,
    pub version: Option<String>,
    pub asset_version_id: Option<String>,
    pub module_place_id: Option<String>,
    pub asset_format: Option<String>,
    #[serde(rename = "roblox-assetFormat")]
    pub roblox_asset_format: Option<String>,
    pub content_representation_priority_list: Option<String>,
    pub do_not_fallback_to_baseline_representation: Option<bool>,
}

/// Response for AssetBatch
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetBatchResponse {
    pub errors: Option<Vec<RobloxError>>,

    pub locations: Option<Vec<Location>>,
    pub request_id: Option<String>,
    pub is_hash_dynamic: Option<bool>,
    pub is_copyright_protected: Option<bool>,
    pub is_archived: Option<bool>,
    pub asset_type_id: Option<u8>,
    pub content_representation_specifier: Option<ContentRepresentationSpecifier>,
    pub is_recordable: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub asset_format: Option<String>,
    pub location: Option<String>,
    pub asset_metadatas: Option<Vec<AssetMetadata>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentRepresentationSpecifier {
    pub format: Option<String>,
    pub major_version: Option<String>,
    pub fidelity: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RobloxError {
    pub code: u16,
    pub message: String,
    pub custom_error_code: Option<i32>,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct AssetError {
//     #[serde(rename = "Code")]
//     pub code: Option<u32>,
//     #[serde(rename = "Message")]
//     pub message: Option<String>,
//     #[serde(rename = "CustomErrorCode")]
//     pub custom_error_code: Option<u32>,
// }
