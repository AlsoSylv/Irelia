use hyper::{client::HttpConnector, http::uri, Request};
use hyper_tls::HttpsConnector;
use serde_json::Value;

use crate::{utils::request::setup_hyper_client, Errors};

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
    pub fn new() -> Result<Self, Errors> {
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
    pub async fn all_game_data(&self) -> Result<Option<Value>, Errors> {
        self.live_client("allgamedata").await
    }

    pub async fn active_player(&self) -> Result<Option<Value>, Errors> {
        self.live_client("activeplayer").await
    }

    pub async fn active_player_name(&self) -> Result<Option<Value>, Errors> {
        self.live_client("activeplayername").await
    }

    pub async fn active_player_abilities(&self) -> Result<Option<Value>, Errors> {
        self.live_client("activeplayerabilities").await
    }

    pub async fn active_player_runes(&self) -> Result<Option<Value>, Errors> {
        self.live_client("activeplayerrunes").await
    }

    pub async fn player_list(&self, team: Option<TeamID>) -> Result<Option<Value>, Errors> {
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

    pub async fn player_scores(&self, summoner: &str) -> Result<Option<Value>, Errors> {
        self.live_client_with_summoner("playerscores", summoner)
            .await
    }

    pub async fn player_summoner_spells(&self, summoner: &str) -> Result<Option<Value>, Errors> {
        self.live_client_with_summoner("playersummonerspells", summoner)
            .await
    }

    pub async fn player_main_runes(&self, summoner: &str) -> Result<Option<Value>, Errors> {
        self.live_client_with_summoner("playermainrunes", summoner)
            .await
    }

    pub async fn player_items(&self, summoner: &str) -> Result<Option<Value>, Errors> {
        self.live_client_with_summoner("playeritems", summoner)
            .await
    }

    pub async fn event_data(&self, event_id: Option<i32>) -> Result<Option<Value>, Errors> {
        let event_id = match event_id {
            Some(id) => format!("?eventID={}", id),
            None => "".to_owned(),
        };
        let endpoint = format!("eventdata{}", event_id);
        self.live_client(&endpoint).await
    }

    pub async fn game_stats(&self) -> Result<Option<Value>, Errors> {
        self.live_client("gamestats").await
    }

    async fn live_client(&self, endpoint: &str) -> Result<Option<Value>, Errors> {
        let endpoint = format!("/liveclientdata/{}", endpoint);
        self.tempalte(&endpoint).await
    }

    async fn live_client_with_summoner(
        &self,
        endpoint: &str,
        summoner: &str,
    ) -> Result<Option<Value>, Errors> {
        let endpoint = format!("/liveclientdata/{}?summonerName={}", endpoint, summoner);
        self.tempalte(&endpoint).await
    }

    async fn tempalte(&self, endpoint: &str) -> Result<Option<Value>, Errors> {
        let Ok(uri) = uri::Builder::new().scheme("https").authority(self.url.as_bytes()).path_and_query(endpoint).build() else {
            return Err(Errors::InvalidRequest);
        };
        print!("{}", uri);
        let Ok(req) = Request::builder()
            .method("GET")
            .uri(uri)
            .body(hyper::Body::empty()) else {
                return Err(Errors::InvalidRequest);
            };

        match self.client.request(req).await {
            Ok(mut res) => {
                let body = res.body_mut();
                let bytes = hyper::body::to_bytes(body).await;
                match bytes {
                    Ok(bytes) => {
                        if bytes.is_empty() {
                            Ok(None)
                        } else {
                            serde_json::from_slice::<Value>(&bytes)
                                .map_or(Err(Errors::FailedParseJson), |value| Ok(Some(value)))
                        }
                    }
                    Err(err) => panic!("{}", err),
                }
            }
            Err(err) => {
                if err.is_connect() {
                    Err(Errors::LeagueStoppedRunning)
                } else {
                    panic!("{}", err)
                }
            }
        }
    }
}
