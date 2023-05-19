use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct AssetThumbnailUrlResponse {
    pub data: Vec<AssetThumbnailUrlDataRaw>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct AssetThumbnailUrlDataRaw {
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "errorCode")]
    pub error_code: i64,
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    #[serde(rename = "targetId")]
    pub target_id: i64,
    pub state: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
}
