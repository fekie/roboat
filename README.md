[![Crates.io](https://img.shields.io/crates/v/roboat.svg)](https://crates.io/crates/roboat)
[![Documentation](https://docs.rs/roboat/badge.svg)](https://docs.rs/roboat/)
[![dependency status](https://deps.rs/repo/github/chloe-woahie/roboat/status.svg)](https://deps.rs/repo/github/chloe-woahie/roboat)

[![](https://dcbadge.vercel.app/api/server/QmBEgPaFSD)](https://discord.gg/QmBEgPaFSD)

# roboat
An API wrapper for Roblox.com.

This library is designed to be high-performance capable, meaning that it supports proxies
and is capable of making requests in parallel.

# Covered Endpoints
* Catalog API - [`catalog.roblox.com/*`]
    - Item Details - `/v1/catalog/items/details`
* Economy API - [`economy.roblox.com/*`]
    - Robux Balance - `/v1/users/{user_id}/currency`
    - Resellers - `/v1/assets/{item_id}/resellers`
    - User Sales - `/v2/users/{user_id}/transactions?transactionType=Sale`
    - Put Limited On Sale - `/v1/assets/{item_id}/resellable-copies/{uaid}`
    - Take Limited Off Sale - `/v1/assets/{item_id}/resellable-copies/{uaid}`
* Users API - [`users.roblox.com/*`]
    - User Information - `/v1/users/authenticated`
* Presence API - [`presence.roblox.com/*`]
    - Register Presence - `/v1/presence/register-app-presence`

# Quick Start Examples

## Example 1

This code snippet allows you to get the details of an item.

```rust
use roboat::catalog::avatar_catalog::{ItemArgs, ItemType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = roboat::Client::new();

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

## Example 2

This code snippet allows you view the lowest price of a limited item by
fetching a list of reseller listings.

```rust
// Replace this value with your own roblosecurity token.
const ROBLOSECURITY: &str = "your-roblosecurity-token";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = roboat::Client::with_roblosecurity(ROBLOSECURITY.to_string());

    let item_id = 1365767;
    let limit = roboat::Limit::Ten;
    let cursor = None;

    let (resellers, _) = client.resellers(item_id, limit, cursor).await?;

    println!("Lowest Price for Valkyrie Helm: {}", resellers[0].price);  

    Ok(())   
}
```

## Example 3

This code snippet allows you to get your current robux, id, username, and display name.

```rust
// Replace this value with your own roblosecurity token.
const ROBLOSECURITY: &str = "your-roblosecurity-token";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = roboat::Client::with_roblosecurity(ROBLOSECURITY.to_string());

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

# Related Crates
This crate is a sister crate of [roli](https://crates.io/crates/roli), an API wrapper for [Rolimons.com](https://www.rolimons.com/).

# Contributing
Pull requests and issues are welcome! 

Please refer to [CONVENTIONS.md](CONVENTIONS.md) for information on conventions used in this crate.

Additional resources used to help make this crate are available in [RESOURCES.md](RESOURCES.md).

# License
MIT License
