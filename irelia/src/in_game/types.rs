//! This module is full of auto generated JSON for in game events
//!
//! if anything fails to serialize this module probably needs to
//! be updated to a newer version of the in-game API.

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllGameData {
    pub active_player: ActivePlayer,
    pub all_players: Vec<AllPlayer>,
    pub events: Events,
    pub game_data: GameData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivePlayer {
    pub abilities: Abilities,
    pub champion_stats: ChampionStats,
    pub current_gold: f64,
    pub full_runes: FullRunes,
    pub level: i64,
    pub summoner_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Abilities {
    #[serde(rename = "E")]
    pub e: E,
    #[serde(rename = "Passive")]
    pub passive: Passive,
    #[serde(rename = "Q")]
    pub q: Q,
    #[serde(rename = "R")]
    pub r: R,
    #[serde(rename = "W")]
    pub w: W,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct E {
    pub ability_level: i64,
    pub display_name: String,
    pub id: String,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Passive {
    pub display_name: String,
    pub id: String,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Q {
    pub ability_level: i64,
    pub display_name: String,
    pub id: String,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct R {
    pub ability_level: i64,
    pub display_name: String,
    pub id: String,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct W {
    pub ability_level: i64,
    pub display_name: String,
    pub id: String,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats {
    pub ability_power: f64,
    pub armor: f64,
    pub armor_penetration_flat: f64,
    pub armor_penetration_percent: f64,
    pub attack_damage: f64,
    pub attack_range: f64,
    pub attack_speed: f64,
    pub bonus_armor_penetration_percent: f64,
    pub bonus_magic_penetration_percent: f64,
    pub cooldown_reduction: f64,
    pub crit_chance: f64,
    pub crit_damage: f64,
    pub current_health: f64,
    pub health_regen_rate: f64,
    pub life_steal: f64,
    pub magic_lethality: f64,
    pub magic_penetration_flat: f64,
    pub magic_penetration_percent: f64,
    pub magic_resist: f64,
    pub max_health: f64,
    pub move_speed: f64,
    pub physical_lethality: f64,
    pub resource_max: f64,
    pub resource_regen_rate: f64,
    pub resource_type: String,
    pub resource_value: f64,
    pub spell_vamp: f64,
    pub tenacity: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullRunes {
    pub keystone: Keystone,
    pub primary_rune_tree: PrimaryRuneTree,
    pub secondary_rune_tree: SecondaryRuneTree,
    pub general_runes: Vec<GeneralRune>,
    pub stat_runes: Vec<StatRune>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralRune {
    pub display_name: String,
    pub id: i64,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keystone {
    pub display_name: String,
    pub id: i64,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryRuneTree {
    pub display_name: String,
    pub id: i64,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryRuneTree {
    pub display_name: String,
    pub id: i64,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatRune {
    pub id: i64,
    pub raw_description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllPlayer {
    pub champion_name: String,
    pub is_bot: bool,
    pub is_dead: bool,
    pub items: Vec<Item>,
    pub level: i64,
    pub position: String,
    pub raw_champion_name: String,
    pub respawn_timer: f64,
    pub runes: Runes,
    pub scores: Scores,
    #[serde(rename = "skinID")]
    pub skin_id: i64,
    pub summoner_name: String,
    pub summoner_spells: SummonerSpells,
    pub team: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runes {
    pub keystone: Keystone,
    pub primary_rune_tree: PrimaryRuneTree,
    pub secondary_rune_tree: SecondaryRuneTree,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scores {
    pub assists: i64,
    pub creep_score: i64,
    pub deaths: i64,
    pub kills: i64,
    pub ward_score: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpells {
    pub summoner_spell_one: SummonerSpellOne,
    pub summoner_spell_two: SummonerSpellTwo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpellOne {
    pub display_name: String,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpellTwo {
    pub display_name: String,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub can_use: bool,
    pub consumable: bool,
    pub count: i64,
    pub display_name: String,
    #[serde(rename = "itemID")]
    pub item_id: i64,
    pub price: i64,
    pub raw_description: String,
    pub raw_display_name: String,
    pub slot: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Events {
    #[serde(rename = "Events")]
    pub events: Vec<Event>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(rename = "EventID")]
    pub event_id: i64,
    #[serde(rename = "EventName")]
    pub event_name: String,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
    pub game_mode: String,
    pub game_time: f64,
    pub map_name: String,
    pub map_number: i64,
    pub map_terrain: String,
}

/// Enum representation of different team IDs
pub enum TeamID {
    ALL,
    UNKNOWN,
    ORDER,
    CHAOS,
    NEUTRAL,
}
