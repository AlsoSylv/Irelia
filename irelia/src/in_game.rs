//! Module for the `LoL` `in_game` API, docs have been copied from their [official counterparts](https://developer.riotgames.com/docs/lol#game-client-api)
//!
//! All types are all generated from the official JSON snippets

/// Types returned by the in game API
pub mod types;

use types::ActivePlayerOrNull;

use self::types::{
    Abilities, ActivePlayer, AllGameData, AllPlayer, Events, GameData, Item, Runes, Scores,
    SummonerSpells, TeamID,
};
use crate::Error;
use crate::in_game::sealed::GameClientInternal;
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
    ) -> impl Future<Output = Result<Self::Response, Error<Self::Error>>> + Send {
        self.socketv4_raw_request_template(
            URL,
            endpoint,
            "HEAD",
            None,
            None,
            crate::requests::RequestFmt::MsgPack,
        )
    }

    //noinspection SpellCheckingInspection
    /// Get all available data.
    ///
    /// A sample response can be found [here](https://static.developer.riotgames.com/docs/lol/liveclientdata_sample.json).
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    fn all_game_data(
        &self,
    ) -> impl Future<Output = Result<AllGameData, Error<Self::Error>>> + Send {
        self.live_client("allgamedata", None)
    }

    //noinspection SpellCheckingInspection
    /// Get all data about the active player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    fn active_player(
        &self,
    ) -> impl Future<Output = Result<Option<ActivePlayer>, Error<Self::Error>>> + Send {
        async {
            let active_player: ActivePlayerOrNull = self.live_client("activeplayer", None).await?;

            Ok(active_player.into_option())
        }
    }

    //noinspection SpellCheckingInspection
    /// Returns the player name.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    fn active_player_name(
        &self,
    ) -> impl Future<Output = Result<String, Error<Self::Error>>> + Send {
        self.live_client("activeplayername", None)
    }

    //noinspection SpellCheckingInspection
    /// Get Abilities for the active player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    fn active_player_abilities(
        &self,
    ) -> impl Future<Output = Result<Abilities, Error<Self::Error>>> + Send {
        self.live_client("activeplayerabilities", None)
    }

    //noinspection SpellCheckingInspection
    /// Retrieve the full list of runes for the active player.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    fn active_player_runes(
        &self,
    ) -> impl Future<Output = Result<Runes, Error<Self::Error>>> + Send {
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
    ) -> impl Future<Output = Result<Box<[AllPlayer]>, Error<Self::Error>>> + Send {
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
    ) -> impl Future<Output = Result<Scores, Error<Self::Error>>> + Send {
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
    ) -> impl Future<Output = Result<SummonerSpells, Error<Self::Error>>> + Send {
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
    ) -> impl Future<Output = Result<Runes, Error<Self::Error>>> + Send {
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
    ) -> impl Future<Output = Result<Box<[Item]>, Error<Self::Error>>> + Send {
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
    ) -> impl Future<Output = Result<Events, Error<Self::Error>>> + Send {
        let event_id = event_id.map_or(String::new(), |id| format!("?eventID={id}"));
        let endpoint = format!("eventdata{event_id}");
        async move { self.live_client(&endpoint, None).await }
    }

    //noinspection SpellCheckingInspection
    /// Basic data about the game.
    ///
    /// # Errors
    /// This will return an error if the game API is not running
    fn game_stats(&self) -> impl Future<Output = Result<GameData, Error<Self::Error>>> + Send {
        self.live_client("gamestats", None)
    }
}

mod sealed {
    use super::URL;
    use crate::{Error, utils::requests::RequestClientTrait};
    use serde::de::DeserializeOwned;
    use std::future::Future;

    pub trait GameClientInternal: RequestClientTrait + Sync {
        fn live_client<R: DeserializeOwned>(
            &self,
            endpoint: &str,
            riot_id: Option<&str>,
        ) -> impl Future<Output = Result<R, Error<Self::Error>>> + Send {
            async move {
                let endpoint = riot_id.map_or_else(
                    || format!("/liveclientdata/{endpoint}"),
                    |riot_id| format!("/liveclientdata/{endpoint}?riotId={riot_id}"),
                );

                let buf = self
                    .socketv4_request_template(
                        URL,
                        &endpoint,
                        "GET",
                        None,
                        None,
                        crate::requests::RequestFmt::MsgPack,
                    )
                    .await?;

                Ok(Self::decode_response_bytes(
                    buf,
                    crate::requests::RequestFmt::MsgPack,
                )?)
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
        ) -> impl Future<Output = Result<R, Error<Self::Error>>> + Send
        where
            R: DeserializeOwned,
        {
            async move {
                let body = Self::encode_body(body, crate::requests::RequestFmt::MsgPack)?;

                let buffer = self
                    .socketv4_request_template(
                        URL,
                        endpoint,
                        method,
                        body,
                        None,
                        crate::requests::RequestFmt::MsgPack,
                    )
                    .await?;

                Ok(Self::decode_response_bytes(
                    buffer,
                    crate::requests::RequestFmt::MsgPack,
                )?)
            }
        }
    }

    impl<T> GameClientInternal for T
    where
        T: RequestClientTrait + Sync,
        Self: Sync,
    {
    }
}

impl<T> GameClient for T where T: GameClientInternal {}
