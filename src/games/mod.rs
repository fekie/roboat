#![allow(missing_docs)]
use crate::games::request_types::{CreatorInformation, RootPlaceInformation};
use crate::{Client, RoboatError};
use serde::{Deserialize, Serialize};

const GAMES_V2_API: &str = "https://games.roblox.com/v2";
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameInformationV2 {
    /// String for the Id. The game_id is in root_place information
    pub id: u64,

    /// Name of the place
    pub name: String,

    /// Optional String for Description
    pub description: Option<String>,

    /// Struct CreatorInformation that holds Owner Id and Type (Group or User)
    pub creator: CreatorInformation,

    /// Struct RootPlaceInformation that holds the game id and asset type
    pub root_place: RootPlaceInformation,

    /// String of the date the game was created
    pub created: String,

    /// String of the date the game was last updated
    pub updated: String,

    /// Shows how many visits the place has
    pub place_visits: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The response body thats from https://games.roblox.com/v2/users/{USERID} and games.roblox.com/v2/groups/{GROUPID}/gamesv2
pub struct GamesResponseV2 {
    pub previous_page_cursor: Option<String>,
    pub next_page_cursor: Option<String>,
    pub data: Vec<GameInformationV2>,
}

//https://games.roblox.com/v2/groups/3190902/gamesv2?cursor=&limit=100&sortOrder=Desc

mod request_types;
impl Client {
    /// Retrieves the first 50 games for a specified user_id.
    ///
    /// # Endpoint
    /// Sends a `GET` request to `https://games.roblox.com/v2/users/{user_id}/games?limit=50`
    ///
    /// # Notes
    /// * This is a public endpoint that does not require authentication.
    /// * The limit is automatically set to 50, which is the maximum allowed by this endpoint.
    /// * Returns games in descending order by default (most recent first).
    /// * Use pagination cursors in the response to fetch additional pages of results.
    ///
    /// # Parameters
    /// * `user_id` – The numeric user ID to retrieve games for
    ///
    /// # Return Value Notes
    /// * Returns `GamesResponseV2` containing the paginated list of games if successful.
    /// * The response includes pagination cursors for fetching additional results.
    /// * Each game contains detailed information including creator, root place, visit counts, and timestamps.
    ///
    /// # Errors
    /// * [RoboatError::ReqwestError] – For any network issues.
    /// * [RoboatError::ResponseError] – If Roblox returns a failure response.
    /// * [RoboatError::MalformedResponse] – If the JSON response cannot be parsed.
    ///
    /// # Example
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let user_id = 3054007;
    /// let games_response = client.user_games(user_id).await?;
    ///
    /// println!("Found {} games", games_response.data.len());
    ///
    /// for game in games_response.data {
    ///     println!("Game: {} (ID: {})", game.name, game.id);
    ///     println!("  Visits: {}", game.place_visits);
    ///     println!("  Created: {}", game.created);
    /// }
    ///
    /// // Check for more pages
    /// if let Some(next_cursor) = games_response.next_page_cursor {
    ///     println!("More results available with cursor: {}", next_cursor);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn user_games(&self, user_id: u64) -> Result<GamesResponseV2, RoboatError> {
        match self.user_games_internal(user_id).await {
            Ok(x) => Ok(x),
            Err(e) => Err(e),
        }
    }

    /// Retrieves the first 100 games for a specified group id..
    ///
    /// # Endpoint
    /// Sends a `GET` request to `https://games.roblox.com/v2/groups/{group_id}/gamesv2?limit=100`
    ///
    /// # Notes
    /// * This is a public endpoint that does not require authentication.
    /// * The limit is automatically set to 100, which is the maximum allowed by this endpoint.
    /// * Returns games in descending order by default (most recent first).
    /// * Use pagination cursors in the response to fetch additional pages of results.
    ///
    /// # Parameters
    /// * `group_id` – The numeric group ID to retrieve games for
    ///
    /// # Return Value Notes
    /// * Returns `GamesResponseV2` containing the paginated list of games if successful.
    /// * The response includes pagination cursors for fetching additional results.
    /// * Each game contains detailed information including creator, root place, visit counts, and timestamps.
    ///
    /// # Errors
    /// * [RoboatError::ReqwestError] – For any network issues.
    /// * [RoboatError::ResponseError] – If Roblox returns a failure response.
    /// * [RoboatError::MalformedResponse] – If the JSON response cannot be parsed.
    ///
    /// # Example
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let group_id = 3190902;
    /// let games_response = client.group_games(group_id).await?;
    ///
    /// println!("Found {} games for group", games_response.data.len());
    ///
    /// for game in games_response.data {
    ///     println!("Game: {} (ID: {})", game.name, game.id);
    ///     println!("  Root Place ID: {}", game.root_place.id);
    ///     println!("  Visits: {}", game.place_visits);
    ///     println!("  Last Updated: {}", game.updated);
    /// }
    ///
    /// // Check for more pages
    /// if let Some(next_cursor) = games_response.next_page_cursor {
    ///     println!("More results available with cursor: {}", next_cursor);
    /// }
    /// # Ok(())
    /// # }
    /// ``
    pub async fn group_games(&self, group_id: u64) -> Result<GamesResponseV2, RoboatError> {
        match self.group_games_interal(group_id).await {
            Ok(x) => Ok(x),
            Err(e) => Err(e),
        }
    }
}

mod internal {
    use crate::{
        games::{GamesResponseV2, GAMES_V2_API},
        Client, RoboatError,
    };

    impl Client {
        pub(super) async fn user_games_internal(
            &self,
            user_id: u64,
        ) -> Result<GamesResponseV2, RoboatError> {
            // Max limit is 50
            let formatted_url = format!("{}/users/{}/games?limit=50", GAMES_V2_API, user_id);
            let request_result = self.reqwest_client.get(formatted_url).send().await;

            let response = Self::validate_request_result(request_result).await?;
            let users_games_json = Self::parse_to_raw::<GamesResponseV2>(response).await?;
            Ok(users_games_json)
        }

        pub(super) async fn group_games_interal(
            &self,
            group_id: u64,
        ) -> Result<GamesResponseV2, RoboatError> {
            let formatted_url = format!("{}/groups/{}/gamesv2?limit=100", GAMES_V2_API, group_id);
            let request_result = self.reqwest_client.get(formatted_url).send().await;

            let response = Self::validate_request_result(request_result).await?;
            let group_games_json = Self::parse_to_raw::<GamesResponseV2>(response).await?;
            Ok(group_games_json)
        }
    }
}
