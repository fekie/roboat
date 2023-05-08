use crate::{Client, RoboatError};
use serde::{Deserialize, Serialize};

mod request_types;

const GROUP_ROLES_API: &str = "https://groups.roblox.com/v1/groups/{group_id}/roles";

/// A role in a group.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    /// The ID of the role.
    pub id: u64,
    /// The name of the role.
    pub name: String,
    /// A number from 0 to 255 that determines the role's rank, with
    /// 255 being the highest rank and 0 being the lowest rank.
    pub rank: u8,
    /// The number of members in the role.
    pub member_count: u64,
}

impl Client {
    /// In order by rank starting from lowest rank.
    ///
    /// Returns the roles of a group using <https://groups.roblox.com/v1/groups/{group_id}/roles>.
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    /// const GROUP_ID: u64 = 1127093;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// let roles = client.group_roles(GROUP_ID).await?;
    ///
    /// // Print all roles in order by rank
    /// for role in roles {
    ///     println!(
    ///        "Role: {} / ID: {} / Rank: {}",
    ///        role.name, role.id, role.rank
    ///    );
    ///  }
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn group_roles(&self, group_id: u64) -> Result<Vec<Role>, RoboatError> {
        let formatted_url = GROUP_ROLES_API.replace("{group_id}", &group_id.to_string());

        let request_result = self.reqwest_client.get(formatted_url).send().await;

        let response = Self::validate_request_result(request_result).await?;
        let raw = Self::parse_to_raw::<request_types::RolesResponse>(response).await?;

        let mut roles = raw.roles;

        // Enforce that the roles are in order by rank in ascending order
        roles.sort_by(|a, b| a.rank.cmp(&b.rank));

        Ok(roles)
    }
}
