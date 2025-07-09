use crate::presence::PresenceType;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPresenceResponse {
    pub user_presences: Vec<UserPresence>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPresence {
    #[serde(deserialize_with = "deserialize_presence_type")]
    pub user_presence_type: PresenceType,

    // TODO: Maybe also deserialize this instead of using string
    pub last_location: Option<String>,
    pub place_id: Option<u64>,
    pub root_place_id: Option<u64>,
    pub game_id: Option<String>,
    pub universe_id: Option<u64>,
    pub user_id: u64,
    pub last_online: Option<String>,
}

// Simple deserializer: defaults to 0 if invalid/missing
fn deserialize_presence_type<'de, D>(deserializer: D) -> Result<PresenceType, D::Error>
where
    D: Deserializer<'de>,
{
    let value = u8::deserialize(deserializer).unwrap_or(0);
    Ok(PresenceType::try_from(value).unwrap_or(PresenceType::Offline))
}
