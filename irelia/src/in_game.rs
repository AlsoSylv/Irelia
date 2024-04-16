//! Module for the `LoL` `in_game` API, docs have been copied from their [official counterparts](https://developer.riotgames.com/docs/lol#game-client-api)
//!
//! All types are all generated from the official JSON snippets

pub mod types;

use serde::de::DeserializeOwned;

use crate::{Error, RequestClient};

use self::types::{
    Abilities, ActivePlayer, AllGameData, AllPlayer, Events, FullRunes, GameData, Item, Runes,
    Scores, SummonerSpells, TeamID,
};

/// The only url the in game API can be used on
pub const URL: &str = "127.0.0.1:2999";

/// Struct that represents a connection to the in game api client
/// Because the URL is constant, this is a zero sized struct to help organize code
pub struct GameClient;

impl GameClient {
    #[must_use]
    pub fn new() -> GameClient {
        GameClient
    }

    #[must_use]
    /// Returns the url, which is currently static
    pub fn url(&self) -> &str {
        URL
    }

    //noinspection SpellCheckingInspection
    /// Get all available data.
    ///
    /// A sample response can be found [here](https://static.developer.riotgames.com/docs/lol/liveclientdata_sample.json).
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    pub async fn all_game_data(
        &self,
        request_client: &RequestClient,
    ) -> Result<AllGameData, Error> {
        self.live_client("allgamedata", None, request_client).await
    }

    //noinspection SpellCheckingInspection
    /// Get all data about the active player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    pub async fn active_player(
        &self,
        request_client: &RequestClient,
    ) -> Result<ActivePlayer, Error> {
        self.live_client("activeplayer", None, request_client).await
    }

    //noinspection SpellCheckingInspection
    /// Returns the player name.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    pub async fn active_player_name(
        &self,
        request_client: &RequestClient,
    ) -> Result<String, Error> {
        self.live_client("activeplayername", None, request_client)
            .await
    }

    //noinspection SpellCheckingInspection
    /// Get Abilities for the active player.    
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    pub async fn active_player_abilities(
        &self,
        request_client: &RequestClient,
    ) -> Result<Abilities, Error> {
        self.live_client("activeplayerabilities", None, request_client)
            .await
    }

    //noinspection SpellCheckingInspection
    /// Retrieve the full list of runes for the active player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    pub async fn active_player_runes(
        &self,
        request_client: &RequestClient,
    ) -> Result<FullRunes, Error> {
        self.live_client("activeplayerrunes", None, request_client)
            .await
    }

    //noinspection SpellCheckingInspection
    /// Retrieve the list of heroes in the game and their stats.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    pub async fn player_list(
        &self,
        team: Option<TeamID>,
        request_client: &RequestClient,
    ) -> Result<Vec<AllPlayer>, Error> {
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

        let endpoint = format!("playerlist{team}");
        self.live_client(&endpoint, None, request_client).await
    }

    //noinspection SpellCheckingInspection
    /// Retrieve the list of the current scores for the player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    pub async fn player_scores(
        &self,
        summoner: &str,
        request_client: &RequestClient,
    ) -> Result<Scores, Error> {
        self.live_client("playerscores", Some(summoner), request_client)
            .await
    }

    //noinspection SpellCheckingInspection
    /// Retrieve the list of the summoner spells for the player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    pub async fn player_summoner_spells(
        &self,
        summoner: &str,
        request_client: &RequestClient,
    ) -> Result<SummonerSpells, Error> {
        self.live_client("playersummonerspells", Some(summoner), request_client)
            .await
    }

    //noinspection SpellCheckingInspection
    /// Retrieve the basic runes of any player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    pub async fn player_main_runes(
        &self,
        summoner: &str,
        request_client: &RequestClient,
    ) -> Result<Runes, Error> {
        self.live_client("playermainrunes", Some(summoner), request_client)
            .await
    }

    //noinspection SpellCheckingInspection
    /// Retrieve the list of items for the player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    pub async fn player_items(
        &self,
        summoner: &str,
        request_client: &RequestClient,
    ) -> Result<Vec<Item>, Error> {
        self.live_client("playeritems", Some(summoner), request_client)
            .await
    }

    //noinspection SpellCheckingInspection
    /// Get a list of events that have occurred in the game.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    pub async fn event_data(
        &self,
        event_id: Option<i32>,
        request_client: &RequestClient,
    ) -> Result<Events, Error> {
        let event_id = if let Some(id) = event_id {
            format!("?eventID={id}")
        } else {
            String::new()
        };
        let endpoint = format!("eventdata{event_id}");
        self.live_client(&endpoint, None, request_client).await
    }

    //noinspection SpellCheckingInspection
    /// Basic data about the game.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    pub async fn game_stats(&self, request_client: &RequestClient) -> Result<GameData, Error> {
        self.live_client("gamestats", None, request_client).await
    }

    //noinspection SpellCheckingInspection
    async fn live_client<R>(
        &self,
        endpoint: &str,
        summoner: Option<&str>,
        request_client: &RequestClient,
    ) -> Result<R, Error>
    where
        R: DeserializeOwned,
    {
        let endpoint = if let Some(summoner) = summoner {
            format!("/liveclientdata/{endpoint}?summonerName={summoner}")
        } else {
            format!("/liveclientdata/{endpoint}")
        };

        request_client
            .request_template(URL, &endpoint, "GET", None::<()>, None, |bytes| {
                Ok(serde_json::from_slice(&bytes)?)
            })
            .await
    }
}

impl Default for GameClient {
    fn default() -> Self {
        GameClient
    }
}
