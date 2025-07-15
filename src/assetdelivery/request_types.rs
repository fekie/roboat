use serde::Deserialize;
use serde::Serialize;

//NOTE: Asset Deliver API responses are really dynamic and can miss all fields, make sure to use
//option

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
