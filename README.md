[![Crates.io](https://img.shields.io/crates/v/roboat.svg)](https://crates.io/crates/roboat)
[![Documentation](https://docs.rs/roboat/badge.svg)](https://docs.rs/roboat/)
[![dependency status](https://deps.rs/repo/github/chloe-woahie/roboat/status.svg)](https://deps.rs/repo/github/chloe-woahie/roboat)

[![](https://dcbadge.vercel.app/api/server/QmBEgPaFSD)](https://discord.gg/QmBEgPaFSD)

<img align="right" src="images/icon.png" height="150px" alt="roboat logo">

# roboat
A high performance interface for the Roblox API.

This library is designed to be high-performance capable, meaning that it supports proxies
and is capable of making requests in parallel.

Note that this crate is currently economy-focused, meaning that endpoints related to items and trades are currently prioritized.

# Documentation
Extensive documentation is used throughout this crate. 
All public methods in this crate are documented and have at least one corresponding example.

Documentation can be found [here](https://docs.rs/roboat/).

# Coverage
* Catalog API - [`catalog.roblox.com/*`]
    - Fetch Item Details - [`Client::item_details`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.item_details)
    - Fetch Product ID - [`Client::product_id`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.product_id)
    - Fetch Product ID Bulk - [`Client::product_id_bulk`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.product_id_bulk)
    - Fetch Collectible Item ID - [`Client::collectible_item_id`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.collectible_item_id)
    - Fetch Collectible Item ID Bulk - [`Client::collectible_item_id_bulk`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.collectible_item_id_bulk)
* Economy API - [`economy.roblox.com/*`]
    - Fetch Robux Balance - [`Client::robux`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.robux)
    - Fetch Resellers - [`Client::resellers`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.resellers)
    - Fetch User Sales - [`Client::user_sales`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.user_sales)
    - Put Limited On Sale - [`Client::put_limited_on_sale`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.put_limited_on_sale)
    - Take Limited Off Sale - [`Client::take_limited_off_sale`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.take_limited_off_sale)
    - Purchase Tradable Limited - [`Client::purchase_tradable_limited`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.purchase_tradable_limited)
* Users API - [`users.roblox.com/*`]
    - Fetch User ID - [`Client::user_id`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.user_id)
    - Fetch Username - [`Client::username`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.username)
    - Fetch Display Name - [`Client::display_name`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.display_name)
    - User Search - [`Client::user_search`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.user_search)
* Presence API - [`presence.roblox.com/*`]
    - Register Presence - [`Client::register_presence`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.register_presence)
* Trades API - [`trades.roblox.com/*`]
    - Fetch Trades List - [`Client::trades`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.trades)
* Auth API - [`auth.roblox.com/*`]
    - Force Refresh X-CSRF-TOKEN - [`Client::force_refresh_xcsrf_token`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.force_refresh_xcsrf_token)
* Group API - [`groups.roblox.com/*`]
    - Fetch Group Roles - [`Client::group_roles`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.group_roles)
* BEDEV2 API - [`apis.roblox.com/*`] 
    - Fetch Non-Tradable Limited Details - [`Client::non_tradable_limited_details`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.non_tradable_limited_details)
    - Fetch Collectible Product ID - [`Client::collectible_product_id`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.collectible_product_id)
    - Fetch Collectible Product ID Bulk - [`Client::collectible_product_id_bulk`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.collectible_product_id_bulk)
    - Fetch Collectible Creator ID - [`Client::collectible_creator_id`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.collectible_creator_id)
    - Purchase Non-Tradable Limited - [`Client::purchase_non_tradable_limited`](https://docs.rs/roboat/latest/roboat/struct.Client.html#method.purchase_non_tradable_limited)

# Setup
You can add the latest version of roboat to your project by running:
```bash
cargo add roboat
```

Alternatively, you can add a specific version of roboat to your project by adding the crate to your `Cargo.toml`:

```toml
[dependencies]
roboat = "0.16.1"
```

# Quick Start Examples

## Example 1 - Purchase Free UGC Limited
This code snippet allows you to purchase a free ugc limited.

It can be modified to purchase a non-free ugc limited by changing the price.

```rust
// Replace this value with your own roblosecurity token.
const ROBLOSECURITY: &str = "your-roblosecurity-token";
// Replace this value with the item id of the item you want to purchase.
const ITEM_ID: u64 = 13119979433;
// Replace this value if you want to purchase a non-free item.
const PRICE: u64 = 0;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = roboat::ClientBuilder::new()
        .roblosecurity(ROBLOSECURITY.to_string())
        .build();

    let collectible_item_id = client.collectible_item_id(ITEM_ID).await?;

    let collectible_product_id = client
        .collectible_product_id(collectible_item_id.clone())
        .await?;

    let collectible_creator_id = client
        .collectible_creator_id(collectible_item_id.clone())
        .await?;

    client
        .purchase_non_tradable_limited(
            collectible_item_id,
            collectible_product_id,
            collectible_creator_id,
            PRICE,
        )
        .await?;

    println!("Purchased item {} for {} robux!", ITEM_ID, PRICE);

    Ok(())
}
```

## Example 2 - Fetch User Info

This code snippet allows you to get your current robux, id, username, and display name.

```rust
// Replace this value with your own roblosecurity token.
const ROBLOSECURITY: &str = "your-roblosecurity-token";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = roboat::ClientBuilder::new()
        .roblosecurity(ROBLOSECURITY.to_string())
        .build();

    let robux = client.robux().await?;
    let user_id = client.user_id().await?;
    let username = client.username().await?;
    let display_name = client.display_name().await?;    

    println!("Robux: {}", robux);
    println!("User ID: {}", user_id);
    println!("Username: {}", username);
    println!("Display Name: {}", display_name);

    Ok(())   
}
```

## Example 3 - Fetch Price of Tradable Limited

This code snippet allows you to view the lowest price of a tradable limited item by
fetching a list of reseller listings.

```rust
// Replace this value with your own roblosecurity token.
const ROBLOSECURITY: &str = "your-roblosecurity-token";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = roboat::ClientBuilder::new()
        .roblosecurity(ROBLOSECURITY.to_string())
        .build();

    let item_id = 1365767;
    let limit = roboat::Limit::Ten;
    let cursor = None;

    let (resellers, _) = client.resellers(item_id, limit, cursor).await?;

    println!("Lowest Price for Valkyrie Helm: {}", resellers[0].price);  

    Ok(())   
}
```

## Example 4 - Fetch Item Details

This code snippet allows you to get the details of an item.

```rust
use roboat::catalog::avatar_catalog::{ItemArgs, ItemType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = roboat::ClientBuilder::new().build();

    let item = ItemArgs {
        item_type: ItemType::Asset,
        id: 1365767,
    };

    let details = &client.item_details(vec![item]).await?[0];

    let name = &details.name;
    let description = &details.description;
    let creator_name = &details.creator_name;
    let price = details.price.unwrap_or(0);

    println!("Name: {}", name);
    println!("Description: {}", description);
    println!("Creator Name: {}", creator_name);
    println!("Price: {}", price);

    Ok(())   
}
```

# More Examples
More examples can be found in the [examples](examples) directory.

# Related Crates
This crate is a sister crate of [roli](https://crates.io/crates/roli), an API wrapper for [Rolimons.com](https://www.rolimons.com/).

# Contributing
Pull requests and issues are welcome! 

Please refer to [CONVENTIONS.md](CONVENTIONS.md) for information on conventions used in this crate.

Additional resources used to help make this crate are available in [RESOURCES.md](RESOURCES.md).

# License
MIT License
