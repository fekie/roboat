use super::Role;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RolesResponse {
    pub group_id: i64,
    pub roles: Vec<Role>,
}
