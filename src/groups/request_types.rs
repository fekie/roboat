use super::Role;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RolesResponse {
    pub group_id: i64,
    pub roles: Vec<Role>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RoleMembersResponse {
    pub previous_page_cursor: Option<String>,
    pub next_page_cursor: Option<String>,
    pub data: Vec<MemberRaw>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct MemberRaw {
    pub has_verified_badge: bool,
    pub user_id: u64,
    pub username: String,
    pub display_name: String,
}
