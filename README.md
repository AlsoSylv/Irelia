# Irelia

Irelia is a set of bindings to native LoL APIs built on top of Hyper.

This crate provides support for Windows, with untested macOS support.

Please note, League of Legends will not be playable on wine as of patch 14.5, and as such, the platform is no longer supported

Note: The base64 encoder used in irelia requires a .cargo/config.toml, an example can be found [here](./.cargo/config.toml)

#
### Cargo Features

This crate is designed with modularity in mind, and as such API support has been split into different cargo features. 

By default, this crate only ships with the rest feature enabled.

- `["full"]` - enables support for all APIs
- `["ws"]` - enables support for the LCU websocket
- `["in_game"]` - enables support for the native in game API
- `["tauri]` - derives serialize on errors
- `["batched"]` - enabled the batched request system
- `["replay"]` - enables the replay API interface


#
### Examples
```rust
use irelia::{Error, RequestClient, rest::LcuClient};
use serde_json::Value;

/// Get the player from the client API
async fn get_summoner() -> Result<Option<Value>, Error> {
    // Create a new general request client
    let client = RequestClient::new();

    // Read the lock file regardless of any running client or game
    let lcu_client = LcuClient::new(true)?;

    // The return type must be defined
    // And can be any struct that implements serde::Deserialize
    lcu_client
        .get("/lol-summoner/v1/current-summoner", &client)
        .await
}
```

```rust
use irelia::{RequestClient, in_game::{GameClient, types::ActivePlayer}};

/// Get the player from the in game API
async fn get_in_game_summoner() -> Result<ActivePlayer, Error> {
    // Create a new general request client
    let client = RequestClient::new();

    // Create a connection to the game client
    let game_client = GameClient::new();

    game_client.active_player(&client).await
}

```

```rust
use irelia::{RequestClient, replay::{ReplayClient, types::Playback}};

/// Get the playback status from the replay API
async fn get_replay_playback() -> Result<Playback, Error> {
    // Create a new general request client
    let client = RequestClient::new();

    // Create a replay connection
    let replay_client = ReplayClient::new();

    replay_client.get_playback(&client).await
}
```