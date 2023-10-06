# Irelia

Irelia is a set of bindings to native LoL APIs built on top of Hyper.

This crate provides support for Windows, and Linux, with untested MacOS support.

Note: The base64 encoder used in irelia requires a .cargo/config.toml, an example can be found [here](./.cargo/config.toml)

#
### Cargo Features

This crate is designed with modularity in mind, and as such API support has been split into different cargo features. 

By default, this crate only ships with the rest feature enabled.

- `["full"]` - enables support for all APIs
- `["ws"]` - enables support for the LCU websocket
- `["in_game"]` - enables support for the native in game API
- `["tauri]` - derives searialize on errors
- `["batched"]` - enabled the batched request system


#
### Examples
```rust
use irelia::{RequestClient, rest::LCUClient};
use serde_json::Value;

/// Get the player from the client API
async fn get_summoner() -> Result<Option<Value>, LCUError> {
    // Create a new general request client
    let client = RequestClient::new();

    // Pass the client to the LCU connection
    let lcu_client = LCUClient::new()?;

    // The return type must be defined
    // And can be any struct that implements serde::Deserialize
    client.get("/lol-summoner/v1/current-summoner", &client).await
}
```

```rust
use irelia::{RequestClient, in_game::InGameClient};
use serde_json::Value;

/// Get the player from the in game API
async fn get_in_game_summoner() -> Result<ActivePlayer, LCUError> {
    // Create a new general request client
    let client = RequestClient::new();

    // Pass the client to the LCU connection
    let game_client = InGameClient::new()?;

    game_client.active_player(&client)
}

```