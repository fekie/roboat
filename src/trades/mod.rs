use crate::{Client, Limit, RoboatError, User};
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

mod request_types;

const TRADES_API: &str = "https://trades.roblox.com/v1/trades/";
const TRADE_DETAILS_API: &str = "https://trades.roblox.com/v1/trades/{trade_id}";
const DECLINE_TRADE_API: &str = "https://trades.roblox.com/v1/trades/{trade_id}/decline";
const SEND_TRADE_API: &str = "https://trades.roblox.com/v1/trades/send";
const ACCEPT_TRADE_API: &str = "https://trades.roblox.com/v1/trades/{trade_id}/accept";

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
    pub partner: User,
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

impl std::str::FromStr for TradeStatus {
    type Err = RoboatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Open" => Ok(Self::Open),
            "Completed" => Ok(Self::Completed),
            "Declined" => Ok(Self::Declined),
            "Expired" => Ok(Self::Expired),
            "RejectedDueToError" => Ok(Self::RejectedDueToError),
            _ => Err(RoboatError::MalformedResponse),
        }
    }
}

/// The details of a trade.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct TradeDetails {
    /// Your partner in the trade deal.
    pub partner: User,
    /// The items you're offering.
    pub your_items: Vec<TradeItem>,
    /// The amount of robux you're offering.
    pub your_robux: u64,
    /// The items your partner is offering.
    pub partner_items: Vec<TradeItem>,
    /// The amount of robux your partner is offering.
    pub partner_robux: u64,
    /// The creation time of the trade in ISO 8601 format.
    pub created: String,
    /// The expiration time of the trade in ISO 8601 format.
    pub expiration: Option<String>,
    /// Whether one of the parties can still act on the trade.
    pub is_active: bool,
    /// The status of the trade.
    pub status: TradeStatus,
}

/// The details of an item in a trade. This is separate from other item structs
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct TradeItem {
    pub item_id: u64,
    /// The serial number of the item. Only exists for limited Us.
    pub serial_number: Option<u64>,
    /// The unique asset id of the item. This is the only item with this uaid.
    pub uaid: u64,
    pub name: String,
    /// The recent average price of the item.
    pub rap: u64,
}

impl Client {
    /// Returns a list of trades using the endpoint <https://trades.roblox.com/v1/{trade_type}>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    /// * Trades are ordered newest to oldest.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    ///
    /// # Example
    /// ```no_run
    /// use roboat::ClientBuilder;
    /// use roboat::trades::TradeType;
    /// use roboat::Limit;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// let trade_type = TradeType::Inbound;
    /// let limit = Limit::Ten;
    /// let cursor = None;
    ///
    /// let (trades, next_cursor) = client.trades(trade_type, limit, cursor).await?;
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
    ) -> Result<(Vec<Trade>, Option<String>), RoboatError> {
        let limit = limit.to_u64();
        let cursor = cursor.unwrap_or_default();

        let cookie_string = self.cookie_string()?;

        let trade_type_str = match trade_type {
            TradeType::Inbound => "inbound",
            TradeType::Outbound => "outbound",
            TradeType::Completed => "completed",
            TradeType::Inactive => "inactive",
        };

        let formatted_url = format!(
            "{}{}?sortOrder={}&cursor={}&limit={}",
            TRADES_API, trade_type_str, SORT_ORDER, cursor, limit
        );

        let request_result = self
            .reqwest_client
            .get(&formatted_url)
            .header(header::COOKIE, cookie_string)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;
        let raw = Self::parse_to_raw::<request_types::InboundTradesResponse>(response).await?;

        let next_cursor = raw.next_page_cursor;

        let mut trades = Vec::new();

        for trade in raw.data {
            let partner = User {
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

        Ok((trades, next_cursor))
    }

    /// Returns the details of a trade using <https://trades.roblox.com/v1/trades/{trade_id}>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    ///
    /// # Example
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    /// const TRADE_ID: u64 = 123456789;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// let trade_details = client.trade_details(TRADE_ID).await?;
    ///
    /// println!("Trade Details: {:#?}", trade_details);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn trade_details(&self, trade_id: u64) -> Result<TradeDetails, RoboatError> {
        let formatted_url = TRADE_DETAILS_API.replace("{trade_id}", &trade_id.to_string());
        let cookie_string = self.cookie_string()?;

        let response_result = self
            .reqwest_client
            .get(&formatted_url)
            .header(header::COOKIE, cookie_string)
            .send()
            .await;

        let response = Self::validate_request_result(response_result).await?;
        let raw = Self::parse_to_raw::<request_types::TradeDetailsResponse>(response).await?;

        let partner = User {
            user_id: raw.offers[1].user.id as u64,
            username: raw.offers[1].user.name.clone(),
            display_name: raw.offers[1].user.display_name.clone(),
        };

        let mut your_items: Vec<TradeItem> = Vec::new();

        for item in &raw.offers[0].user_assets {
            let trade_item = TradeItem {
                item_id: item.asset_id as u64,
                serial_number: item.serial_number.map(|x| x as u64),
                uaid: item.id as u64,
                name: item.name.clone(),
                rap: item.recent_average_price as u64,
            };

            your_items.push(trade_item);
        }

        let mut partner_items: Vec<TradeItem> = Vec::new();

        for item in &raw.offers[1].user_assets {
            let trade_item = TradeItem {
                item_id: item.asset_id as u64,
                serial_number: item.serial_number.map(|x| x as u64),
                uaid: item.id as u64,
                name: item.name.clone(),
                rap: item.recent_average_price as u64,
            };

            partner_items.push(trade_item);
        }

        let your_robux = raw.offers[0].robux as u64;
        let partner_robux = raw.offers[1].robux as u64;

        let created = raw.created;
        let expiration = raw.expiration;
        let is_active = raw.is_active;

        let trade_status = TradeStatus::from_str(&raw.status)?;

        let trade_details = TradeDetails {
            partner,
            your_items,
            partner_items,
            your_robux,
            partner_robux,
            created,
            expiration,
            is_active,
            status: trade_status,
        };

        Ok(trade_details)
    }

    /// Declines a trade using <https://trades.roblox.com/v1/trades/{trade_id}/decline>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    /// * Will repeat once if the x-csrf-token is invalid.
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
    /// const TRADE_ID: u64 = 123456789;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// client.decline_trade(TRADE_ID).await?;
    ///
    /// println!("Declined trade {}", TRADE_ID);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn decline_trade(&self, trade_id: u64) -> Result<(), RoboatError> {
        match self.decline_trade_internal(trade_id).await {
            Ok(x) => Ok(x),
            Err(e) => match e {
                RoboatError::InvalidXcsrf(new_xcsrf) => {
                    self.set_xcsrf(new_xcsrf).await;

                    self.decline_trade_internal(trade_id).await
                }
                _ => Err(e),
            },
        }
    }

    /// your_robux and partner robux is before tax
    ///
    /// /// Declines a trade using <https://trades.roblox.com/v1/trades/{trade_id}/decline>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    /// * Will repeat once if the x-csrf-token is invalid.
    ///
    /// # Argument Notes
    /// * `your_robux` and `partner` is before 30% tax.
    /// * Uaids are NOT item/asset ids. They are unique ids for each item.
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
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// let partner_id = 12345;
    /// let your_uaids = vec![123, 456];
    /// let your_robux = 100;
    /// let partner_uaids = vec![321, 654];
    /// let partner_robux = 0;
    ///
    /// let trade_id = client
    ///     .send_trade(
    ///         partner_id,
    ///         your_uaids,
    ///         your_robux,
    ///         partner_uaids,
    ///         partner_robux,
    ///     )
    ///     .await?;
    ///
    /// println!("Sent Trade! Trade ID: {}", trade_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_trade(
        &self,
        partner_id: u64,
        your_item_uaids: Vec<u64>,
        your_robux: u64,
        partner_item_uaids: Vec<u64>,
        partner_robux: u64,
    ) -> Result<u64, RoboatError> {
        match self
            .send_trade_internal(
                partner_id,
                your_item_uaids.clone(),
                your_robux,
                partner_item_uaids.clone(),
                partner_robux,
            )
            .await
        {
            Ok(x) => Ok(x),
            Err(e) => match e {
                RoboatError::InvalidXcsrf(new_xcsrf) => {
                    self.set_xcsrf(new_xcsrf).await;

                    self.send_trade_internal(
                        partner_id,
                        your_item_uaids,
                        your_robux,
                        partner_item_uaids,
                        partner_robux,
                    )
                    .await
                }
                _ => Err(e),
            },
        }
    }

    /// Accepts a trade using <https://trades.roblox.com/v1/trades/{trade_id}/accept>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    /// * Will repeat once if the x-csrf-token is invalid.
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
    /// const TRADE_ID: u64 = 123456789;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// client.accept_trade(TRADE_ID).await?;
    ///
    /// println!("Accepted trade {}", TRADE_ID);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn accept_trade(&self, trade_id: u64) -> Result<(), RoboatError> {
        match self.accept_trade_internal(trade_id).await {
            Ok(x) => Ok(x),
            Err(e) => match e {
                RoboatError::InvalidXcsrf(new_xcsrf) => {
                    self.set_xcsrf(new_xcsrf).await;

                    self.accept_trade_internal(trade_id).await
                }
                _ => Err(e),
            },
        }
    }
}

mod internal {
    use super::{request_types, ACCEPT_TRADE_API, DECLINE_TRADE_API, SEND_TRADE_API};
    use crate::{Client, RoboatError, XCSRF_HEADER};
    use reqwest::header;

    impl Client {
        pub(super) async fn decline_trade_internal(
            &self,
            trade_id: u64,
        ) -> Result<(), RoboatError> {
            let formatted_url = DECLINE_TRADE_API.replace("{trade_id}", &trade_id.to_string());
            let cookie_string = self.cookie_string()?;
            let xcsrf = self.xcsrf().await;

            let response_result = self
                .reqwest_client
                .post(&formatted_url)
                .header(header::COOKIE, cookie_string)
                .header(XCSRF_HEADER, xcsrf)
                .send()
                .await;

            Self::validate_request_result(response_result).await?;

            Ok(())
        }

        pub(super) async fn send_trade_internal(
            &self,
            partner_id: u64,
            your_item_uaids: Vec<u64>,
            your_robux: u64,
            partner_item_uaids: Vec<u64>,
            partner_robux: u64,
        ) -> Result<u64, RoboatError> {
            let cookie_string = self.cookie_string()?;
            let xcsrf = self.xcsrf().await;

            let user_id = self.user_id().await?;
            let user_trade_offer = request_types::SendTradeOffer {
                user_id,
                user_asset_ids: your_item_uaids,
                robux: your_robux,
            };

            let partner_trade_offer = request_types::SendTradeOffer {
                user_id: partner_id,
                user_asset_ids: partner_item_uaids,
                robux: partner_robux,
            };

            let body = request_types::SendTradeBody {
                // The partner trade offer always comes first.
                offers: vec![partner_trade_offer, user_trade_offer],
            };

            let response_result = self
                .reqwest_client
                .post(SEND_TRADE_API)
                .header(header::COOKIE, cookie_string)
                .header(XCSRF_HEADER, xcsrf)
                .json(&body)
                .send()
                .await;

            let response = Self::validate_request_result(response_result).await?;
            let raw = Self::parse_to_raw::<request_types::SendTradeResponse>(response).await?;

            Ok(raw.id)
        }

        pub(super) async fn accept_trade_internal(&self, trade_id: u64) -> Result<(), RoboatError> {
            let formatted_url = ACCEPT_TRADE_API.replace("{trade_id}", &trade_id.to_string());
            let cookie_string = self.cookie_string()?;
            let xcsrf = self.xcsrf().await;

            let response_result = self
                .reqwest_client
                .post(&formatted_url)
                .header(header::COOKIE, cookie_string)
                .header(XCSRF_HEADER, xcsrf)
                .send()
                .await;

            Self::validate_request_result(response_result).await?;

            // The response is empty, so we just return Ok(()).
            Ok(())
        }
    }
}
