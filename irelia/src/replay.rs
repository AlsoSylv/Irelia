pub mod types;

/// The `replay` and `in_game` API use the same URL
/// A number of endpoints are also shared
/// Hence why the replay API enables the `in_game` feature
pub use super::in_game::URL;
use crate::replay::types::{Playback, Recording, Render, Sequence};
use crate::{Error, RequestClient};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[allow(clippy::module_name_repetitions)]
#[derive(Default)]
pub struct ReplayClient;

impl ReplayClient {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Information about the game client process.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    pub async fn game(&self, request_client: &RequestClient) -> Result<types::Game, Error> {
        self.replay("game", "GET", None::<()>, request_client).await
    }

    /// Information about particle visibility.
    /// Returns a map of particle names to current visibility status.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    pub async fn get_particles(
        &self,
        request_client: &RequestClient,
    ) -> Result<serde_json::Value, Error> {
        self.replay("particles", "GET", None::<()>, request_client)
            .await
    }

    /// Allows modifying the currently visible particles.
    /// Takes a map of particle name to visibility and applies it to the currently rendering particles.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    pub async fn post_particles(
        &self,
        body: serde_json::Value,
        request_client: &RequestClient,
    ) -> Result<serde_json::Value, Error> {
        self.replay("particles", "POST", Some(body), request_client)
            .await
    }

    /// Returns the current replay playback state such as pause and current time.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    pub async fn get_playback(&self, request_client: &RequestClient) -> Result<Playback, Error> {
        self.replay("playback", "GET", None::<()>, request_client)
            .await
    }

    /// Allows modifying the playback state such as play / pause and the game time to seek to. All values are optional.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    pub async fn post_playback(
        &self,
        body: Playback,
        request_client: &RequestClient,
    ) -> Result<Playback, Error> {
        self.replay("playback", "POST", Some(body), request_client)
            .await
    }

    /// Returns the current status of video recording. Poll this resource for progress on the output.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    pub async fn get_recording(&self, request_client: &RequestClient) -> Result<Recording, Error> {
        self.replay("recording", "GET", None::<()>, request_client)
            .await
    }

    /// Post to begin a recording specifying the codec and output filepath. Subsequent GET requests to this resource will update the status.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    pub async fn post_recording(
        &self,
        body: Recording,
        request_client: &RequestClient,
    ) -> Result<Recording, Error> {
        self.replay("recording", "POST", Some(body), request_client)
            .await
    }

    /// Returns the current render properties.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    pub async fn get_render(&self, request_client: &RequestClient) -> Result<Render, Error> {
        self.replay("render", "GET", None::<()>, request_client)
            .await
    }

    /// Allows modifying the current render properties. All values are optional.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    pub async fn post_render(
        &self,
        body: Render,
        request_client: &RequestClient,
    ) -> Result<Render, Error> {
        self.replay("render", "POST", Some(body), request_client)
            .await
    }

    /// Returns the sequence currently being applied.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    pub async fn get_sequence(&self, request_client: &RequestClient) -> Result<Sequence, Error> {
        self.replay("sequence", "GET", None::<()>, request_client)
            .await
    }

    /// Post to apply a sequence of keyframes that the replay should play. Post an empty object to remove the sequence.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    pub async fn post_sequence(
        &self,
        body: Sequence,
        request_client: &RequestClient,
    ) -> Result<Sequence, Error> {
        self.replay("sequence", "POST", Some(body), request_client)
            .await
    }

    /// Internal abstraction over `request_client`, this lets me cut out anything that only applies here,
    /// and reduce the usage in oneliners
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
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
