pub mod types;

/// The `replay` and `in_game` API use the same URL
/// A number of endpoints are also shared
/// Hence why the replay API enables the `in_game` feature
pub use super::in_game::URL;
use crate::{Error, RequestClient};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::replay::types::{Playback, Recording, Render, Sequence};

#[allow(clippy::module_name_repetitions)]
pub struct ReplayClient;

impl ReplayClient {
    pub async fn game(&self, request_client: &RequestClient) -> Result<types::Game, Error> {
        self.replay("game", "GET", None::<()>, request_client).await
    }

    pub async fn get_particles(&self, request_client: &RequestClient) -> Result<serde_json::Value, Error> {
        self.replay("particles", "GET", None::<()>, request_client).await
    }

    pub async fn post_particles(&self, body: serde_json::Value, request_client: &RequestClient) -> Result<serde_json::Value, Error> {
        self.replay("particles", "POST", Some(body), request_client).await
    }

    pub async fn get_playback(&self, request_client: &RequestClient) -> Result<Playback, Error> {
        self.replay("playback", "GET", None::<()>, request_client).await
    }

    pub async fn post_playback(&self, body: Playback, request_client: &RequestClient) -> Result<Playback, Error> {
        self.replay("playback", "POST", Some(body), request_client).await
    }

    pub async fn get_recording(&self, request_client: &RequestClient) -> Result<Recording, Error> {
        self.replay("recording", "GET", None::<()>, request_client).await
    }

    pub async fn post_recording(&self, body: Recording, request_client: &RequestClient) -> Result<Recording, Error> {
        self.replay("recording", "POST", Some(body), request_client).await
    }

    pub async fn get_render(&self, request_client: &RequestClient) -> Result<Render, Error> {
        self.replay("render", "GET", None::<()>, request_client).await
    }

    pub async fn post_render(&self, body: Render, request_client: &RequestClient) -> Result<Render, Error> {
        self.replay("render", "POST", Some(body), request_client).await
    }

    pub async fn get_sequence(&self, request_client: &RequestClient) -> Result<Sequence, Error> {
        self.replay("sequence", "GET", None::<()>, request_client).await
    }

    pub async fn post_sequence(&self, body: Sequence, request_client: &RequestClient) -> Result<Sequence, Error> {
        self.replay("sequence", "POST", Some(body), request_client).await
    }

    async fn replay<R>(
        &self,
        endpoint: &str,
        method: &'static str,
        body: Option<impl Serialize>,
        request_client: &RequestClient,
    ) -> Result<R, Error>
    where
        R: DeserializeOwned,
    {
        let endpoint = format!("/replay/{endpoint}");

        request_client
            .request_template(URL, &endpoint, method, body, None, |bytes| {
                serde_json::from_slice(&bytes).map_err(Error::SerdeJsonError)
            })
            .await
    }
}
