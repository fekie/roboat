use crate::{ide::ide_types::NewAnimation, Client, RoboatError};

/// Types for all the IDE API
pub mod ide_types;

const UPLOAD_ANIMATION_API: &str = "https://www.roblox.com/ide/publish/uploadnewanimation?assetTypeName=Animation&name={name}&description={description}&AllID=1&ispublic=False&allowComments=True&isGamesAsset=False&groupId={groupId}";

// IDE is used for private APIs like ide/uploadnewanimation and ide/places/createV2

impl Client {
    /// Uploads a new animation asset to Roblox using the internal `ide/publish/uploadnewanimation` endpoint.
    ///
    /// # Endpoint
    /// Sends a `POST` request to `https://www.roblox.com/ide/publish/uploadnewanimation`
    /// with animation metadata as query parameters and animation binary data in the body.
    ///
    /// # Notes
    /// * Requires a valid `.ROBLOSECURITY` cookie for authentication.
    /// * The animation data (`animation_data`) must be provided as binary (e.g., R15 animation XML).
    /// * If the X-CSRF token is expired or invalid, it will retry the request once with a refreshed token.
    ///
    /// # Upload Animation Query Parameters
    /// Automatically included in the request URL:
    /// * `AllID`, `ispublic`, `allowComments`, `isGamesAsset`
    /// * `assetTypeName` – Always set to `"Animation"`
    /// * `name` – The title of the animation
    /// * `description` – The description of the animation
    ///
    /// # Optional Animation Params
    /// * `groupId` – Optional group ID (if uploading to a group)
    ///
    /// # Return Value Notes
    /// * Returns `Ok(())` if the animation was uploaded successfully.
    ///
    /// # Errors
    /// * [RoboatError::MissingAuth] – If the `.ROBLOSECURITY` cookie is missing.
    /// * [RoboatError::InvalidXcsrf] – If the CSRF token needs refreshing (retry will be attempted).
    /// * [RoboatError::ReqwestError] – For any network issues.
    /// * [RoboatError::ResponseError] – If Roblox returns a failure response.
    ///
    /// # Example
    /// ```no_run
    /// use bytes::Bytes;
    /// use roboat::{ClientBuilder, ide::request_types::Animation};
    ///
    /// const ROBLOSECURITY: &str = "your_.ROBLOSECURITY_cookie";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new()
    ///     .roblosecurity(ROBLOSECURITY.to_string())
    ///     .build();
    ///
    /// let animation = Animation {
    ///     id: None,
    ///     title: "MyCoolAnimation".to_string(),
    ///     description: "A test animation created by Roboat.".to_string(),
    ///     group_id: Some(123456),
    ///     animation_data: Some(Bytes::from_static(b"<KeyframeSequence>...</KeyframeSequence>")),
    /// };
    ///
    /// client.upload_new_animation(animation).await?;
    ///
    /// println!("Successfully uploaded animation.");
    /// # Ok(())
    /// # }
    /// ```

    pub async fn upload_new_animation(
        &self,
        animation_info: NewAnimation,
    ) -> Result<(), RoboatError> {
        match self
            .upload_new_animation_internal(animation_info.clone())
            .await
        {
            Ok(()) => Ok(()),
            Err(RoboatError::InvalidXcsrf(new_xcsrf)) => {
                self.set_xcsrf(new_xcsrf).await;
                self.upload_new_animation_internal(animation_info).await
            }
            Err(e) => Err(e),
        }
    }
}

mod internal {
    use crate::{
        ide::{ide_types::NewAnimation, UPLOAD_ANIMATION_API},
        Client, RoboatError, XCSRF_HEADER,
    };
    use reqwest::header::{self, USER_AGENT};
    impl Client {
        pub(super) async fn upload_new_animation_internal(
            &self,
            animation_info: NewAnimation,
        ) -> Result<(), RoboatError> {
            let cookie = self.cookie_string()?;
            let xcsrf = self.xcsrf().await;

            let mut formatted_url = UPLOAD_ANIMATION_API
                .replace("{name}", &animation_info.name)
                .replace("{description}", &animation_info.description);

            // Add group Id
            if let Some(group_id) = animation_info.group_id {
                formatted_url = formatted_url.replace("{groupId}", &group_id.to_string());
            }

            let request_result = self
                .reqwest_client
                .post(formatted_url)
                .header(header::COOKIE, cookie)
                .body(animation_info.animation_data)
                .header(XCSRF_HEADER, xcsrf)
                .header(USER_AGENT, "Roblox/WinInet")
                .send()
                .await;

            let _ = Self::validate_request_result(request_result).await?;

            Ok(())
        }
    }
}
