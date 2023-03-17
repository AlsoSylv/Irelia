use serde_json::Value;
use std::ffi::{c_char, c_int, CString, CStr};
use tokio::runtime::Runtime;

use crate::in_game::{InGameClient, TeamID};

#[derive(Debug)]
#[repr(C)]
pub struct InGameResponse {
    pub json: *mut c_char,
    pub error: c_int,
}

#[repr(C)]
pub struct NewInGame<'a> {
    pub client: *mut InGame<'a>,
    pub error: c_int,
}

pub struct InGame<'a> {
    client: InGameClient<'a>,
    rt: Runtime,
}

impl NewInGame<'_> {
    pub fn new() -> Self {
        match InGameClient::new() {
            Ok(client) => {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("PAIN");
                Self {
                    client: Box::into_raw(Box::new(InGame { client, rt })),
                    error: 0,
                }
            }
            Err(err) => Self {
                client: std::ptr::null_mut(),
                error: err as c_int,
            },
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn new_in_game<'a>() -> NewInGame<'a> {
    NewInGame::new()
}

#[no_mangle]
pub unsafe extern "C" fn all_game_data(client: *mut InGame) -> InGameResponse {
    in_game_live_client(client, "allgamedata", None)
}

#[no_mangle]
pub unsafe extern "C" fn active_player(client: *mut InGame) -> InGameResponse {
    in_game_live_client(client, "activeplayer", None)
}

#[no_mangle]
pub unsafe extern "C" fn active_player_name(client: *mut InGame) -> InGameResponse {
    in_game_live_client(client, "activeplayername", None)
}

#[no_mangle]
pub unsafe extern "C" fn active_player_abilities(client: *mut InGame) -> InGameResponse {
    in_game_live_client(client, "activeplayerabilities", None)
}

#[no_mangle]
pub unsafe extern "C" fn active_player_runes(client: *mut InGame) -> InGameResponse {
    in_game_live_client(client, "activeplayerrunes", None)
}

#[no_mangle]
pub unsafe extern "C" fn player_list(client: *mut InGame, team: *const TeamID) -> InGameResponse {
    if team.is_null() {
        in_game_live_client(client, "playerlist", None)
    } else {
        let team = match &*team {
            TeamID::ALL => "ALL",
            TeamID::UNKNOWN => "UNKNOWN",
            TeamID::ORDER => "ORDER",
            TeamID::CHAOS => "CHAOS",
            TeamID::NEUTRAL => "NEUTRAL",
        };
        let endpoint = format!("playerlist?teamID={}", team);
        in_game_live_client(client, &endpoint, None)
    }
}

#[no_mangle]
pub unsafe extern "C" fn player_scores(client: *mut InGame, summoner: *const c_char) -> InGameResponse {
    let summoner = CStr::from_ptr(summoner).to_string_lossy();
    in_game_live_client(client, "playerscores", Some(&summoner))
}

#[no_mangle]
pub unsafe extern "C" fn player_summoner_spells(client: *mut InGame, summoner: *const c_char) -> InGameResponse {
    let summoner = CStr::from_ptr(summoner).to_string_lossy();
    in_game_live_client(client, "playersummonerspells", Some(&summoner))
}

#[no_mangle]
pub unsafe extern "C" fn player_main_runes(client: *mut InGame, summoner: *const c_char) -> InGameResponse {
    let summoner = CStr::from_ptr(summoner).to_string_lossy();
    in_game_live_client(client, "playermainrunes", Some(&summoner))
}

#[no_mangle]
pub unsafe extern "C" fn player_items(client: *mut InGame, summoner: *const c_char) -> InGameResponse {
    let summoner = CStr::from_ptr(summoner).to_string_lossy();
    in_game_live_client(client, "playeritems", Some(&summoner))
}

#[no_mangle]
pub unsafe extern "C" fn event_data(client: *mut InGame, event_id: *const c_int) -> InGameResponse {
    let event_id = if !event_id.is_null() {
        format!("?eventID={}", &*event_id)
    } else {
        "".to_owned()
    };
    let endpoint = format!("eventdata{}", event_id);
    in_game_live_client(client, &endpoint, None)
}

#[no_mangle]
pub unsafe extern "C" fn game_stats(client: *mut InGame) -> InGameResponse {
    in_game_live_client(client, "gamestats", None)
}

#[no_mangle]
pub extern "C" fn in_game_drop(game: *mut InGame) {
    let client = unsafe { Box::from_raw(game) };
    drop(client)
}

#[no_mangle]
pub extern "C" fn in_game_drop_res(res: InGameResponse) {
    unsafe { CString::from_raw(res.json) };
}

unsafe fn in_game_live_client(
    client: *mut InGame,
    endpoint: &str,
    summoner: Option<&str>,
) -> InGameResponse {
    let client = &*client;

    let fut = client.client.live_client::<Value>(&endpoint, summoner);

    let res = client.rt.block_on(fut);
    match res {
        Ok(json) => {
            let string = json.to_string();
            let c_string = CString::new(string).unwrap();
            InGameResponse {
                json: c_string.into_raw(),
                error: 0,
            }
        }
        Err(err) => InGameResponse {
            json: std::ptr::null_mut(),
            error: err as c_int,
        },
    }
}
