[![Crates.io](https://img.shields.io/crates/v/roboat.svg)](https://crates.io/crates/roboat)
[![Documentation](https://docs.rs/roboat/badge.svg)](https://docs.rs/roboat/)
[![dependency status](https://deps.rs/repo/github/chloe-woahie/roboat/status.svg)](https://deps.rs/repo/github/chloe-woahie/roboat)

[![](https://dcbadge.vercel.app/api/server/QmBEgPaFSD)](https://discord.gg/QmBEgPaFSD)

# roboat
An API wrapper for Roblox.com.

This library is designed to be high-performance capable, meaning that it supports proxies
and is capable of making requests in parallel.

# Covered Endpoints
* Catalog API
    - Item Details - <https://catalog.roblox.com/v1/catalog/items/details>
* Economy API
    - Robux Balance - <https://economy.roblox.com/v1/users/{user_id}/currency>
* User API
    - User Information - <https://users.roblox.com/v1/users/authenticated>

# Contributing
Pull requests and issues are welcome! 

Please refer to [CONVENTIONS.md](CONVENTIONS.md) for information on conventions used in this crate.

Additional resources used to help make this crate are available in [RESOURCES.md](RESOURCES.md).

# License
MIT License
