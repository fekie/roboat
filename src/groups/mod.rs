use crate::{Client, Limit, RoboatError, User};
use serde::{Deserialize, Serialize};

mod request_types;

const GROUP_ROLES_API: &str = "https://groups.roblox.com/v1/groups/{group_id}/roles";

const GROUP_ROLE_MEMBERS_SORT_ORDER: &str = "Desc";
const GROUP_ROLE_MEMBERS_API: &str =
    "https://groups.roblox.com/v1/groups/{group_id}/roles/{role_id}/users?cursor={cursor}&limit={limit}&sortOrder={sort_order}";

const CHANGE_GROUP_MEMBER_ROLE_API: &str =
    "https://groups.roblox.com/v1/groups/{group_id}/users/{user_id}";

/// A role in a group.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct Role {
    /// The ID of the role.
    pub id: u64,
    /// The name of the role.
    pub name: String,
    /// A number from 0 to 255 that determines the role's rank, with
    /// 255 being the highest rank and 0 being the lowest rank.
    pub rank: u8,
    #[serde(alias = "memberCount")]
    /// The number of members in the role.
    pub member_count: u64,
}

impl Client {
    /// Returns the roles of a group using <https://groups.roblox.com/v1/groups/{group_id}/roles>.
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * Returns roles in ascending order by rank.
    /// * Does not appear to have a rate limit.
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
    /// <https://groups.roblox.com/v1/groups/{group_id}/roles/{role_id}/users?cursor={cursor}&limit={limit}&sortOrder=Desc>.
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
            .replace("{cursor}", &cursor.unwrap_or_default())
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

    /// Sets a group member's role by role id using <https://groups.roblox.com/v1/groups/{group_id}/users/{user_id}>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    /// * Will repeat once if the x-csrf-token is invalid.
    ///
    /// # Return Value Notes
    /// * Will return `Ok(())` if the role was successfully set.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    /// * All errors under [X-CSRF-TOKEN Required Errors](#x-csrf-token-required-errors).
    ///
    /// # Example
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    /// const user_id: u64 = 123456789;
    /// const group_id: u64 = 1127093;
    /// const role_id: u64 = 78505465;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// let _ = client.set_group_member_role(user_id, group_id, role_id).await?;
    ///
    /// println!(
    ///     "Set user {}'s role to role id {} in group {}.",
    ///     user_id, role_id, group_id
    /// );
    /// # Ok(())
    /// # }
    pub async fn set_group_member_role(
        &self,
        user_id: u64,
        group_id: u64,
        role_id: u64,
    ) -> Result<(), RoboatError> {
        match self
            .set_group_member_role_internal(user_id, group_id, role_id)
            .await
        {
            Ok(x) => Ok(x),
            Err(e) => match e {
                RoboatError::InvalidXcsrf(new_xcsrf) => {
                    self.set_xcsrf(new_xcsrf).await;

                    self.set_group_member_role_internal(user_id, group_id, role_id)
                        .await
                }
                _ => Err(e),
            },
        }
    }
}

mod internal {
    use super::CHANGE_GROUP_MEMBER_ROLE_API;
    use crate::{Client, RoboatError, XCSRF_HEADER};
    use reqwest::header;

    impl Client {
        pub(super) async fn set_group_member_role_internal(
            &self,
            user_id: u64,
            group_id: u64,
            role_id: u64,
        ) -> Result<(), RoboatError> {
            let formatted_url = CHANGE_GROUP_MEMBER_ROLE_API
                .replace("{group_id}", &group_id.to_string())
                .replace("{user_id}", &user_id.to_string());

            let cookie = self.cookie_string()?;
            let xcsrf = self.xcsrf().await;

            let json = serde_json::json!({ "roleId": role_id });

            let request_result = self
                .reqwest_client
                .patch(formatted_url)
                .json(&json)
                .header(header::COOKIE, cookie)
                .header(XCSRF_HEADER, xcsrf)
                .send()
                .await;

            let _ = Self::validate_request_result(request_result).await?;

            // If we got a status code 200, it was successful.

            Ok(())
        }
    }
}
