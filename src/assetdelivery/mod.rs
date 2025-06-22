// mod request_types;
#![allow(missing_docs)]

const ASSETDELIVERY_ASSET_API: &str = "https://assetdelivery.roblox.com/v1/asset/?ID={id}";
const ASSETDELIVERY_V2_API: &str = "https://assetdelivery.roblox.com/v2";

use crate::{
    assetdelivery::request_types::{AssetBatchPayload, AssetBatchResponse, AssetMetaData},
    Client, RoboatError, XCSRF_HEADER,
};
use bytes::Bytes;
use reqwest::header;

/// All the payload/response structs
pub mod request_types;

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

    pub async fn fetch_asset_metadata(&self, asset_id: u64) -> Result<AssetMetaData, RoboatError> {
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
    use super::request_types;
    use crate::assetdelivery::request_types::{AssetBatchPayload, AssetBatchResponse};
    use crate::assetdelivery::{AssetMetaData, ASSETDELIVERY_V2_API};
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
            let meta_data =
                Self::parse_to_raw::<Vec<request_types::AssetBatchResponse>>(response).await?;

            // Scan response for roblox errors, if its 401 just return Invalid Cookie (Can't be
            // CSRF on this API)
            for batch_resp in &meta_data {
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
        ) -> Result<AssetMetaData, RoboatError> {
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
            let meta_data = Self::parse_to_raw::<request_types::AssetMetaData>(response).await?;

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
