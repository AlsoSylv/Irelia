//! Module for the LoL in_game API, docs have been copied from their [official counterparts](https://developer.riotgames.com/docs/lol#game-client-api)
//!
//! All types are all generated from the official JSON snippets

pub mod types;

use serde::de::DeserializeOwned;

use crate::{LCUError, RequestClient};

use self::types::*;

pub struct InGameClient<'a> {
    client: &'a RequestClient,
    url: &'a str,
}

impl InGameClient<'_> {
    pub fn new(client: &RequestClient) -> InGameClient {
        InGameClient {
            client,
            url: "127.0.0.1:2999",
        }
    }

    /// Get all available data.
    ///
    /// A sample response can be found [here](https://static.developer.riotgames.com/docs/lol/liveclientdata_sample.json).
    pub async fn all_game_data(&self) -> Result<AllGameData, LCUError> {
        self.live_client("allgamedata", None).await
    }

    /// Get all data about the active player.
    pub async fn active_player(&self) -> Result<ActivePlayer, LCUError> {
        self.live_client("activeplayer", None).await
    }

    /// Returns the player name.
    pub async fn active_player_name(&self) -> Result<String, LCUError> {
        self.live_client("activeplayername", None).await
    }

    /// Get Abilities for the active player.
    pub async fn active_player_abilities(&self) -> Result<Abilities, LCUError> {
        self.live_client("activeplayerabilities", None).await
    }

    /// Retrieve the full list of runes for the active player.
    pub async fn active_player_runes(&self) -> Result<FullRunes, LCUError> {
        self.live_client("activeplayerrunes", None).await
    }

    /// Retrieve the list of heroes in the game and their stats.
    pub async fn player_list(&self, team: Option<TeamID>) -> Result<Vec<AllPlayer>, LCUError> {
        let team = team.map_or_else(
            || "",
            |team| match team {
                TeamID::ALL => "?teamID=ALL",
                TeamID::UNKNOWN => "?teamID=UNKNOWN",
                TeamID::ORDER => "?teamID=ORDER",
                TeamID::CHAOS => "?teamID=CHAOS",
                TeamID::NEUTRAL => "?teamID=NEUTRAL",
            },
        );

        let endpoint = format!("playerlist{}", team);
        self.live_client(&endpoint, None).await
    }

    /// Retrieve the list of the current scores for the player.
    pub async fn player_scores(&self, summoner: &str) -> Result<Scores, LCUError> {
        self.live_client("playerscores", Some(summoner)).await
    }

    /// Retrieve the list of the summoner spells for the player.
    pub async fn player_summoner_spells(&self, summoner: &str) -> Result<SummonerSpells, LCUError> {
        self.live_client("playersummonerspells", Some(summoner))
            .await
    }

    /// Retrieve the basic runes of any player.
    pub async fn player_main_runes(&self, summoner: &str) -> Result<Runes, LCUError> {
        self.live_client("playermainrunes", Some(summoner)).await
    }

    /// Retrieve the list of items for the player.
    pub async fn player_items(&self, summoner: &str) -> Result<Vec<Item>, LCUError> {
        self.live_client("playeritems", Some(summoner)).await
    }

    /// Get a list of events that have occurred in the game.
    pub async fn event_data(&self, event_id: Option<i32>) -> Result<Events, LCUError> {
        let event_id = match event_id {
            Some(id) => format!("?eventID={}", id),
            None => "".to_owned(),
        };
        let endpoint = format!("eventdata{}", event_id);
        self.live_client(&endpoint, None).await
    }

    /// Basic data about the game.
    pub async fn game_stats(&self) -> Result<GameData, LCUError> {
        self.live_client("gamestats", None).await
    }

    async fn live_client<R>(&self, endpoint: &str, summoner: Option<&str>) -> Result<R, LCUError>
    where
        R: DeserializeOwned,
    {
        let endpoint = summoner.map_or_else(
            || format!("/liveclientdata/{}", endpoint),
            |summoner| format!("/liveclientdata/{}?summonerName={}", endpoint, summoner),
        );

        self.client
            .request_template::<(), R>(self.url, &endpoint, "GET", None, None, |bytes| {
                serde_json::from_slice(&bytes)
                    .map_or_else(|err| Err(LCUError::SerdeJsonError(err)), Ok)
            })
            .await
    }
}
