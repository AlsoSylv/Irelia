//! Module for the `LoL` `in_game` API, docs have been copied from their [official counterparts](https://developer.riotgames.com/docs/lol#game-client-api)
//!
//! All types are all generated from the official JSON snippets

/// Types returned by the in game API
pub mod types;

use self::types::{
    Abilities, ActivePlayer, AllGameData, AllPlayer, Events, GameData, Item, Runes, Scores,
    SummonerSpells, TeamID,
};
use crate::in_game::sealed::GameClientInternal;
use crate::{Error, RequestClient};
use hyper::body::Incoming;
use hyper::Response;
use std::future::Future;
use std::net::{Ipv4Addr, SocketAddrV4};

/// The only url the in game API can be used on
pub const URL: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 2999);

/// Trait that represents a connection to the in game api client
pub trait GameClient: GameClientInternal {
    /// This makes a head request to the in game API
    ///
    /// Please note, when using invalid methods (such as `HEAD`) with an Operation ID
    /// The API will always return 200 with an empty body, at least in my testing
    /// This can be used to see if the API is alive and if you're outside the loading screen
    ///
    /// # Errors
    /// This will return an error if it is unable to connect to the server
    fn head(
        &self,
        endpoint: &str,
    ) -> impl Future<Output = Result<Response<Incoming>, Error>> + Send {
        self.request_client()
            .raw_request_template(URL, endpoint, "HEAD", None, None)
    }

    //noinspection SpellCheckingInspection
    /// Get all available data.
    ///
    /// A sample response can be found [here](https://static.developer.riotgames.com/docs/lol/liveclientdata_sample.json).
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    fn all_game_data(&self) -> impl Future<Output = Result<AllGameData, Error>> + Send {
        self.live_client("allgamedata", None)
    }

    //noinspection SpellCheckingInspection
    /// Get all data about the active player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    fn active_player(&self) -> impl Future<Output = Result<ActivePlayer, Error>> + Send {
        self.live_client("activeplayer", None)
    }

    //noinspection SpellCheckingInspection
    /// Returns the player name.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    fn active_player_name(&self) -> impl Future<Output = Result<String, Error>> + Send {
        self.live_client("activeplayername", None)
    }

    //noinspection SpellCheckingInspection
    /// Get Abilities for the active player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    fn active_player_abilities(&self) -> impl Future<Output = Result<Abilities, Error>> + Send {
        self.live_client("activeplayerabilities", None)
    }

    //noinspection SpellCheckingInspection
    /// Retrieve the full list of runes for the active player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    fn active_player_runes(&self) -> impl Future<Output = Result<Runes, Error>> + Send {
        self.live_client("activeplayerrunes", None)
    }

    //noinspection SpellCheckingInspection
    /// Retrieve the list of heroes in the game and their stats.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    fn player_list(
        &self,
        team: Option<TeamID>,
    ) -> impl Future<Output = Result<Box<[AllPlayer]>, Error>> + Send {
        let endpoint = team.map_or_else(
            || "playerlist",
            |team| match team {
                TeamID::All => "playerlist?teamID=ALL",
                TeamID::Unknown => "playerlist?teamID=UNKNOWN",
                TeamID::Order => "playerlist?teamID=ORDER",
                TeamID::Chaos => "playerlist?teamID=CHAOS",
                TeamID::Neutral => "playerlist?teamID=NEUTRAL",
            },
        );

        self.live_client(endpoint, None)
    }

    //noinspection SpellCheckingInspection
    /// Retrieve the list of the current scores for the player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    fn player_scores(
        &self,
        summoner: impl AsRef<str> + Send,
    ) -> impl Future<Output = Result<Scores, Error>> + Send {
        async move {
            self.live_client("playerscores", Some(summoner.as_ref()))
                .await
        }
    }

    //noinspection SpellCheckingInspection
    /// Retrieve the list of the summoner spells for the player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    fn player_summoner_spells(
        &self,
        riot_id: impl AsRef<str> + Send,
    ) -> impl Future<Output = Result<SummonerSpells, Error>> + Send {
        async move {
            self.live_client("playersummonerspells", Some(riot_id.as_ref()))
                .await
        }
    }

    //noinspection SpellCheckingInspection
    /// Retrieve the basic runes of any player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    fn player_main_runes(
        &self,
        riot_id: impl AsRef<str> + Send,
    ) -> impl Future<Output = Result<Runes, Error>> + Send {
        async move {
            self.live_client("playermainrunes", Some(riot_id.as_ref()))
                .await
        }
    }

    //noinspection SpellCheckingInspection
    /// Retrieve the list of items for the player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    fn player_items(
        &self,
        riot_id: impl AsRef<str> + Send,
    ) -> impl Future<Output = Result<Box<[Item]>, Error>> + Send {
        async move {
            self.live_client("playeritems", Some(riot_id.as_ref()))
                .await
        }
    }

    //noinspection SpellCheckingInspection
    /// Get a list of events that have occurred in the game.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    fn event_data(
        &self,
        event_id: Option<i32>,
    ) -> impl Future<Output = Result<Events, Error>> + Send {
        let event_id = event_id.map_or(String::new(), |id| format!("?eventID={id}"));
        let endpoint = format!("eventdata{event_id}");
        async move { self.live_client(&endpoint, None).await }
    }

    //noinspection SpellCheckingInspection
    /// Basic data about the game.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    fn game_stats(&self) -> impl Future<Output = Result<GameData, Error>> + Send {
        self.live_client("gamestats", None)
    }
}

mod sealed {
    use super::URL;
    use crate::{Error, RequestClient};
    use serde::de::DeserializeOwned;
    use std::future::Future;

    pub trait GameClientInternal: Sync {
        fn request_client(&self) -> &RequestClient;

        fn live_client<R: DeserializeOwned>(
            &self,
            endpoint: &str,
            riot_id: Option<&str>,
        ) -> impl Future<Output = Result<R, Error>> + Send {
            async move {
                use hyper::body::Buf;

                let endpoint = riot_id.map_or_else(
                    || format!("/liveclientdata/{endpoint}"),
                    |riot_id| format!("/liveclientdata/{endpoint}?riotId={riot_id}"),
                );

                let buf = self
                    .request_client()
                    .request_template(URL, &endpoint, "GET", None::<()>, None)
                    .await?;

                Ok(rmp_serde::from_read(buf.aggregate().reader())?)
            }
        }

        #[cfg(feature = "replay")]
        /// Internal abstraction over `request_client`, this lets me cut out anything that only applies here,
        /// and reduce the usage in oneliners
        ///
        /// # Errors
        /// This will return an error if there is not an active replay running
        fn replay<R>(
            &self,
            endpoint: &'static str,
            method: &'static str,
            body: Option<impl serde::Serialize + Send>,
        ) -> impl Future<Output = Result<R, Error>> + Send
        where
            R: DeserializeOwned,
        {
            async move {
                use hyper::body::Buf;

                let buffer = self
                    .request_client()
                    .request_template(URL, endpoint, method, body, None)
                    .await?;

                Ok(rmp_serde::from_read(buffer.aggregate().reader())?)
            }
        }
    }

    impl GameClientInternal for RequestClient {
        fn request_client(&self) -> &RequestClient {
            self
        }
    }
}

impl GameClient for RequestClient {}
