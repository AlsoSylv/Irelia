//! This is a module for the League replay API, it's still WIP, and documentation is done on a best effort basis
//!
//! Please note, well most `in_game` endpoints will work when using the `replay` API, riot does not support
//! using the `active_player` endpoints, and as such, they are expected to return errors instead
//!
//! The `replay` API uses `MsgPack` internally to communicate, as there is a max request size of 512kb, and
//! the API will not accept compressed inputs, but is willing to return compressed outputs

/// Types returned and sent to the API
pub mod types;

/// The `replay` and `in_game` API use the same URL
/// A number of endpoints are also shared
/// Hence why the replay API enables the `in_game` feature
pub use super::in_game::URL;
use crate::in_game::GameClient;
use crate::replay::types::{Playback, RecordingState, Render, Sequence};
use crate::{Error, in_game};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::future::Future;

/// Trait for replay API endpoints
#[allow(clippy::module_name_repetitions)]
pub trait ReplayClient: in_game::GameClient {
    /// Information about the game client process.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    fn game(&self) -> impl Future<Output = Result<types::Game, Error<Self::Error>>> + Send {
        self.replay("/replay/game", "GET", None::<()>)
    }

    /// Information about particle visibility.
    /// Returns a map of particle names to current visibility status.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    fn get_particles(
        &self,
    ) -> impl Future<Output = Result<HashMap<String, bool>, Error<Self::Error>>> + Send {
        self.replay("/replay/particles", "GET", None::<()>)
    }

    /// Allows modifying the currently visible particles.
    /// Takes a map of particle name to visibility and applies it to the currently rendering particles.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    fn post_particles(
        &self,
        body: impl Borrow<HashMap<String, bool>> + Send,
    ) -> impl Future<Output = Result<HashMap<String, bool>, Error<Self::Error>>> + Send {
        async move {
            self.replay("/replay/particles", "POST", Some(body.borrow()))
                .await
        }
    }

    /// Returns the current replay playback state such as pause and current time.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    fn get_playback(&self) -> impl Future<Output = Result<Playback, Error<Self::Error>>> + Send {
        self.replay("/replay/playback", "GET", None::<()>)
    }

    /// Allows modifying the playback state such as play / pause and the game time to seek to. All values are optional.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    fn post_playback(
        &self,
        body: impl Borrow<Playback> + Send,
    ) -> impl Future<Output = Result<Playback, Error<Self::Error>>> + Send {
        async move {
            self.replay("/replay/playback", "POST", Some(body.borrow()))
                .await
        }
    }

    /// Returns the current status of video recording. Poll this resource for progress on the output.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    fn get_recording(
        &self,
    ) -> impl Future<Output = Result<RecordingState, Error<Self::Error>>> + Send {
        self.replay("/replay/recording", "GET", None::<()>)
    }

    /// Post to begin a recording specifying the codec and output filepath. Subsequent GET requests to this resource will update the status.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    fn post_recording(
        &self,
        body: impl Borrow<RecordingState> + Send,
    ) -> impl Future<Output = Result<RecordingState, Error<Self::Error>>> + Send {
        async move {
            self.replay("/replay/recording", "POST", Some(body.borrow()))
                .await
        }
    }

    /// Returns the current render properties.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    fn get_render(&self) -> impl Future<Output = Result<Render, Error<Self::Error>>> + Send {
        self.replay("/replay/render", "GET", None::<()>)
    }

    /// Allows modifying the current render properties. All values are optional.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    fn post_render(
        &self,
        body: impl Borrow<Render> + Send,
    ) -> impl Future<Output = Result<Render, Error<Self::Error>>> + Send {
        async move {
            self.replay("/replay/render", "POST", Some(body.borrow()))
                .await
        }
    }

    /// Returns the sequence currently being applied.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    fn get_sequence(&self) -> impl Future<Output = Result<Sequence, Error<Self::Error>>> + Send {
        self.replay("/replay/sequence", "GET", None::<()>)
    }

    /// Post to apply a sequence of keyframes that the replay should play
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    fn post_sequence(
        &self,
        body: impl Borrow<Sequence> + Send,
    ) -> impl Future<Output = Result<Sequence, Error<Self::Error>>> + Send {
        async move {
            self.replay("/replay/sequence", "POST", Some(body.borrow()))
                .await
        }
    }

    /// Sends a post request to the replay API with `None` as the body, aka null.
    ///
    /// # Errors
    /// This will return an error if there is not an active replay running
    fn reset_sequence(&self) -> impl Future<Output = Result<Sequence, Error<Self::Error>>> + Send {
        self.replay("/replay/sequence", "POST", Some(None::<Sequence>))
    }
}

impl<T> ReplayClient for T where T: GameClient {}
