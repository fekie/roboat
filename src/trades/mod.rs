use crate::{Client, Limit, RoboatError};
use reqwest::header;
use serde::{Deserialize, Serialize};

mod reqwest_types;

const INBOUND_TRADES_API: &str = "https://trades.roblox.com/v1/trades/";

/// For requests related to trades, we use Descending as the sort order.
/// This is because there is hardly any use case for using a reverse sort order for trades.
const SORT_ORDER: &str = "Desc";

/// The type of the trade you want to request (Inbound, Outbound, Completed, Inactive).
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[allow(missing_docs)]
pub enum TradeType {
    Inbound,
    Outbound,
    Completed,
    #[default]
    Inactive,
}

/// The details of a Roblox trade.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct Trade {
    /// The id of the trade. Used for accepting, declining, ... trades.
    pub trade_id: u64,
    /// The details of the person you're trading with.
    pub partner: Partner,
    /// Whether one of the parties can still act on the trade.
    pub is_active: bool,
    /// The status of the trade.
    pub status: TradeStatus,
}

/// The status of a Roblox trade. [`Self::Open`] is the status for both
/// inbound and outbound trades.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[allow(missing_docs)]
pub enum TradeStatus {
    Open,
    Completed,
    Declined,
    #[default]
    Expired,
    RejectedDueToError,
}

/// The details of the account you're trading with.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Partner {
    pub user_id: u64,
    pub username: String,
    pub display_name: String,
}

impl Client {
    /// Returns a list of trades using the endpoint <https://trades.roblox.com/v1/{trade_type}>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    /// * Trades are ordered newest to oldest.
    ///
    /// # Example
    /// ```no_run
    /// use roboat::Client;
    /// use roboat::trades::TradeType;
    /// use roboat::Limit;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::with_roblosecurity("roblosecurity".to_string());
    ///
    /// let trade_type = TradeType::Inbound;
    /// let limit = Limit::Ten;
    /// let cursor = None;
    ///
    /// let trades = client.trades(trade_type, limit, cursor).await?;
    ///
    /// println!("Inbound Trade #1 Partner: {}", trades[0].partner.username);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn trades(
        &self,
        trade_type: TradeType,
        limit: Limit,
        cursor: Option<String>,
    ) -> Result<Vec<Trade>, RoboatError> {
        let limit = limit.to_u64();
        let cursor = cursor.unwrap_or_default();

        let roblosecurity = self.create_cookie_string()?;

        let trade_type_str = match trade_type {
            TradeType::Inbound => "inbound",
            TradeType::Outbound => "outbound",
            TradeType::Completed => "completed",
            TradeType::Inactive => "inactive",
        };

        let formatted_url = format!(
            "{}{}?sortOrder={}&cursor={}&limit={}",
            INBOUND_TRADES_API, trade_type_str, SORT_ORDER, cursor, limit
        );

        let request_result = self
            .reqwest_client
            .get(&formatted_url)
            .header(header::COOKIE, roblosecurity)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;
        let raw = Self::parse_to_raw::<reqwest_types::InboundTradesResponse>(response).await?;

        let mut trades = Vec::new();

        for trade in raw.data {
            let partner = Partner {
                user_id: trade.user.id as u64,
                username: trade.user.name,
                display_name: trade.user.display_name,
            };

            let trade = Trade {
                trade_id: trade.id as u64,
                partner,
                is_active: trade.is_active,
                status: trade.status,
            };

            trades.push(trade);
        }

        Ok(trades)
    }
}
