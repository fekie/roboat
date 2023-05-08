use crate::{Client, Limit, RoboatError, User};

use serde::{Deserialize, Serialize};

mod request_types;

const GROUP_ROLES_API: &str = "https://groups.roblox.com/v1/groups/{group_id}/roles";
const GROUP_ROLE_MEMBERS_API: &str =
    "https://groups.roblox.com/v1/groups/{group_id}/roles/{role_id}/users?cursor={cursor_id}&limit={limit}&sortOrder={sort_order}";
const GROUP_ROLE_MEMBERS_SORT_ORDER: &str = "Desc";

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
    /// Returns the roles of a group using <https://groups.roblox.com/v1/groups/{group_id}/roles>.
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * Returns roles in ascending order by rank.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const GROUP_ID: u64 = 1127093;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
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

    /// Returns a page of members of a group role using
    /// <https://groups.roblox.com/v1/groups/{group_id}/roles/{role_id}/users?cursor={cursor_id}&limit={limit}&sortOrder={sort_order}>.
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
    /// use roboat::{ClientBuilder, Limit};
    ///
    /// const GROUP_ID: u64 = 1127093;
    /// const ROLE_ID: u64 = 18792070;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let (members, cursor) = client
    ///     .group_role_members(GROUP_ID, ROLE_ID, Limit::Hundred, None)
    ///     .await?;
    ///
    /// for member in members {
    ///     println!(
    ///     "User ID: {} / Username: {} / Display Name: {}",
    ///     member.user_id, member.username, member.display_name
    ///     );
    /// }
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn group_role_members(
        &self,
        group_id: u64,
        role_id: u64,
        limit: Limit,
        cursor: Option<String>,
    ) -> Result<(Vec<User>, Option<String>), RoboatError> {
        let formatted_url = GROUP_ROLE_MEMBERS_API
            .replace("{group_id}", &group_id.to_string())
            .replace("{role_id}", &role_id.to_string())
            .replace("{cursor_id}", &cursor.unwrap_or_default())
            .replace("{limit}", &limit.to_u64().to_string())
            .replace("{sort_order}", GROUP_ROLE_MEMBERS_SORT_ORDER);

        let request_result = self.reqwest_client.get(formatted_url).send().await;

        let response = Self::validate_request_result(request_result).await?;
        let raw = Self::parse_to_raw::<request_types::RoleMembersResponse>(response).await?;

        let mut users = Vec::new();

        for member in raw.data {
            users.push(User {
                user_id: member.user_id,
                username: member.username,
                display_name: member.display_name,
            });
        }

        let next_cursor = raw.next_page_cursor;

        Ok((users, next_cursor))
    }
}
