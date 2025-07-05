// mod request_types;
#![allow(missing_docs)]

const ASSETDELIVERY_ASSET_API: &str = "https://assetdelivery.roblox.com/v1/asset/?ID={id}";
const ASSETDELIVERY_V2_API: &str = "https://assetdelivery.roblox.com/v2";

use crate::catalog::AssetType;
use crate::{Client, RoboatError, XCSRF_HEADER};
use bytes::Bytes;
use reqwest::header;
use serde_with::skip_serializing_none;

/// All the payload/response structs
pub mod request_types;

use serde::Deserialize;
use serde::Serialize;

//https://create.roblox.com/docs/cloud/legacy/assetdelivery/v2#/AssetFetchV2/get_v2_assetId__assetId_
/// Structs For MetaData /v2/assetId/{assetId} Endpoint
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetIdResponse {
    pub errors: Option<Vec<request_types::RobloxError>>,
    pub locations: Vec<request_types::AssetLocation>,
    pub request_id: String,
    #[serde(rename = "IsHashDynamic")]
    pub is_hash_dynamic: bool,
    #[serde(rename = "IsCopyrightProtected")]
    pub is_copyright_protected: bool,
    pub is_archived: bool,
    pub asset_type_id: u64,
    pub is_recordable: bool,
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
    pub errors: Option<Vec<request_types::RobloxError>>,
    pub locations: Option<Vec<request_types::Location>>,
    pub request_id: Option<String>,
    pub is_hash_dynamic: Option<bool>,
    pub is_copyright_protected: Option<bool>,
    pub is_archived: Option<bool>,
    pub asset_type_id: Option<u8>,
    pub asset_type: Option<AssetType>,
    pub content_representation_specifier: Option<request_types::ContentRepresentationSpecifier>,
    pub is_recordable: Option<bool>,
}
/// WARNING: Some AssetDelivery V2 API returns Errors even when its status code 200
impl Client {
    /// Gets Meta data from item, this works on Animations, outfits, places and other assets.
    /// This will return <AssetMetaData> which has asset_type_id and the download for the file.
    /// Also more information like if the item is deleted.
    ///
    /// # Notes
    /// Requires valid roblosecurity
    /// *Can return a sucess but still have error codes in the response
    /// Doesn't need xcrf, but will add one if it gets 401

    pub async fn fetch_asset_metadata(
        &self,
        asset_id: u64,
    ) -> Result<AssetIdResponse, RoboatError> {
        match self.fetch_asset_metadata_internal(asset_id).await {
            Ok(x) => Ok(x),
            Err(e) => match e {
                RoboatError::InvalidXcsrf(new_xcsrf) => {
                    self.set_xcsrf(new_xcsrf).await;
                    self.fetch_asset_metadata_internal(asset_id).await
                }
                _ => Err(e),
            },
        }
    }

    /// Sends a batch request to fetch metadata for multiple assets.
    ///
    /// This method accepts a vector of `AssetBatchPayload` items, sends them
    /// to the Roblox AssetDelivery v2 batch API endpoint, and returns a vector
    /// of `AssetBatchResponse` objects corresponding to each requested asset.
    ///
    /// # Parameters
    /// - `asset_batch`: A vector of `AssetBatchPayload` structs representing
    ///   the assets for which metadata is requested.
    ///
    /// # Notes
    /// Needs Roblox Cookie but not CSRF
    /// Can return a sucess but still have error codes in the response
    ///     
    /// # Returns
    /// Returns a `Result` containing a vector of `AssetBatchResponse` on success,
    /// or a `RoboatError` if the request fails or the response is malformed.
    ///
    /// # Behavior
    /// - Automatically handles `InvalidXcsrf` errors by refreshing the X-CSRF token
    ///   and retrying the request once.
    /// - Requires a valid `.ROBLOSECURITY` cookie set in the `Client`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use clap::Parser;
    /// use roboat::assetdelivery::request_types::AssetBatchPayload;
    ///
    /// #[derive(Parser, Debug)]
    /// struct Args {
    ///     #[arg(long, short)]
    ///     roblosecurity: String,
    /// }
    ///
    /// use roboat::ClientBuilder;
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let args = Args::parse();
    ///     let client = ClientBuilder::new()
    ///         .roblosecurity(args.roblosecurity)
    ///         .build();
    ///     let payloads = vec![AssetBatchPayload {
    ///         asset_id: Some("105277031944789".to_string()),
    ///         request_id: Some("0".to_string()),
    ///         ..Default::default()
    ///     }];
    ///     let responses = client.post_asset_metadata_batch(payloads).await?;
    ///     for response in responses {
    ///         println!("Response for request {:?}", response);
    ///     }
    ///     Ok(())
    ///}
    /// ```
    ///
    /// # Errors
    /// This function will return errors including but not limited to:
    /// - Network or HTTP errors from the reqwest client
    /// - Invalid or expired X-CSRF tokens (which it will attempt to refresh automatically)
    /// - Malformed responses from the API

    pub async fn post_asset_metadata_batch(
        &self,
        asset_batch: Vec<AssetBatchPayload>,
    ) -> Result<Vec<AssetBatchResponse>, RoboatError> {
        match self
            .post_asset_metadata_batch_internal(asset_batch.clone())
            .await
        {
            Ok(x) => Ok(x),
            Err(e) => match e {
                RoboatError::InvalidXcsrf(new_xcsrf) => {
                    self.set_xcsrf(new_xcsrf).await;
                    self.post_asset_metadata_batch_internal(asset_batch).await
                }
                _ => Err(e),
            },
        }
    }

    /// Downloads a raw asset bytes using the endpoint <https://assetdelivery.roblox.com/v1/asset/?id={id}>.
    ///
    /// # Notes
    /// * Requires Gzip feature on reqwest for automatic decompression of the data.
    /// * Requires a valid `.ROBLOSECURITY` cookie for private or restricted assets.
    /// * Returns the raw binary content of the asset (e.g., `.rbxm`, `.rbxmx`, etc.).
    /// * The use case for this API, would be download Animation, Models and outfits.
    /// * very usefully for studio projects
    ///
    /// # Errors
    /// * [Standard Errors](#standard-errors)
    /// * [Auth Required Errors](#auth-required-errors)
    ///
    /// # Example
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "your_cookie";
    /// const ASSET_ID: u64 = 12345678;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new()
    ///     .roblosecurity(ROBLOSECURITY.to_string())
    ///     .build();
    ///
    /// let data = client.fetch_asset_data(ASSET_ID).await?;
    /// println!("Downloaded {} bytes", data.len());
    /// # Ok(())
    /// # }
    /// ```

    // WARNING: Theres a V2 API https://assetdelivery.roblox.com/v2/assetid/119472671657225 that
    // has location of the file. Migrate to it if they ever ratelimit/remove the v1 API
    /// If this API hangs, use a timeout and retry.
    pub async fn fetch_asset_data(&self, asset_id: u64) -> Result<Bytes, RoboatError> {
        let cookie_string = self.cookie_string()?;
        let formatted_url = ASSETDELIVERY_ASSET_API.replace("{id}", &asset_id.to_string());

        let xcsrf = self.xcsrf().await;

        let response_result = self
            .reqwest_client
            .get(&formatted_url)
            .header(header::COOKIE, cookie_string)
            .header(XCSRF_HEADER, xcsrf)
            .send()
            .await;

        let response = Self::validate_request_result(response_result).await?;

        let bytes = response.bytes().await.map_err(RoboatError::ReqwestError)?;
        Ok(bytes)
    }
}

mod internal {
    use crate::assetdelivery::AssetBatchPayload;
    use crate::assetdelivery::AssetBatchResponse;
    use crate::assetdelivery::AssetIdResponse;
    use crate::assetdelivery::ASSETDELIVERY_V2_API;
    use crate::catalog::catalog_types;
    use crate::{Client, RoboatError};
    use reqwest::header;

    impl Client {
        pub(super) async fn post_asset_metadata_batch_internal(
            &self,
            asset_payload: Vec<AssetBatchPayload>,
        ) -> Result<Vec<AssetBatchResponse>, RoboatError> {
            let cookie = self.cookie_string()?;
            let formatted_url = format!("{ASSETDELIVERY_V2_API}/assets/batch");

            let request_result = self
                .reqwest_client
                .post(formatted_url)
                .header(header::COOKIE, cookie)
                .json(&asset_payload)
                .send()
                .await;

            let response = Self::validate_request_result(request_result).await?;
            let mut meta_data = Self::parse_to_raw::<Vec<AssetBatchResponse>>(response).await?;

            // Scan response for roblox errors, if its 401 just return Invalid Cookie (Can't be
            // CSRF on this API)
            for batch_resp in &mut meta_data {
                // TODO: Convert the Asset Id type to struct
                if let Some(id) = batch_resp.asset_type_id {
                    match catalog_types::AssetType::try_from(id as u64) {
                        Ok(e) => batch_resp.asset_type = Some(e),
                        Err(..) => {}
                    }
                }
                if let Some(errors) = &batch_resp.errors {
                    for error in errors {
                        // 401 Error will be .ROBLOSECURITY. and not CSRF.
                        if error.code == 401 {
                            return Err(RoboatError::InvalidRoblosecurity);
                        }
                    }
                }
            }

            Ok(meta_data)
        }
        pub(super) async fn fetch_asset_metadata_internal(
            &self,
            asset_id: u64,
        ) -> Result<AssetIdResponse, RoboatError> {
            let cookie = self.cookie_string()?;
            // let xcsrf = self.xcsrf().await;
            let formatted_url = format!("{}/assetid/{}", ASSETDELIVERY_V2_API, asset_id);

            let request_result = self
                .reqwest_client
                .get(formatted_url)
                .header(header::COOKIE, cookie)
                .send()
                .await;

            let response = Self::validate_request_result(request_result).await?;
            let meta_data = Self::parse_to_raw::<AssetIdResponse>(response).await?;

            // Scan response for roblox errors, if its 401 just return Invalid Cookie (Can't be
            // CSRF on this API)
            if let Some(errors) = &meta_data.errors {
                for error in errors {
                    // 401 Error will be .ROBLOSECURITY. and not CSRF.
                    if error.code == 401 {
                        return Err(RoboatError::InvalidRoblosecurity);
                    } else {
                        // this API either gets one Error or Asset. Return the Err
                        // NOTE: This Error could be that the Asset is Private.
                        return Err(RoboatError::UnidentifiedStatusCode(error.code));
                    }
                }
            }

            Ok(meta_data)
        }
    }
}
