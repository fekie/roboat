use crate::{Client, RoboatError};
use serde::{Deserialize, Serialize};
use std::fmt;

mod request_types;

const THUMBNAIL_API_URL: &str = "https://thumbnails.roblox.com/v1/batch";

/// A size for an asset thumbnail.
///
/// Sizes are taken from <https://thumbnails.roblox.com/docs/index.html#operations-Assets-get_v1_assets>.
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum AssetThumbnailSize {
    S30x30,
    S42x42,
    S50x50,
    S60x62,
    S75x75,
    S110x110,
    S140x140,
    S150x150,
    S160x100,
    S160x600,
    S250x250,
    S256x144,
    S300x250,
    S304x166,
    S384x216,
    S396x216,
    #[default]
    S420x420,
    S480x270,
    S512x512,
    S576x324,
    S700x700,
    S728x90,
    S768x432,
    S1200x80,
}

impl fmt::Display for AssetThumbnailSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::S30x30 => write!(f, "30x30"),
            Self::S42x42 => write!(f, "42x42"),
            Self::S50x50 => write!(f, "50x50"),
            Self::S60x62 => write!(f, "60x62"),
            Self::S75x75 => write!(f, "75x75"),
            Self::S110x110 => write!(f, "110x110"),
            Self::S140x140 => write!(f, "140x140"),
            Self::S150x150 => write!(f, "150x150"),
            Self::S160x100 => write!(f, "160x100"),
            Self::S160x600 => write!(f, "160x600"),
            Self::S250x250 => write!(f, "250x250"),
            Self::S256x144 => write!(f, "256x144"),
            Self::S300x250 => write!(f, "300x250"),
            Self::S304x166 => write!(f, "304x166"),
            Self::S384x216 => write!(f, "384x216"),
            Self::S396x216 => write!(f, "396x216"),
            Self::S420x420 => write!(f, "420x420"),
            Self::S480x270 => write!(f, "480x270"),
            Self::S512x512 => write!(f, "512x512"),
            Self::S576x324 => write!(f, "576x324"),
            Self::S700x700 => write!(f, "700x700"),
            Self::S728x90 => write!(f, "728x90"),
            Self::S768x432 => write!(f, "768x432"),
            Self::S1200x80 => write!(f, "1200x80"),
        }
    }
}

impl Client {
    /// Fetches multiple asset thumbnails of a specified size using <https://thumbnails.roblox.com/v1/batch>.
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * Can handle up to 100 asset ids at once.
    /// * Does not appear to have a rate limit.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    /// use roboat::thumbnails::AssetThumbnailSize;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let size = AssetThumbnailSize::S420x420;
    /// let asset_id_1 = 20418400;
    /// let asset_id_2 = 12660007639;
    ///
    /// let urls = client
    ///     .asset_thumbnail_url_bulk(vec![asset_id_1, asset_id_2], size)
    ///     .await?;
    ///
    /// println!("Asset {} thumbnail url: {}", asset_id_1, urls[0]);
    /// println!("Asset {} thumbnail url: {}", asset_id_2, urls[1]);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn asset_thumbnail_url_bulk(
        &self,
        asset_ids: Vec<u64>,
        size: AssetThumbnailSize,
    ) -> Result<Vec<String>, RoboatError> {
        let url = THUMBNAIL_API_URL;

        let mut json_item_requests = Vec::new();

        for asset_id in &asset_ids {
            json_item_requests.push(serde_json::json!({
                "requestId": format!("{}::Asset:{}:png:regular", asset_id, size),
                "type": "Asset",
                "targetId": asset_id,
                "token": "",
                "format": "png",
                "size": size.to_string(),
            }));
        }

        let body = serde_json::json!(json_item_requests);

        let request_result = self.reqwest_client.post(url).json(&body).send().await;

        let response = Self::validate_request_result(request_result).await?;
        let mut raw =
            Self::parse_to_raw::<request_types::AssetThumbnailUrlResponse>(response).await?;

        sort_url_datas_by_argument_order(&mut raw.data, &asset_ids);

        let mut urls = Vec::new();

        for data in raw.data {
            urls.push(data.image_url);
        }

        Ok(urls)
    }

    /// Fetches an asset thumbnail of a specified size using <https://thumbnails.roblox.com/v1/batch>.
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * Does not appear to have a rate limit.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    /// use roboat::thumbnails::AssetThumbnailSize;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let size = AssetThumbnailSize::S420x420;
    /// let asset_id = 20418400;
    ///
    /// let url = client
    ///     .asset_thumbnail_url(asset_id, size)
    ///     .await?;
    ///
    /// println!("Asset {} thumbnail url: {}", asset_id, url);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn asset_thumbnail_url(
        &self,
        asset_id: u64,
        size: AssetThumbnailSize,
    ) -> Result<String, RoboatError> {
        let urls = self.asset_thumbnail_url_bulk(vec![asset_id], size).await?;
        let url = urls.get(0).ok_or(RoboatError::MalformedResponse)?;
        Ok(url.to_owned())
    }

    /// Fetches multiple avatar thumbnails of a specified size using <https://thumbnails.roblox.com/v1/batch>.
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * Can handle up to 100 asset ids at once.
    /// * Does not appear to have a rate limit.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    /// use roboat::thumbnails::AssetThumbnailSize;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let size = AssetThumbnailSize::S420x420;
    /// let avatar_id_1 = 20418400;
    /// let avatar_id_2 = 12660007639;
    ///
    /// let urls = client
    ///     .avatar_thumbnail_url_bulk(vec![avatar_id_1, avatar_id_2], size)
    ///     .await?;
    ///
    /// println!("Avatar {} thumbnail url: {}", avatar_id_1, urls[0]);
    /// println!("Avatar {} thumbnail url: {}", avatar_id_2, urls[1]);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn avatar_thumbnail_url_bulk(
        &self,
        player_ids: Vec<u64>,
        size: AssetThumbnailSize,
    ) -> Result<Vec<String>, RoboatError> {
        let url = THUMBNAIL_API_URL;

        let mut json_item_requests = Vec::new();

        for avatar_id in &player_ids {
            json_item_requests.push(serde_json::json!({
                "requestId": format!("{}:undefined:Avatar:{}:null:regular", avatar_id, size),
                "type": "Avatar",
                "targetId": avatar_id,
                "format": None::<String>,
                "size": size.to_string(),
            }));
        }

        let body = serde_json::json!(json_item_requests);

        let request_result = self.reqwest_client.post(url).json(&body).send().await;

        let response = Self::validate_request_result(request_result).await?;
        let mut raw =
            Self::parse_to_raw::<request_types::AssetThumbnailUrlResponse>(response).await?;

        sort_url_datas_by_argument_order(&mut raw.data, &player_ids);

        let mut urls = Vec::new();

        for data in raw.data {
            urls.push(data.image_url);
        }

        Ok(urls)
    }

    /// Fetches an avatar thumbnail of a specified size using <https://thumbnails.roblox.com/v1/batch>.
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * Does not appear to have a rate limit.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    /// use roboat::thumbnails::AssetThumbnailSize;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let size = AssetThumbnailSize::S420x420;
    /// let avatar_id = 20418400;
    ///
    /// let url = client
    ///     .asset_thumbnail_url(avatar_id, size)
    ///     .await?;
    ///
    /// println!("Avatar {} thumbnail url: {}", avatar_id, url);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn avatar_thumbnail_url(
        &self,
        player_id: u64,
        size: AssetThumbnailSize,
    ) -> Result<String, RoboatError> {
        let urls = self
            .avatar_thumbnail_url_bulk(vec![player_id], size)
            .await?;
        let url = urls.get(0).ok_or(RoboatError::MalformedResponse)?;
        Ok(url.to_owned())
    }
}

/// Makes sure that the url datas are in the same order as the arguments.
fn sort_url_datas_by_argument_order(
    url_datas: &mut [request_types::AssetThumbnailUrlDataRaw],
    arguments: &[u64],
) {
    url_datas.sort_by(|a, b| {
        let a_index = arguments
            .iter()
            .position(|id| *id == a.target_id as u64)
            .unwrap_or(usize::MAX);

        let b_index = arguments
            .iter()
            .position(|id| *id == b.target_id as u64)
            .unwrap_or(usize::MAX);

        a_index.cmp(&b_index)
    });
}
