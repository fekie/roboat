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
    /// The user id of the user. Not modifiable by user.
    pub(crate) user_id: RwLock<Option<u64>>,
    /// The username of the user. Not modifiable by user.
    pub(crate) username: RwLock<Option<String>>,
    /// The display name of the user. Not modifiable by user.
    pub(crate) display_name: RwLock<Option<String>>,
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
        let guard = self.user_id.read().await;
        let user_id_opt = *guard;

        // Drop the read lock in case this thread grabs the writer lock later in the function.
        drop(guard);

        match user_id_opt {
            Some(user_id) => Ok(user_id),
            None => {
                let user_info = self.user_information_internal().await?;
                Ok(user_info.user_id)
            }
        }
    }

    /// Returns the username of the user. If the username is not cached, it will be fetched from Roblox first.
    ///
    /// Username can change (although rarely). For this reason only user id should be used for differentiating accounts.
    pub async fn username(&self) -> Result<String, RoboatError> {
        let guard = self.username.read().await;
        let username_opt = guard.clone();

        // Drop the read lock in case this thread grabs the writer lock later in the function.
        drop(guard);

        match username_opt {
            Some(username) => Ok(username),
            None => {
                let user_info = self.user_information_internal().await?;
                Ok(user_info.username)
            }
        }
    }

    /// Returns the display name of the user. If the display name is not cached, it will be fetched from Roblox first.
    ///
    /// Display name can change. For this reason only user id should be used for differentiating accounts.
    pub async fn display_name(&self) -> Result<String, RoboatError> {
        let guard = self.display_name.read().await;
        let display_name_opt = guard.clone();

        // Drop the read lock in case this thread grabs the writer lock later in the function.
        drop(guard);

        match display_name_opt {
            Some(display_name) => Ok(display_name),
            None => {
                let user_info = self.user_information_internal().await?;
                Ok(user_info.display_name)
            }
        }
    }

    /// Used in [`Client::user_information_internal`]. This is implemented in the client
    /// module as we do not want other modules to have to interact with the rwlock directly.
    pub(crate) async fn set_user_id(&self, user_id: u64) {
        *self.user_id.write().await = Some(user_id);
    }

    /// Used in [`Client::user_information_internal`]. This is implemented in the client
    /// module as we do not want other modules to have to interact with the rwlock directly.
    pub(crate) async fn set_username(&self, username: String) {
        *self.username.write().await = Some(username);
    }

    /// Used in [`Client::user_information_internal`]. This is implemented in the client
    /// module as we do not want other modules to have to interact with the rwlock directly.
    pub(crate) async fn set_display_name(&self, display_name: String) {
        *self.display_name.write().await = Some(display_name);
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
