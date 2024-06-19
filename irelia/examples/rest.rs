extern crate irelia;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;

use serde_derive::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let request_client = irelia::RequestClient::new();
    let lcu_client = irelia::rest::LcuClient::new(false).unwrap();

    let current_summoner: CurrentSummoner = lcu_client
        .get("/lol-summoner/v1/current-summoner", &request_client)
        .await
        .unwrap();

    println!("{current_summoner:?}");
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentSummoner {
    pub account_id: i64,
    pub display_name: String,
    pub game_name: String,
    pub internal_name: String,
    pub name_change_flag: bool,
    pub percent_complete_for_next_level: i64,
    pub privacy: String,
    pub profile_icon_id: i64,
    pub puuid: String,
    pub reroll_points: RerollPoints,
    pub summoner_id: i64,
    pub summoner_level: i64,
    pub tag_line: String,
    pub unnamed: bool,
    pub xp_since_last_level: i64,
    pub xp_until_next_level: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RerollPoints {
    pub current_points: i64,
    pub max_rolls: i64,
    pub number_of_rolls: i64,
    pub points_cost_to_roll: i64,
    pub points_to_reroll: i64,
}
