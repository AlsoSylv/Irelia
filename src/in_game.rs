use hyper::{client::HttpConnector, Request};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{
    utils::request::{request_template, setup_hyper_client, uri_builder},
    Error,
};

pub struct InGameClient {
    client: hyper::Client<HttpsConnector<HttpConnector>>,
    url: String,
}

pub enum TeamID {
    ALL,
    UNKNOWN,
    ORDER,
    CHAOS,
    NEUTRAL,
}

impl InGameClient {
    pub fn new() -> Result<Self, Error> {
        let client = setup_hyper_client().unwrap();
        let url = "127.0.0.1:2999".to_owned();
        Ok(Self { client, url })
    }

    /// Gets data from the "liveclientdata/allgamedata" endpoint of the ingame api
    ///
    /// ```rust
    /// async fn all_game_data() {
    ///     use samira::in_game::InGameClient;
    ///     
    ///     let client = InGameClient::new().unwrap();
    ///     let all_game_data = client.all_game_data().await.unwrap();
    ///     println!("{:?}", all_game_data);
    /// }
    /// ```
    pub async fn all_game_data(&self) -> Result<Value, Error> {
        self.live_client("allgamedata").await
    }

    pub async fn active_player(&self) -> Result<Value, Error> {
        self.live_client("activeplayer").await
    }

    pub async fn active_player_name(&self) -> Result<String, Error> {
        self.live_client("activeplayername").await
    }

    pub async fn active_player_abilities(&self) -> Result<Value, Error> {
        self.live_client("activeplayerabilities").await
    }

    pub async fn active_player_runes(&self) -> Result<Value, Error> {
        self.live_client("activeplayerrunes").await
    }

    pub async fn player_list(&self, team: Option<TeamID>) -> Result<Value, Error> {
        let team = match team {
            Some(team) => match team {
                TeamID::ALL => "?teamID=ALL",
                TeamID::UNKNOWN => "?teamID=UNKNOWN",
                TeamID::ORDER => "?teamID=ORDER",
                TeamID::CHAOS => "?teamID=CHAOS",
                TeamID::NEUTRAL => "?teamID=NEUTRAL",
            },
            None => "",
        };
        let endpoint = format!("playerlist{}", team);
        self.live_client(&endpoint).await
    }

    pub async fn player_scores(&self, summoner: &str) -> Result<Value, Error> {
        self.live_client_with_summoner("playerscores", summoner)
            .await
    }

    pub async fn player_summoner_spells(&self, summoner: &str) -> Result<Value, Error> {
        self.live_client_with_summoner("playersummonerspells", summoner)
            .await
    }

    pub async fn player_main_runes(&self, summoner: &str) -> Result<Value, Error> {
        self.live_client_with_summoner("playermainrunes", summoner)
            .await
    }

    pub async fn player_items(&self, summoner: &str) -> Result<Value, Error> {
        self.live_client_with_summoner("playeritems", summoner)
            .await
    }

    pub async fn event_data(&self, event_id: Option<i32>) -> Result<Value, Error> {
        let event_id = match event_id {
            Some(id) => format!("?eventID={}", id),
            None => "".to_owned(),
        };
        let endpoint = format!("eventdata{}", event_id);
        self.live_client(&endpoint).await
    }

    pub async fn game_stats(&self) -> Result<Value, Error> {
        self.live_client("gamestats").await
    }

    async fn live_client<R: DeserializeOwned>(&self, endpoint: &str) -> Result<R, Error> {
        let endpoint = format!("/liveclientdata/{}", endpoint);
        self.in_game_tempalte(&endpoint).await
    }

    async fn live_client_with_summoner<R: DeserializeOwned>(
        &self,
        endpoint: &str,
        summoner: &str,
    ) -> Result<R, Error> {
        let endpoint = format!("/liveclientdata/{}?summonerName={}", endpoint, summoner);
        self.in_game_tempalte(&endpoint).await
    }

    async fn in_game_tempalte<R: DeserializeOwned>(&self, endpoint: &str) -> Result<R, Error> {
        let uri = uri_builder(&self.url, endpoint)?;

        let req = Request::builder()
            .method("GET")
            .uri(uri)
            .body(hyper::Body::empty());

        request_template::<R>(Error::LeagueStoppedRunning, req, &self.client, |bytes| {
            serde_json::from_slice::<R>(&bytes)
                .map_or(Err(Error::FailedParseJson), |value| Ok(value))
        })
        .await
    }
}
