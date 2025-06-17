// mod request_types;

const ASSETDELIVERY_ASSET_API: &str = "https://assetdelivery.roblox.com/v1/asset/?ID={id}";

use crate::{Client, RoboatError, XCSRF_HEADER};
use bytes::Bytes;
use reqwest::header;

impl Client {
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
    /// let blob = client.fetch_asset_data(ASSET_ID).await?;
    /// println!("Downloaded {} bytes", blob.len());
    /// # Ok(())
    /// # }
    /// ```
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
