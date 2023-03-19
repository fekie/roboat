/* let result = params.client_roblosec_pairs[params.pair_index]
.client
.get(format!(
    "https://economy.roblox.com/v1/assets/{}/resellers?cursor=&limit=10",
    params.watched_item.id,
))
.header(
    header::COOKIE,
    format!(
        ".ROBLOSECURITY={}",
        params.client_roblosec_pairs[params.pair_index].roblosec
    ),
)
.header(
    header::USER_AGENT,
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:101.0) Gecko/20100101 Firefox/101.0",
)
.header(header::CACHE_CONTROL, "no-store no-cache")
.header(header::CONNECTION, "keep-alive")
.header(header::CONTENT_TYPE, "application/json;charset=utf-8")
.send()
.await; */

pub enum ResellerAmount {}

mod external {
    use crate::{Client, Error};

    impl Client {
        /// Grabs a list of resellers from <https://economy.roblox.com/v1/assets/xxx/resellers>.
        pub async fn resellers(&self, item_id: u64) -> Result<u64, Error> {
            // Does not require xcsrf.

            self.resellers_internal(item_id).await
        }
    }
}

use crate::{Client, Error};

impl Client {
    async fn resellers_internal(&self, item_id: u64) -> Result<u64, Error> {
        todo!()
    }
}
