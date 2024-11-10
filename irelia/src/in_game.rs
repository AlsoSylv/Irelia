//! Module for the `LoL` `in_game` API, docs have been copied from their [official counterparts](https://developer.riotgames.com/docs/lol#game-client-api)
//!
//! All types are all generated from the official JSON snippets

pub mod types;

use hyper::body::Incoming;
use hyper::Response;
use serde::de::DeserializeOwned;
use std::net::{Ipv4Addr, SocketAddrV4};

use crate::{Error, RequestClient};

use self::types::{
    Abilities, ActivePlayer, AllGameData, AllPlayer, Events, GameData, Item, Runes, Scores,
    SummonerSpells, TeamID,
};

/// The only url the in game API can be used on
pub const URL: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 2999);

/// Struct that represents a connection to the in game api client
/// Because the URL is constant, this is a zero sized struct to help organize code
/// Please note, well this is currently 0 sized, it could change
/// So it is advised to use `GameClient::new()` instead
pub struct GameClient;

impl GameClient {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    #[must_use]
    /// Returns the url, which is currently static
    pub const fn url(&self) -> SocketAddrV4 {
        URL
    }

    /// This makes a head request to the in game API
    ///
    /// Please note, when using invalid methods (such as `HEAD`) with an Operation ID
    /// The API will always return 200 with an empty body, at least in my testing
    /// This can be used to see if the API is alive and if you're outside the loading screen
    ///
    /// # Errors
    /// This will return an error if it is unable to connect to the server
    pub async fn head(
        &self,
        endpoint: &str,
        request_client: &RequestClient,
    ) -> Result<Response<Incoming>, Error> {
        request_client
            .raw_request_template(URL, endpoint, "HEAD", None, None)
            .await
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
    ) -> Result<Runes, Error> {
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
    ) -> Result<Box<[AllPlayer]>, Error> {
        let team = team.map_or_else(
            || "",
            |team| match team {
                TeamID::All => "?teamID=ALL",
                TeamID::Unknown => "?teamID=UNKNOWN",
                TeamID::Order => "?teamID=ORDER",
                TeamID::Chaos => "?teamID=CHAOS",
                TeamID::Neutral => "?teamID=NEUTRAL",
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
        summoner: impl AsRef<str> + Send,
        request_client: &RequestClient,
    ) -> Result<Scores, Error> {
        self.live_client("playerscores", Some(summoner.as_ref()), request_client)
            .await
    }

    //noinspection SpellCheckingInspection
    /// Retrieve the list of the summoner spells for the player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    pub async fn player_summoner_spells(
        &self,
        riot_id: impl AsRef<str> + Send,
        request_client: &RequestClient,
    ) -> Result<SummonerSpells, Error> {
        self.live_client(
            "playersummonerspells",
            Some(riot_id.as_ref()),
            request_client,
        )
        .await
    }

    //noinspection SpellCheckingInspection
    /// Retrieve the basic runes of any player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    pub async fn player_main_runes(
        &self,
        riot_id: impl AsRef<str> + Send,
        request_client: &RequestClient,
    ) -> Result<Runes, Error> {
        self.live_client("playermainrunes", Some(riot_id.as_ref()), request_client)
            .await
    }

    //noinspection SpellCheckingInspection
    /// Retrieve the list of items for the player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    pub async fn player_items(
        &self,
        riot_id: impl AsRef<str> + Send,
        request_client: &RequestClient,
    ) -> Result<Box<[Item]>, Error> {
        self.live_client("playeritems", Some(riot_id.as_ref()), request_client)
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
        let event_id = event_id.map_or(String::new(), |id| format!("?eventID={id}"));
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
    /// Makes a request to a liveclientdata endpoint
    async fn live_client<R: DeserializeOwned>(
        &self,
        endpoint: &str,
        riot_id: Option<&str>,
        request_client: &RequestClient,
    ) -> Result<R, Error> {
        use hyper::body::Buf;

        let endpoint = riot_id.map_or_else(
            || format!("/liveclientdata/{endpoint}"),
            |riot_id| format!("/liveclientdata/{endpoint}?riotId={riot_id}"),
        );

        let buf = request_client
            .request_template(URL, &endpoint, "GET", None::<()>, None)
            .await?;

        Ok(rmp_serde::from_read(buf.reader())?)
    }
}

impl Default for GameClient {
    fn default() -> Self {
        Self
    }
}
