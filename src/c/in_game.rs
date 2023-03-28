use serde_json::Value;
use std::{
    ffi::{c_char, c_int, CStr, CString},
    ptr::{null_mut, NonNull},
};

use crate::{
    in_game::{InGameClient, TeamID},
    LcuResponse,
};

use super::{rest::CFuture, runtime::RT, utils::json_to_cstring};

/// SAFETY: Do not pass null pointers, do not
/// drop before futures are awaited or aborted
#[no_mangle]
pub unsafe extern "C" fn new_in_game(client: Option<NonNull<*mut InGameClient>>) -> LcuResponse {
    match InGameClient::new() {
        Ok(in_game_client) => {
            *client.unwrap().as_ptr() = Box::into_raw(Box::new(in_game_client));
            LcuResponse::Success
        }
        Err(err) => err,
    }
}

/// SAFETY: Do not pass null pointers, do not
/// drop before futures are awaited or aborted
#[no_mangle]
pub unsafe extern "C" fn all_game_data(
    client: Option<NonNull<InGameClient<'static>>>,
    rt: Option<NonNull<RT>>,
    func: extern "C" fn(*mut c_char, LcuResponse),
) -> *mut CFuture {
    in_game_live_client(client, rt, func, "allgamedata".to_owned(), None)
}

/// SAFETY: Do not pass null pointers, do not
/// drop before futures are awaited or aborted
#[no_mangle]
pub unsafe extern "C" fn active_player(
    client: Option<NonNull<InGameClient<'static>>>,
    rt: Option<NonNull<RT>>,
    func: extern "C" fn(*mut c_char, LcuResponse),
) -> *mut CFuture {
    in_game_live_client(client, rt, func, "activeplayer".to_owned(), None)
}

/// SAFETY: Do not pass null pointers, do not
/// drop before futures are awaited or aborted
#[no_mangle]
pub unsafe extern "C" fn active_player_name(
    client: Option<NonNull<InGameClient<'static>>>,
    rt: Option<NonNull<RT>>,
    func: extern "C" fn(*mut c_char, LcuResponse),
) -> *mut CFuture {
    in_game_live_client(client, rt, func, "activeplayername".to_owned(), None)
}

/// SAFETY: Do not pass null pointers, do not
/// drop before futures are awaited or aborted
#[no_mangle]
pub unsafe extern "C" fn active_player_abilities(
    client: Option<NonNull<InGameClient<'static>>>,
    rt: Option<NonNull<RT>>,
    func: extern "C" fn(*mut c_char, LcuResponse),
) -> *mut CFuture {
    in_game_live_client(client, rt, func, "activeplayerabilities".to_owned(), None)
}

/// SAFETY: Do not pass null pointers, do not
/// drop before futures are awaited or aborted
#[no_mangle]
pub unsafe extern "C" fn active_player_runes(
    client: Option<NonNull<InGameClient<'static>>>,
    rt: Option<NonNull<RT>>,
    func: extern "C" fn(*mut c_char, LcuResponse),
) -> *mut CFuture {
    in_game_live_client(client, rt, func, "activeplayerrunes".to_owned(), None)
}

/// SAFETY: Do not pass null pointers, do not
/// drop before futures are awaited or aborted
#[no_mangle]
pub unsafe extern "C" fn player_list(
    client: Option<NonNull<InGameClient<'static>>>,
    rt: Option<NonNull<RT>>,
    func: extern "C" fn(*mut c_char, LcuResponse),
    team: *const TeamID,
) -> *mut CFuture {
    if team.is_null() {
        in_game_live_client(client, rt, func, "playerlist".to_owned(), None)
    } else {
        let team = match &*team {
            TeamID::ALL => "ALL",
            TeamID::UNKNOWN => "UNKNOWN",
            TeamID::ORDER => "ORDER",
            TeamID::CHAOS => "CHAOS",
            TeamID::NEUTRAL => "NEUTRAL",
        };
        let endpoint = format!("playerlist?teamID={team}");
        in_game_live_client(client, rt, func, endpoint, None)
    }
}

/// SAFETY: Do not pass null pointers, do not
/// drop before futures are awaited or aborted
#[no_mangle]
pub unsafe extern "C" fn player_scores(
    client: Option<NonNull<InGameClient<'static>>>,
    rt: Option<NonNull<RT>>,
    func: extern "C" fn(*mut c_char, LcuResponse),
    summoner: *mut c_char,
) -> *mut CFuture {
    in_game_live_client(client, rt, func, "playerscores".to_owned(), Some(summoner))
}

/// SAFETY: Do not pass null pointers, do not
/// drop before futures are awaited or aborted
#[no_mangle]
pub unsafe extern "C" fn player_summoner_spells(
    client: Option<NonNull<InGameClient<'static>>>,
    rt: Option<NonNull<RT>>,
    func: extern "C" fn(*mut c_char, LcuResponse),
    summoner: *mut c_char,
) -> *mut CFuture {
    in_game_live_client(
        client,
        rt,
        func,
        "playersummonerspells".to_owned(),
        Some(summoner),
    )
}

/// SAFETY: Do not pass null pointers, do not
/// drop before futures are awaited or aborted
#[no_mangle]
pub unsafe extern "C" fn player_main_runes(
    client: Option<NonNull<InGameClient<'static>>>,
    rt: Option<NonNull<RT>>,
    func: extern "C" fn(*mut c_char, LcuResponse),
    summoner: *mut c_char,
) -> *mut CFuture {
    in_game_live_client(
        client,
        rt,
        func,
        "playermainrunes".to_owned(),
        Some(summoner),
    )
}

/// SAFETY: Do not pass null pointers, do not
/// drop before futures are awaited or aborted
#[no_mangle]
pub unsafe extern "C" fn player_items(
    client: Option<NonNull<InGameClient<'static>>>,
    rt: Option<NonNull<RT>>,
    func: extern "C" fn(*mut c_char, LcuResponse),
    summoner: *mut c_char,
) -> *mut CFuture {
    in_game_live_client(client, rt, func, "playeritems".to_owned(), Some(summoner))
}

/// SAFETY: Do not pass null pointers, do not
/// drop before futures are awaited or aborted
#[no_mangle]
pub unsafe extern "C" fn event_data(
    client: Option<NonNull<InGameClient<'static>>>,
    rt: Option<NonNull<RT>>,
    func: extern "C" fn(*mut c_char, LcuResponse),
    event_id: *const c_int,
) -> *mut CFuture {
    let event_id = if !event_id.is_null() {
        format!("?eventID={}", &*event_id)
    } else {
        "".to_owned()
    };
    let endpoint = format!("eventdata{event_id}");
    in_game_live_client(client, rt, func, endpoint, None)
}

/// SAFETY: Do not pass null pointers, do not
/// drop before futures are awaited or aborted
#[no_mangle]
pub unsafe extern "C" fn game_stats(
    client: Option<NonNull<InGameClient<'static>>>,
    rt: Option<NonNull<RT>>,
    func: extern "C" fn(*mut c_char, LcuResponse),
) -> *mut CFuture {
    in_game_live_client(client, rt, func, "gamestats".to_owned(), None)
}

/// Drops in game handle
/// SAFETY: Do not pass null pointers
#[no_mangle]
pub unsafe extern "C" fn in_game_drop(game: *mut *mut InGameClient) {
    drop(Box::from_raw(*game))
}

/// Drops the game response
/// SAFETY: Do not pass null pointers
#[no_mangle]
pub unsafe extern "C" fn in_game_drop_res(res: *mut *mut c_char) {
    drop(CString::from_raw(*res));
}

/// SAFETY: This function assumes that no pointers are null
/// and that the client will not be dropped before futures
/// are all completed
unsafe fn in_game_live_client(
    client: Option<NonNull<InGameClient<'static>>>,
    rt: Option<NonNull<RT>>,
    func: extern "C" fn(*mut c_char, LcuResponse),
    endpoint: String,
    summoner: Option<*mut c_char>,
) -> *mut CFuture {
    let client = client.unwrap().as_ref();
    let rt = &rt.unwrap().as_ref().rt;

    let summoner = summoner.map(|ptr| CStr::from_ptr(ptr));

    Box::into_raw(Box::new(CFuture {
        fut: rt.spawn(async move {
            let fut =
                client.live_client::<Value>(&endpoint, summoner.map(|ptr| ptr.to_str().unwrap()));

            let res = fut.await;
            match res {
                Ok(val) => func(json_to_cstring(val), LcuResponse::Success),
                Err(err) => func(null_mut(), err),
            };
        }),
    }))
}

#[cfg(test)]
mod tests {
    #[test]
    fn in_game_test() {}
}
