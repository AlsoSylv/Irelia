use serde_json::Value;
use std::{
    ffi::{c_char, c_int, CStr, CString},
    mem::ManuallyDrop,
    ptr::NonNull,
};

use crate::{
    in_game::{InGameClient, TeamID},
    LcuResponse,
};

use super::runtime::RT;

#[no_mangle]
pub unsafe extern "C" fn new_in_game(client: *mut ManuallyDrop<InGameClient>) -> LcuResponse {
    match InGameClient::new() {
        Ok(in_game_client) => {
            *client = ManuallyDrop::new(in_game_client);
            LcuResponse::Success
        }
        Err(err) => err,
    }
}

#[no_mangle]
pub unsafe extern "C" fn all_game_data(
    client: Option<NonNull<InGameClient>>,
    rt: Option<NonNull<RT>>,
    json: Option<NonNull<*mut c_char>>,
) -> LcuResponse {
    in_game_live_client(client, rt, json, "allgamedata", None)
}

#[no_mangle]
pub unsafe extern "C" fn active_player(
    client: Option<NonNull<InGameClient>>,
    rt: Option<NonNull<RT>>,
    json: Option<NonNull<*mut c_char>>,
) -> LcuResponse {
    in_game_live_client(client, rt, json, "activeplayer", None)
}

#[no_mangle]
pub unsafe extern "C" fn active_player_name(
    client: Option<NonNull<InGameClient>>,
    rt: Option<NonNull<RT>>,
    json: Option<NonNull<*mut c_char>>,
) -> LcuResponse {
    in_game_live_client(client, rt, json, "activeplayername", None)
}

#[no_mangle]
pub unsafe extern "C" fn active_player_abilities(
    client: Option<NonNull<InGameClient>>,
    rt: Option<NonNull<RT>>,
    json: Option<NonNull<*mut c_char>>,
) -> LcuResponse {
    in_game_live_client(client, rt, json, "activeplayerabilities", None)
}

#[no_mangle]
pub unsafe extern "C" fn active_player_runes(
    client: Option<NonNull<InGameClient>>,
    rt: Option<NonNull<RT>>,
    json: Option<NonNull<*mut c_char>>,
) -> LcuResponse {
    in_game_live_client(client, rt, json, "activeplayerrunes", None)
}

#[no_mangle]
pub unsafe extern "C" fn player_list(
    client: Option<NonNull<InGameClient>>,
    rt: Option<NonNull<RT>>,
    json: Option<NonNull<*mut c_char>>,
    team: *const TeamID,
) -> LcuResponse {
    if team.is_null() {
        in_game_live_client(client, rt, json, "playerlist", None)
    } else {
        let team = match &*team {
            TeamID::ALL => "ALL",
            TeamID::UNKNOWN => "UNKNOWN",
            TeamID::ORDER => "ORDER",
            TeamID::CHAOS => "CHAOS",
            TeamID::NEUTRAL => "NEUTRAL",
        };
        let endpoint = format!("playerlist?teamID={team}");
        in_game_live_client(client, rt, json, &endpoint, None)
    }
}

#[no_mangle]
pub unsafe extern "C" fn player_scores(
    client: Option<NonNull<InGameClient>>,
    rt: Option<NonNull<RT>>,
    json: Option<NonNull<*mut c_char>>,
    summoner: *const c_char,
) -> LcuResponse {
    let summoner = CStr::from_ptr(summoner).to_string_lossy();
    in_game_live_client(client, rt, json, "playerscores", Some(&summoner))
}

#[no_mangle]
pub unsafe extern "C" fn player_summoner_spells(
    client: Option<NonNull<InGameClient>>,
    rt: Option<NonNull<RT>>,
    json: Option<NonNull<*mut c_char>>,
    summoner: *const c_char,
) -> LcuResponse {
    let summoner = CStr::from_ptr(summoner).to_string_lossy();
    in_game_live_client(client, rt, json, "playersummonerspells", Some(&summoner))
}

#[no_mangle]
pub unsafe extern "C" fn player_main_runes(
    client: Option<NonNull<InGameClient>>,
    rt: Option<NonNull<RT>>,
    json: Option<NonNull<*mut c_char>>,
    summoner: *const c_char,
) -> LcuResponse {
    let summoner = CStr::from_ptr(summoner).to_string_lossy();
    in_game_live_client(client, rt, json, "playermainrunes", Some(&summoner))
}

#[no_mangle]
pub unsafe extern "C" fn player_items(
    client: Option<NonNull<InGameClient>>,
    rt: Option<NonNull<RT>>,
    json: Option<NonNull<*mut c_char>>,
    summoner: *const c_char,
) -> LcuResponse {
    let summoner = CStr::from_ptr(summoner).to_string_lossy();
    in_game_live_client(client, rt, json, "playeritems", Some(&summoner))
}

#[no_mangle]
pub unsafe extern "C" fn event_data(
    client: Option<NonNull<InGameClient>>,
    rt: Option<NonNull<RT>>,
    json: Option<NonNull<*mut c_char>>,
    event_id: *const c_int,
) -> LcuResponse {
    let event_id = if !event_id.is_null() {
        format!("?eventID={}", &*event_id)
    } else {
        "".to_owned()
    };
    let endpoint = format!("eventdata{event_id}");
    in_game_live_client(client, rt, json, &endpoint, None)
}

#[no_mangle]
pub unsafe extern "C" fn game_stats(
    client: Option<NonNull<InGameClient>>,
    rt: Option<NonNull<RT>>,
    json: Option<NonNull<*mut c_char>>,
) -> LcuResponse {
    in_game_live_client(client, rt, json, "gamestats", None)
}

/// Drops in game handle
#[no_mangle]
pub unsafe extern "C" fn in_game_drop(game: &mut ManuallyDrop<InGameClient>) {
    ManuallyDrop::drop(game)
}

/// Drops the game response
#[no_mangle]
pub unsafe extern "C" fn in_game_drop_res(res: *mut *mut c_char) {
    drop(CString::from_raw(*res));
}

unsafe fn in_game_live_client(
    client: Option<NonNull<InGameClient>>,
    rt: Option<NonNull<RT>>,
    c_json: Option<NonNull<*mut c_char>>,
    endpoint: &str,
    summoner: Option<&str>,
) -> LcuResponse {
    let client = client.unwrap().as_ref();
    let rt = &rt.unwrap().as_ref().rt;

    let fut = client.live_client::<Value>(endpoint, summoner);

    let res = rt.block_on(fut);
    match res {
        Ok(json) => {
            let string = json.to_string();
            let c_string = CString::new(string).unwrap();
            *c_json.unwrap().as_ptr() = c_string.into_raw();
            LcuResponse::Success
        }
        Err(err) => err,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn in_game_test() {}
}
