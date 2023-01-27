## Irelia
---
Irelia is a set of bindings to native LoL APIs built on top of Hyper.
This crate provides support for Windows, and Linux, with untested MacOS support.

---

### Cargo Features

This crate is designed with modularity in mind, and as such API support has been split into different cargo features. By default, this crate only ships with support for the LCU Rest API.

- `features = ["full"]` - enables support for all APIs
- `features = ["ws"]` - enables support for the LCU websocket
- `features = ["in_game"]` - enables support for the native in game API

---

### Examples
```rust
use irelia::rest::LCUClient;
use serde_json::Value;

async fn get_summoner() {
    let client = LCUClient::new().unwrap();
    // The return type must be defined
    // And can be any struct that implements serde::Deserialize
    let summoner: Option<Value> = client.get("/lol-summoner/v1/current-summoner").await.unwrap();
}
```