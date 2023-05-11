use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct UnreadMessageCountResponse {
    pub count: u64,
}
