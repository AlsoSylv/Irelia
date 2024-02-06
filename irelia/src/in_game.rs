//! Module for the LoL in_game API, docs have been copied from their [official counterparts](https://developer.riotgames.com/docs/lol#game-client-api)
//!
//! All types are all generated from the official JSON snippets

pub mod types;

use serde::de::DeserializeOwned;

use crate::{LCUError, RequestClient};

use self::types::*;

#[derive(Default)]
/// Struct that represents a connection to the in_game client
pub struct InGameClient {
    url: &'static str,
}

impl InGameClient {
    pub fn new() -> InGameClient {
        InGameClient {
            url: "127.0.0.1:2999",
        }
    }

    /// Returns the url, which is currently static
    pub fn url(&self) -> &str {
        self.url
    }

    /// Get all available data.
    ///
    /// A sample response can be found [here](https://static.developer.riotgames.com/docs/lol/liveclientdata_sample.json).
    pub async fn all_game_data(
        &self,
        request_client: &RequestClient,
    ) -> Result<AllGameData, LCUError> {
        self.live_client("allgamedata", None, request_client).await
    }

    /// Get all data about the active player.
    pub async fn active_player(
        &self,
        request_client: &RequestClient,
    ) -> Result<ActivePlayer, LCUError> {
        self.live_client("activeplayer", None, request_client).await
    }

    /// Returns the player name.
    pub async fn active_player_name(
        &self,
        request_client: &RequestClient,
    ) -> Result<String, LCUError> {
        self.live_client("activeplayername", None, request_client)
            .await
    }

    /// Get Abilities for the active player.
    pub async fn active_player_abilities(
        &self,
        request_client: &RequestClient,
    ) -> Result<Abilities, LCUError> {
        self.live_client("activeplayerabilities", None, request_client)
            .await
    }

    /// Retrieve the full list of runes for the active player.
    pub async fn active_player_runes(
        &self,
        request_client: &RequestClient,
    ) -> Result<FullRunes, LCUError> {
        self.live_client("activeplayerrunes", None, request_client)
            .await
    }

    /// Retrieve the list of heroes in the game and their stats.
    pub async fn player_list(
        &self,
        team: Option<TeamID>,
        request_client: &RequestClient,
    ) -> Result<Vec<AllPlayer>, LCUError> {
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
        self.live_client(&endpoint, None, request_client).await
    }

    /// Retrieve the list of the current scores for the player.
    pub async fn player_scores(
        &self,
        summoner: &str,
        request_client: &RequestClient,
    ) -> Result<Scores, LCUError> {
        self.live_client("playerscores", Some(summoner), request_client)
            .await
    }

    /// Retrieve the list of the summoner spells for the player.
    pub async fn player_summoner_spells(
        &self,
        summoner: &str,
        request_client: &RequestClient,
    ) -> Result<SummonerSpells, LCUError> {
        self.live_client("playersummonerspells", Some(summoner), request_client)
            .await
    }

    /// Retrieve the basic runes of any player.
    pub async fn player_main_runes(
        &self,
        summoner: &str,
        request_client: &RequestClient,
    ) -> Result<Runes, LCUError> {
        self.live_client("playermainrunes", Some(summoner), request_client)
            .await
    }

    /// Retrieve the list of items for the player.
    pub async fn player_items(
        &self,
        summoner: &str,
        request_client: &RequestClient,
    ) -> Result<Vec<Item>, LCUError> {
        self.live_client("playeritems", Some(summoner), request_client)
            .await
    }

    /// Get a list of events that have occurred in the game.
    pub async fn event_data(
        &self,
        event_id: Option<i32>,
        request_client: &RequestClient,
    ) -> Result<Events, LCUError> {
        let event_id = match event_id {
            Some(id) => format!("?eventID={}", id),
            None => "".to_owned(),
        };
        let endpoint = format!("eventdata{}", event_id);
        self.live_client(&endpoint, None, request_client).await
    }

    /// Basic data about the game.
    pub async fn game_stats(&self, request_client: &RequestClient) -> Result<GameData, LCUError> {
        self.live_client("gamestats", None, request_client).await
    }

    async fn live_client<R>(
        &self,
        endpoint: &str,
        summoner: Option<&str>,
        request_client: &RequestClient,
    ) -> Result<R, LCUError>
    where
        R: DeserializeOwned,
    {
        let endpoint = match summoner {
            Some(summoner) => format!("/liveclientdata/{}?summonerName={}", endpoint, summoner),
            None => format!("/liveclientdata/{}", endpoint),
        };

        request_client
            .request_template(self.url, &endpoint, "GET", None::<()>, None, |bytes| {
                serde_json::from_slice(&bytes).map_err(LCUError::SerdeJsonError)
            })
            .await
    }
}
