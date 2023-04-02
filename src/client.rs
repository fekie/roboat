use crate::RoboatError;
use std::sync::Mutex;

/// A client used for making requests to the Roblox API.
///
/// The client stores the roblosecurity cookie, X-CSRF-TOKEN header, and an HTTPS client to send web
/// requests.
#[derive(Debug, Default)]
pub struct Client {
    /// The cookie used for authentication.
    pub(crate) roblosecurity: Mutex<Option<String>>,
    /// The field holding the value for the X-CSRF-TOKEN header used in and returned by endpoints.
    pub(crate) xcsrf: Mutex<String>,
    /// The user id of the user. Not modifiable by user.
    pub(crate) user_id: Mutex<Option<u64>>,
    /// The username of the user. Not modifiable by user.
    pub(crate) username: Mutex<Option<String>>,
    /// The display name of the user. Not modifiable by user.
    pub(crate) display_name: Mutex<Option<String>>,
    /// A Reqwest HTTP client used to send web requests.
    pub(crate) reqwest_client: reqwest::Client,
}

impl Client {
    /// Used to interface with Roblox.com endpoints.
    ///
    /// Contains any necessary authentication and security tokens, as well as the
    /// reqwest client.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new [`Client`] with a roblosecurity already set.
    pub fn with_roblosecurity(roblosecurity: String) -> Self {
        Self {
            roblosecurity: Mutex::new(Some(roblosecurity)),
            ..Default::default()
        }
    }

    /// Creates a new [`Client`] providing a custom [`reqwest::Client`].
    /// Custom [`reqwest::Client`]s are used for configuring proxies.
    pub fn with_reqwest_client(reqwest_client: reqwest::Client) -> Self {
        Self {
            roblosecurity: Mutex::new(None),
            xcsrf: Mutex::new(String::new()),
            user_id: Mutex::new(None),
            username: Mutex::new(None),
            display_name: Mutex::new(None),
            reqwest_client,
        }
    }

    /// Returns the user id of the user. If the user id is not cached, it will be fetched from Roblox first.
    pub async fn user_id(&self) -> Result<u64, RoboatError> {
        let user_id_opt = *self.user_id.lock().unwrap();
        match user_id_opt {
            Some(user_id) => Ok(user_id),
            None => {
                let user_info = self.user_information_internal().await?;
                Ok(user_info.user_id)
            }
        }
    }

    /// Returns the username of the user. If the username is not cached, it will be fetched from Roblox first.
    pub async fn username(&self) -> Result<String, RoboatError> {
        let username_opt = self.username.lock().unwrap().clone();
        match username_opt {
            Some(username) => Ok(username),
            None => {
                let user_info = self.user_information_internal().await?;
                Ok(user_info.username)
            }
        }
    }

    /// Returns the display name of the user. If the display name is not cached, it will be fetched from Roblox first.
    pub async fn display_name(&self) -> Result<String, RoboatError> {
        let display_name_opt = self.display_name.lock().unwrap().clone();
        match display_name_opt {
            Some(display_name) => Ok(display_name),
            None => {
                let user_info = self.user_information_internal().await?;
                Ok(user_info.display_name)
            }
        }
    }

    /// Sets the Roblosecurity string for the client.
    ///
    /// # Example
    ///
    /// ```
    /// use roboat::Client;
    ///
    /// let client = Client::new();
    /// client.set_roblosecurity("my_roblosecurity".to_string());
    /// ```
    pub fn set_roblosecurity(&self, roblosecurity: String) {
        *self.roblosecurity.lock().unwrap() = Some(roblosecurity);
    }

    /// Returns a copy of the Roblosecurity stored in the client.
    /// If the Roblosecurity has not been set, [`RoboatError::RoblosecurityNotSet`] is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use roboat::Client;
    ///
    /// let client = Client::with_roblosecurity("my_roblosecurity".to_string());
    /// let roblosecurity = client.roblosecurity()?;
    /// assert_eq!(roblosecurity, "my_roblosecurity".to_string());
    /// # Ok(())
    /// # }
    /// ```
    pub fn roblosecurity(&self) -> Result<String, RoboatError> {
        match self.roblosecurity.lock().unwrap().clone() {
            Some(roblosecurity) => Ok(roblosecurity),
            None => Err(RoboatError::RoblosecurityNotSet),
        }
    }

    /// Sets the xcsrf token of the client.
    ///
    /// # Example
    ///
    /// ```
    /// use roboat::Client;
    ///
    /// let client = Client::new();
    /// client.set_xcsrf("my_xcsrf".to_string());
    /// ```
    pub fn set_xcsrf(&self, xcsrf: String) {
        *self.xcsrf.lock().unwrap() = xcsrf;
    }

    /// Returns a copy of the xcsrf stored in the client.
    ///
    /// # Example
    ///
    /// ```
    /// use roboat::Client;
    ///
    /// let client = Client::new();
    /// client.set_xcsrf("my_xcsrf".to_string());
    /// let xcsrf = client.xcsrf();
    /// assert_eq!(xcsrf, "my_xcsrf".to_string());
    /// ```
    pub fn xcsrf(&self) -> String {
        self.xcsrf.lock().unwrap().clone()
    }

    /// Creates a string for the cookie header using the roblosecurity.
    /// If the roblosecurity has not been set, [`RoboatError::RoblosecurityNotSet`] is returned.
    pub(crate) fn create_cookie_string(&self) -> Result<String, RoboatError> {
        Ok(format!(".ROBLOSECURITY={}", self.roblosecurity()?))
    }
}
