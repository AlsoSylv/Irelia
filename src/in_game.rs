//! The methods and types in this crate all line up with the ones
//! in Riots documentation, and should be straight forward to use
//! this module is templated in a way that adding or removing
//! methods should be simple and easy

use hyper::{client::HttpConnector, Request};
use hyper_tls::HttpsConnector;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;

use crate::{
    utils::request::{request_template, uri_builder, HYPER_CLIENT},
    Error,
};

use self::types::{
    Abilities, ActivePlayer, AllGameData, AllPlayer, Events, FullRunes, GameData, Item, Runes,
    Scores, SummonerSpells,
};

pub mod types;

/// Struct that handles connections to the in-game API
/// holding a refernce to the hyper client and url
pub struct InGameClient<'a> {
    client: &'a Lazy<hyper::Client<HttpsConnector<HttpConnector>>>,
    url: &'a str,
}

/// Enum representation of different team IDs
pub enum TeamID {
    ALL,
    UNKNOWN,
    ORDER,
    CHAOS,
    NEUTRAL,
}

impl InGameClient<'_> {
    /// Make a connect to the in game API, does not check if the game is running when connecting
    pub fn new<'a>() -> Result<InGameClient<'a>, Error> {
        let client = &HYPER_CLIENT;
        let url = "127.0.0.1:2999";
        Ok(InGameClient { client, url })
    }

    /// Gets data from the "liveclientdata/allgamedata" endpoint of the ingame api
    ///
    /// ```rust
    /// async fn all_game_data() {
    ///     use irelia::in_game::InGameClient;
    ///     
    ///     let client = InGameClient::new().unwrap();
    ///     let all_game_data = client.all_game_data().await.unwrap();
    ///     println!("{:?}", all_game_data);
    /// }
    /// ```
    pub async fn all_game_data(&self) -> Result<AllGameData, Error> {
        self.live_client("allgamedata", None).await
    }

    pub async fn active_player(&self) -> Result<ActivePlayer, Error> {
        self.live_client("activeplayer", None).await
    }

    pub async fn active_player_name(&self) -> Result<String, Error> {
        self.live_client("activeplayername", None).await
    }

    pub async fn active_player_abilities(&self) -> Result<Abilities, Error> {
        self.live_client("activeplayerabilities", None).await
    }

    pub async fn active_player_runes(&self) -> Result<FullRunes, Error> {
        self.live_client("activeplayerrunes", None).await
    }

    pub async fn player_list(&self, team: Option<TeamID>) -> Result<Vec<AllPlayer>, Error> {
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

    pub async fn player_scores(&self, summoner: &str) -> Result<Scores, Error> {
        self.live_client("playerscores", Some(summoner)).await
    }

    pub async fn player_summoner_spells(&self, summoner: &str) -> Result<SummonerSpells, Error> {
        self.live_client("playersummonerspells", Some(summoner))
            .await
    }

    pub async fn player_main_runes(&self, summoner: &str) -> Result<Runes, Error> {
        self.live_client("playermainrunes", Some(summoner)).await
    }

    pub async fn player_items(&self, summoner: &str) -> Result<Vec<Item>, Error> {
        self.live_client("playeritems", Some(summoner)).await
    }

    pub async fn event_data(&self, event_id: Option<i32>) -> Result<Events, Error> {
        let event_id = match event_id {
            Some(id) => format!("?eventID={}", id),
            None => "".to_owned(),
        };
        let endpoint = format!("eventdata{}", event_id);
        self.live_client(&endpoint, None).await
    }

    pub async fn game_stats(&self) -> Result<GameData, Error> {
        self.live_client("gamestats", None).await
    }

    async fn live_client<R: DeserializeOwned>(
        &self,
        endpoint: &str,
        summoner: Option<&str>,
    ) -> Result<R, Error> {
        let endpoint = summoner.map_or_else(
            || format!("/liveclientdata/{}", endpoint),
            |summoner| format!("/liveclientdata/{}?summonerName={}", endpoint, summoner),
        );
        let uri = uri_builder(self.url, &endpoint)?;

        let req = Request::builder()
            .method("GET")
            .uri(uri)
            .body(hyper::Body::empty());

        request_template(Error::LeagueStoppedRunning, req, self.client, |bytes| {
            serde_json::from_slice(&bytes).map_or(Err(Error::FailedParseJson), Ok)
        })
        .await
    }
}
