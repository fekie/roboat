use crate::users::ClientUserInformation;
use crate::RoboatError;
// We use tokio's version of rwlock so that readers to not starve writers on linux.
use tokio::sync::RwLock;

/// A client used for making requests to the Roblox API.
///
/// The client stores the roblosecurity cookie, X-CSRF-TOKEN header, and an HTTPS client to send web
/// requests. The client also caches the user id, username, and display name of the user.
///
/// Constructed using a [`ClientBuilder`].
///
/// # Construction Examples
///
/// ## Without Roblosecurity or a Custom Reqwest Client
/// ```
/// use roboat::ClientBuilder;
///
/// let client = ClientBuilder::new().build();
/// ```
///
/// ## With a Roblosecurity
/// ```
/// use roboat::ClientBuilder;
///
/// const ROBLOSECURITY: &str = "roblosecurity";
///
/// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
/// ```
///
/// ## With a Custom Reqwest Client
/// ```
/// use roboat::ClientBuilder;
///
/// let reqwest_client = reqwest::Client::new();
/// let client = ClientBuilder::new().reqwest_client(reqwest_client).build();
/// ```
///
/// ## With a Roblosecurity and a Custom Reqwest Client
/// ```
/// use roboat::ClientBuilder;
///
/// const ROBLOSECURITY: &str = "roblosecurity";
///
/// let reqwest_client = reqwest::Client::new();
/// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).reqwest_client(reqwest_client).build();
/// ```
#[derive(Debug, Default)]
pub struct Client {
    /// The cookie used for authentication.
    pub(crate) roblosecurity: Option<String>,
    /// The field holding the value for the X-CSRF-TOKEN header used in and returned by endpoints.
    pub(crate) xcsrf: RwLock<String>,
    /// Holds the user id, username, and display name of the user.
    pub(crate) user_information: RwLock<Option<ClientUserInformation>>,
    /// A Reqwest HTTP client used to send web requests.
    pub(crate) reqwest_client: reqwest::Client,
}

/// A builder used for constructing a [`Client`]. Constructed using [`ClientBuilder::new`].
#[derive(Clone, Debug, Default)]
pub struct ClientBuilder {
    roblosecurity: Option<String>,
    reqwest_client: Option<reqwest::Client>,
}

impl Client {
    /// Returns the user id of the user. If the user id is not cached, it will be fetched from Roblox first.
    ///
    /// The user id should be the only thing used to differentiate between accounts as
    /// username and display name can change.
    pub async fn user_id(&self) -> Result<u64, RoboatError> {
        let guard = self.user_information.read().await;
        let user_information_opt = &*guard;

        match user_information_opt {
            Some(user_information) => Ok(user_information.user_id),
            None => {
                // Drop the read lock as the writer lock will be requested.
                drop(guard);

                let user_info = self.user_information_internal().await?;
                Ok(user_info.user_id)
            }
        }
    }

    /// Returns the username of the user. If the username is not cached, it will be fetched from Roblox first.
    ///
    /// Username can change (although rarely). For this reason only user id should be used for differentiating accounts.
    pub async fn username(&self) -> Result<String, RoboatError> {
        let guard = self.user_information.read().await;
        let user_information_opt = &*guard;

        match user_information_opt {
            Some(user_information) => Ok(user_information.username.clone()),
            None => {
                // Drop the read lock as the writer lock will be requested.
                drop(guard);

                let user_info = self.user_information_internal().await?;
                Ok(user_info.username)
            }
        }
    }

    /// Returns the display name of the user. If the display name is not cached, it will be fetched from Roblox first.
    ///
    /// Display name can change. For this reason only user id should be used for differentiating accounts.
    pub async fn display_name(&self) -> Result<String, RoboatError> {
        let guard = self.user_information.read().await;
        let user_information_opt = &*guard;

        match user_information_opt {
            Some(user_information) => Ok(user_information.display_name.clone()),
            None => {
                // Drop the read lock as the writer lock will be requested.
                drop(guard);

                let user_info = self.user_information_internal().await?;
                Ok(user_info.display_name)
            }
        }
    }

    /// Used in [`Client::user_information_internal`]. This is implemented in the client
    /// module as we do not want other modules to have to interact with the rwlock directly.
    pub(crate) async fn set_user_information(&self, user_information: ClientUserInformation) {
        *self.user_information.write().await = Some(user_information);
    }

    /// Sets the xcsrf token of the client. Remember to .await this method.
    pub(crate) async fn set_xcsrf(&self, xcsrf: String) {
        *self.xcsrf.write().await = xcsrf;
    }

    /// Returns a copy of the xcsrf stored in the client. Remember to .await this method.
    pub(crate) async fn xcsrf(&self) -> String {
        self.xcsrf.read().await.clone()
    }

    /// Creates a string for the cookie header using the roblosecurity.
    /// If the roblosecurity has not been set, [`RoboatError::RoblosecurityNotSet`] is returned.
    pub(crate) fn create_cookie_string(&self) -> Result<String, RoboatError> {
        // We can continue to keep the reader lock as this function will never request a write lock.
        let roblosecurity_opt = &self.roblosecurity;

        match roblosecurity_opt {
            Some(roblosecurity) => Ok(format!(".ROBLOSECURITY={}", roblosecurity)),
            None => Err(RoboatError::RoblosecurityNotSet),
        }
    }
}

impl ClientBuilder {
    /// Creates a new [`ClientBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the roblosecurity for the client.
    ///
    /// # Example
    /// ```rust
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    ///
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    /// ```
    pub fn roblosecurity(mut self, roblosecurity: String) -> Self {
        self.roblosecurity = Some(roblosecurity);
        self
    }

    /// Sets the [`reqwest::Client`] for the client.
    ///
    /// # Example
    /// ```rust
    /// use roboat::ClientBuilder;
    ///
    /// let reqwest_client = reqwest::Client::new();
    /// let client = ClientBuilder::new().reqwest_client(reqwest_client).build();
    /// ```
    pub fn reqwest_client(mut self, reqwest_client: reqwest::Client) -> Self {
        self.reqwest_client = Some(reqwest_client);
        self
    }

    /// Builds the [`Client`]. This consumes the builder.
    ///
    /// # Example
    /// ```rust
    /// use roboat::ClientBuilder;
    ///
    /// let client = ClientBuilder::new().build();
    /// ```
    pub fn build(self) -> Client {
        Client {
            roblosecurity: self.roblosecurity,
            reqwest_client: self.reqwest_client.unwrap_or_default(),
            ..Default::default()
        }
    }
}
