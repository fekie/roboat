// mod request_types;

const ASSETDELIVERY_ASSET_API: &str = "https://assetdelivery.roblox.com/v1/asset/?ID={id}";
const ASSETDELIVERY_V2_API: &str = "https://assetdelivery.roblox.com/v2";

use crate::{assetdelivery::request_types::AssetMetaData, Client, RoboatError, XCSRF_HEADER};
use bytes::Bytes;
use reqwest::header;

mod request_types;

impl Client {
    /// Gets Meta data from item, this works on Animations, outfits, places and other assets.
    /// This will return <AssetMetaData> which has asset_type_id and the download for the file.
    /// Also more information like if the item is deleted.
    /// #Notes
    /// Requires valid roblosecurity
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
    use crate::assetdelivery::{AssetMetaData, ASSETDELIVERY_V2_API};
    use crate::{Client, RoboatError};
    use reqwest::header;

    impl Client {
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

            Ok(meta_data)
        }
    }
}
