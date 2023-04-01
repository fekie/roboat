use crate::{Client, Limit, RoboatError, ROBLOSECURITY_COOKIE_STR};
use reqwest::header;
use serde::{Deserialize, Serialize};

mod reqwest_types;

const ROBUX_API_PART_1: &str = "https://economy.roblox.com/v1/users/";
const ROBUX_API_PART_2: &str = "/currency";

const RESELLERS_API_PART_1: &str = "https://economy.roblox.com/v1/assets/";
const RESELLERS_API_PART_2: &str = "/resellers";

const TRANSACTIONS_API_PART_1: &str = "https://economy.roblox.com/v2/users/";
const TRANSACTIONS_API_PART_2: &str = "/transactions";

const USER_SALES_TRANSACTION_TYPE: &str = "Sale";

/// A reseller of a resale listing.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct Reseller {
    pub user_id: u64,
    pub name: String,
}

/// A resale listing of a limited item.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct Listing {
    /// The unique asset id of the item.
    pub uaid: u64,
    /// The price of the listing.
    pub price: u64,
    /// The reseller of the listing.
    pub reseller: Reseller,
    /// The serial number of the item. This is separate from the uaid and only
    /// exists for Limited U Items.
    pub serial_number: Option<u64>,
}

/// A sale of an asset from the user's transaction history. Retrieved from <https://economy.roblox.com/v2/users/{user_id}/transactions?transactionType=Sale>.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct UserSale {
    /// These appear to be generated in sequential order and appear to be
    /// only related to Sales.
    pub sale_id: u64,
    /// Whether the sale is still pending
    pub is_pending: bool,
    /// The id if the user that purchased the asset.
    pub user_id: u64,
    /// The display name of the user that purchased the asset.
    pub user_display_name: String,
    /// The robux the user received after tax. Note that it's not certain that every
    /// type of item has a 30% tax, so the value is left as-is. To convert this to a price
    /// that the item sold at (assuming 30% tax), use `robux_received * 1.428`.
    pub robux_received: u64,
    /// The asset id of the item that was sold.
    pub asset_id: u64,
    /// The name of the asset that was sold.
    pub asset_name: String,
}

impl Client {
    /// Grabs robux count of the current account from <https://economy.roblox.com/v1/users/{user_id}/currency>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    ///
    /// # Example
    /// ```no_run
    /// use roboat::Client;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new();
    /// client.set_roblosecurity("my_roblosecurity".to_string());
    ///
    /// let robux = client.robux().await?;
    /// println!("Robux: {}", robux);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn robux(&self) -> Result<u64, RoboatError> {
        let user_id = self.user_id().await?;
        let formatted_url = format!("{}{}{}", ROBUX_API_PART_1, user_id, ROBUX_API_PART_2);
        let roblosecurity = match self.roblosecurity() {
            Some(roblosecurity) => roblosecurity,
            None => return Err(RoboatError::RoblosecurityNotSet),
        };
        let cookie = format!("{}={}", ROBLOSECURITY_COOKIE_STR, roblosecurity);

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .header(header::COOKIE, cookie)
            .send()
            .await;

        match request_result {
            Ok(response) => {
                let status_code = response.status().as_u16();

                match status_code {
                    200 => {
                        let raw = match response.json::<reqwest_types::CurrencyResponse>().await {
                            Ok(x) => x,
                            Err(_) => return Err(RoboatError::MalformedResponse),
                        };
                        let robux = raw.robux;
                        Ok(robux)
                    }
                    400 => Err(RoboatError::BadRequest),
                    401 => Err(RoboatError::InvalidRoblosecurity),
                    429 => Err(RoboatError::TooManyRequests),
                    500 => Err(RoboatError::InternalServerError),
                    _ => Err(RoboatError::UnidentifiedStatusCode(status_code)),
                }
            }
            Err(e) => Err(RoboatError::ReqwestError(e)),
        }
    }

    /// Grabs resellers of an item from <https://economy.roblox.com/v1/assets/{item_id}/resellers?cursor={cursor}&limit={limit}>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    ///
    /// # Argument Notes
    /// * The cursor is used to get the a certain page of results. If you want the starting page, use `None`.
    /// * The default `limit` is [`Limit::Ten`].
    ///
    /// # Return Value Notes
    /// * The first value is a vector of reseller listings.
    /// * The second value is the cursor for the next page of results. If there are no more pages, this will be `None`.
    ///
    /// # Example
    /// ```no_run
    /// use roboat::Limit;
    /// use roboat::Client;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new();
    /// client.set_roblosecurity("my_roblosecurity".to_string());
    ///
    /// let item_id = 1365767;
    /// let limit = Limit::Ten;
    /// let cursor = None;
    ///
    /// let (resellers, next_page_cursor) = client.resellers(item_id, limit, cursor).await?;
    /// println!("Lowest Price for Item {}: {}", item_id, resellers[0].price);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn resellers(
        &self,
        item_id: u64,
        limit: Limit,
        cursor: Option<String>,
    ) -> Result<(Vec<Listing>, Option<String>), RoboatError> {
        let limit = limit.to_u64();
        let cursor = cursor.unwrap_or_default();
        let roblosecurity = match self.roblosecurity() {
            Some(roblosecurity) => roblosecurity,
            None => return Err(RoboatError::RoblosecurityNotSet),
        };

        let formatted_url = format!(
            "{}{}{}?cursor={}&limit={}",
            RESELLERS_API_PART_1, item_id, RESELLERS_API_PART_2, cursor, limit
        );
        let cookie = format!("{}={}", ROBLOSECURITY_COOKIE_STR, roblosecurity);

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .header(header::COOKIE, cookie)
            .send()
            .await;

        match request_result {
            Ok(response) => {
                let status_code = response.status().as_u16();

                match status_code {
                    200 => {
                        let raw = match response.json::<reqwest_types::ResellersResponse>().await {
                            Ok(x) => x,
                            Err(_) => return Err(RoboatError::MalformedResponse),
                        };

                        let next_page_cursor = raw.next_page_cursor;

                        let mut listings = Vec::new();

                        for listing in raw.data {
                            let reseller = Reseller {
                                user_id: listing.seller.id,
                                name: listing.seller.name,
                            };

                            let listing = Listing {
                                uaid: listing.user_asset_id,
                                price: listing.price,
                                reseller,
                                serial_number: listing.serial_number,
                            };

                            listings.push(listing);
                        }

                        Ok((listings, next_page_cursor))
                    }
                    400 => Err(RoboatError::BadRequest),
                    401 => Err(RoboatError::InvalidRoblosecurity),
                    429 => Err(RoboatError::TooManyRequests),
                    500 => Err(RoboatError::InternalServerError),
                    _ => Err(RoboatError::UnidentifiedStatusCode(status_code)),
                }
            }
            Err(e) => Err(RoboatError::ReqwestError(e)),
        }
    }

    /// Grabs user sales from <https://economy.roblox.com/v2/users/{user_id}/transactions?transactionType=Sale&cursor={cursor}&limit={limit}>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    ///
    /// # Argument Notes
    /// * The cursor is used to get the a certain page of results. If you want the starting page, use `None`.
    /// * The default `limit` is [`Limit::Hundred`].
    ///
    /// # Return Value Notes
    /// * The first value is a vector of user sales.
    /// * The second value is the cursor for the next page of results. If there are no more pages, this will be `None`.
    ///
    /// # Example
    /// ```no_run
    /// use roboat::Limit;
    /// use roboat::Client;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new();
    /// client.set_roblosecurity("my_roblosecurity".to_string());
    ///
    /// let limit = Limit::Ten;
    /// let cursor = None;
    ///
    /// let (user_sales, next_page_cursor) = client.user_sales(limit, cursor).await?;
    ///
    /// let sale_amount = user_sales.len();
    /// let total_robux_earned = user_sales
    ///     .iter()
    ///     .map(|sale| sale.robux_received)
    ///     .sum::<u64>();
    ///
    /// println!("Robux gained from last {} sales: {}", sale_amount, total_robux_earned);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn user_sales(
        &self,
        limit: Limit,
        cursor: Option<String>,
    ) -> Result<(Vec<UserSale>, Option<String>), RoboatError> {
        let limit = limit.to_u64();
        let cursor = cursor.unwrap_or_default();

        let user_id = self.user_id().await?;
        let roblosecurity = match self.roblosecurity() {
            Some(roblosecurity) => roblosecurity,
            None => return Err(RoboatError::RoblosecurityNotSet),
        };

        let formatted_url = format!(
            "{}{}{}?cursor={}&limit={}&transactionType={}",
            TRANSACTIONS_API_PART_1,
            user_id,
            TRANSACTIONS_API_PART_2,
            cursor,
            limit,
            USER_SALES_TRANSACTION_TYPE
        );

        let cookie = format!("{}={}", ROBLOSECURITY_COOKIE_STR, roblosecurity);

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .header(header::COOKIE, cookie)
            .send()
            .await;

        match request_result {
            Ok(response) => {
                let status_code = response.status().as_u16();

                match status_code {
                    200 => {
                        let raw = match response.json::<reqwest_types::UserSalesResponse>().await {
                            Ok(x) => x,
                            Err(e) => {
                                dbg!(e);
                                return Err(RoboatError::MalformedResponse);
                            }
                        };

                        let next_page_cursor = raw.next_page_cursor;

                        let mut sales = Vec::new();

                        for raw_sale in raw.data {
                            let sale_id = raw_sale.sale_id;
                            let asset_id = raw_sale.details.id;
                            let robux_received = raw_sale.currency.amount;
                            let is_pending = raw_sale.is_pending;
                            let user_id = raw_sale.user.id;
                            let user_display_name = raw_sale.user.user_display_name;
                            let asset_name = raw_sale.details.item_name;

                            let sale = UserSale {
                                sale_id,
                                asset_id,
                                robux_received,
                                is_pending,
                                user_id,
                                user_display_name,
                                asset_name,
                            };

                            sales.push(sale);
                        }

                        Ok((sales, next_page_cursor))
                    }
                    400 => Err(RoboatError::BadRequest),
                    401 => Err(RoboatError::InvalidRoblosecurity),
                    429 => Err(RoboatError::TooManyRequests),
                    500 => Err(RoboatError::InternalServerError),
                    _ => Err(RoboatError::UnidentifiedStatusCode(status_code)),
                }
            }
            Err(e) => Err(RoboatError::ReqwestError(e)),
        }
    }
}
