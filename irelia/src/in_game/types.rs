//! This module is full of auto generated JSON for in game events
//!
//! if anything fails to serialize this module probably needs to
//! be updated to a newer version of the in-game API.

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllGameData {
    active_player: ActivePlayer,
    all_players: Box<[AllPlayer]>,
    events: Events,
    game_data: GameData,
}

impl AllGameData {
    #[must_use]
    pub fn active_player(&self) -> &ActivePlayer {
        &self.active_player
    }
    #[must_use]
    pub fn all_players(&self) -> &[AllPlayer] {
        &self.all_players
    }
    #[must_use]
    pub fn events(&self) -> &Events {
        &self.events
    }
    #[must_use]
    pub fn game_data(&self) -> &GameData {
        &self.game_data
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivePlayer {
    abilities: Abilities,
    champion_stats: ChampionStats,
    current_gold: f64,
    full_runes: FullRunes,
    level: i64,
    summoner_name: String,
}

impl ActivePlayer {
    #[must_use]
    pub fn abilities(&self) -> &Abilities {
        &self.abilities
    }
    #[must_use]
    pub fn champion_stats(&self) -> &ChampionStats {
        &self.champion_stats
    }
    #[must_use]
    pub fn current_gold(&self) -> f64 {
        self.current_gold
    }
    #[must_use]
    pub fn full_runes(&self) -> &FullRunes {
        &self.full_runes
    }
    #[must_use]
    pub fn level(&self) -> i64 {
        self.level
    }
    #[must_use]
    pub fn summoner_name(&self) -> &str {
        &self.summoner_name
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Abilities {
    passive: Passive,
    q: Ability,
    w: Ability,
    e: Ability,
    r: Ability,
}

impl Abilities {
    #[must_use]
    pub fn passive(&self) -> &Passive {
        &self.passive
    }
    #[must_use]
    pub fn q(&self) -> &Ability {
        &self.q
    }
    #[must_use]
    pub fn w(&self) -> &Ability {
        &self.w
    }
    #[must_use]
    pub fn e(&self) -> &Ability {
        &self.e
    }
    #[must_use]
    pub fn r(&self) -> &Ability {
        &self.r
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Passive {
    display_name: String,
    id: String,
    raw_description: String,
    raw_display_name: String,
}

impl Passive {
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
    #[must_use]
    pub fn raw_description(&self) -> &str {
        &self.raw_description
    }
    #[must_use]
    pub fn raw_display_name(&self) -> &str {
        &self.raw_display_name
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    ability_level: i64,
    display_name: String,
    id: String,
    raw_description: String,
    raw_display_name: String,
}

impl Ability {
    #[must_use]
    pub fn ability_level(&self) -> i64 {
        self.ability_level
    }
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
    #[must_use]
    pub fn raw_description(&self) -> &str {
        &self.raw_description
    }
    #[must_use]
    pub fn raw_display_name(&self) -> &str {
        &self.raw_display_name
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats {
    ability_power: f64,
    armor: f64,
    armor_penetration_flat: f64,
    armor_penetration_percent: f64,
    attack_damage: f64,
    attack_range: f64,
    attack_speed: f64,
    bonus_armor_penetration_percent: f64,
    bonus_magic_penetration_percent: f64,
    ability_haste: f64,
    crit_chance: f64,
    crit_damage: f64,
    current_health: f64,
    health_regen_rate: f64,
    life_steal: f64,
    magic_lethality: f64,
    magic_penetration_flat: f64,
    magic_penetration_percent: f64,
    magic_resist: f64,
    max_health: f64,
    move_speed: f64,
    physical_lethality: f64,
    resource_max: f64,
    resource_regen_rate: f64,
    resource_type: String,
    resource_value: f64,
    spell_vamp: f64,
    tenacity: f64,
}

impl ChampionStats {
    #[must_use]
    pub fn ability_power(&self) -> f64 {
        self.ability_power
    }
    #[must_use]
    pub fn armor(&self) -> f64 {
        self.armor
    }
    #[must_use]
    pub fn armor_penetration_flat(&self) -> f64 {
        self.armor_penetration_flat
    }
    #[must_use]
    pub fn armor_penetration_percent(&self) -> f64 {
        self.armor_penetration_percent
    }
    #[must_use]
    pub fn attack_damage(&self) -> f64 {
        self.attack_damage
    }
    #[must_use]
    pub fn attack_range(&self) -> f64 {
        self.attack_range
    }
    #[must_use]
    pub fn attack_speed(&self) -> f64 {
        self.attack_speed
    }
    #[must_use]
    pub fn bonus_armor_penetration_percent(&self) -> f64 {
        self.bonus_armor_penetration_percent
    }
    #[must_use]
    pub fn bonus_magic_penetration_percent(&self) -> f64 {
        self.bonus_magic_penetration_percent
    }
    #[must_use]
    pub fn ability_haste(&self) -> f64 {
        self.ability_haste
    }
    #[must_use]
    pub fn crit_chance(&self) -> f64 {
        self.crit_chance
    }
    #[must_use]
    pub fn crit_damage(&self) -> f64 {
        self.crit_damage
    }
    #[must_use]
    pub fn current_health(&self) -> f64 {
        self.current_health
    }
    #[must_use]
    pub fn health_regen_rate(&self) -> f64 {
        self.health_regen_rate
    }
    #[must_use]
    pub fn life_steal(&self) -> f64 {
        self.life_steal
    }
    #[must_use]
    pub fn magic_lethality(&self) -> f64 {
        self.magic_lethality
    }
    #[must_use]
    pub fn magic_penetration_flat(&self) -> f64 {
        self.magic_penetration_flat
    }
    #[must_use]
    pub fn magic_penetration_percent(&self) -> f64 {
        self.magic_penetration_percent
    }
    #[must_use]
    pub fn magic_resist(&self) -> f64 {
        self.magic_resist
    }
    #[must_use]
    pub fn max_health(&self) -> f64 {
        self.max_health
    }
    #[must_use]
    pub fn move_speed(&self) -> f64 {
        self.move_speed
    }
    #[must_use]
    pub fn physical_lethality(&self) -> f64 {
        self.physical_lethality
    }
    #[must_use]
    pub fn resource_max(&self) -> f64 {
        self.resource_max
    }
    #[must_use]
    pub fn resource_regen_rate(&self) -> f64 {
        self.resource_regen_rate
    }
    #[must_use]
    pub fn resource_type(&self) -> &str {
        &self.resource_type
    }
    #[must_use]
    pub fn resource_value(&self) -> f64 {
        self.resource_value
    }
    #[must_use]
    pub fn spell_vamp(&self) -> f64 {
        self.spell_vamp
    }
    #[must_use]
    pub fn tenacity(&self) -> f64 {
        self.tenacity
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullRunes {
    keystone: Rune,
    primary_rune_tree: Rune,
    secondary_rune_tree: Rune,
    general_runes: Box<[Rune]>,
    stat_runes: Box<[StatRune]>,
}

impl FullRunes {
    #[must_use]
    pub fn keystone(&self) -> &Rune {
        &self.keystone
    }
    #[must_use]
    pub fn primary_rune_tree(&self) -> &Rune {
        &self.primary_rune_tree
    }
    #[must_use]
    pub fn secondary_rune_tree(&self) -> &Rune {
        &self.secondary_rune_tree
    }
    #[must_use]
    pub fn general_runes(&self) -> &[Rune] {
        &self.general_runes
    }
    #[must_use]
    pub fn stat_runes(&self) -> &[StatRune] {
        &self.stat_runes
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rune {
    display_name: String,
    id: i64,
    raw_description: String,
    raw_display_name: String,
}

impl Rune {
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
    #[must_use]
    pub fn id(&self) -> i64 {
        self.id
    }
    #[must_use]
    pub fn raw_description(&self) -> &str {
        &self.raw_description
    }
    #[must_use]
    pub fn raw_display_name(&self) -> &str {
        &self.raw_display_name
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatRune {
    id: i64,
    raw_description: String,
}

impl StatRune {
    #[must_use]
    pub fn id(&self) -> i64 {
        self.id
    }
    #[must_use]
    pub fn raw_description(&self) -> &str {
        &self.raw_description
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllPlayer {
    champion_name: String,
    is_bot: bool,
    is_dead: bool,
    items: Box<[Item]>,
    level: i64,
    position: String,
    raw_champion_name: String,
    respawn_timer: f64,
    runes: Runes,
    scores: Scores,
    #[serde(rename = "skinID")]
    skin_id: i64,
    summoner_name: String,
    summoner_spells: SummonerSpells,
    team: String,
}

impl AllPlayer {
    #[must_use]
    pub fn champion_name(&self) -> &str {
        &self.champion_name
    }
    #[must_use]
    pub fn is_bot(&self) -> bool {
        self.is_bot
    }
    #[must_use]
    pub fn is_dead(&self) -> bool {
        self.is_dead
    }
    #[must_use]
    pub fn items(&self) -> &[Item] {
        &self.items
    }
    #[must_use]
    pub fn level(&self) -> i64 {
        self.level
    }
    #[must_use]
    pub fn position(&self) -> &str {
        &self.position
    }
    #[must_use]
    pub fn raw_champion_name(&self) -> &str {
        &self.raw_champion_name
    }
    #[must_use]
    pub fn respawn_timer(&self) -> f64 {
        self.respawn_timer
    }
    #[must_use]
    pub fn runes(&self) -> &Runes {
        &self.runes
    }
    #[must_use]
    pub fn scores(&self) -> &Scores {
        &self.scores
    }
    #[must_use]
    pub fn skin_id(&self) -> i64 {
        self.skin_id
    }
    #[must_use]
    pub fn summoner_name(&self) -> &str {
        &self.summoner_name
    }
    #[must_use]
    pub fn summoner_spells(&self) -> &SummonerSpells {
        &self.summoner_spells
    }
    #[must_use]
    pub fn team(&self) -> &str {
        &self.team
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runes {
    keystone: Rune,
    primary_rune_tree: Rune,
    secondary_rune_tree: Rune,
}

impl Runes {
    #[must_use]
    pub fn keystone(&self) -> &Rune {
        &self.keystone
    }
    #[must_use]
    pub fn primary_rune_tree(&self) -> &Rune {
        &self.primary_rune_tree
    }
    #[must_use]
    pub fn secondary_rune_tree(&self) -> &Rune {
        &self.secondary_rune_tree
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scores {
    assists: i64,
    creep_score: i64,
    deaths: i64,
    kills: i64,
    ward_score: f64,
}

impl Scores {
    #[must_use]
    pub fn assists(&self) -> i64 {
        self.assists
    }
    #[must_use]
    pub fn creep_score(&self) -> i64 {
        self.creep_score
    }
    #[must_use]
    pub fn deaths(&self) -> i64 {
        self.deaths
    }
    #[must_use]
    pub fn kills(&self) -> i64 {
        self.kills
    }
    #[must_use]
    pub fn ward_score(&self) -> f64 {
        self.ward_score
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpells {
    summoner_spell_one: SummonerSpell,
    summoner_spell_two: SummonerSpell,
}

impl SummonerSpells {
    #[must_use]
    pub fn summoner_spell_one(&self) -> &SummonerSpell {
        &self.summoner_spell_one
    }
    #[must_use]
    pub fn summoner_spell_two(&self) -> &SummonerSpell {
        &self.summoner_spell_two
    }
}

impl core::ops::Index<usize> for SummonerSpells {
    type Output = SummonerSpell;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => self.summoner_spell_one(),
            1 => self.summoner_spell_two(),
            e => panic!("Index Out Of Bounds, expected 0 or 1, but found {e}!"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpell {
    pub display_name: String,
    pub raw_description: String,
    pub raw_display_name: String,
}

impl SummonerSpell {
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
    #[must_use]
    pub fn raw_description(&self) -> &str {
        &self.raw_description
    }
    #[must_use]
    pub fn raw_display_name(&self) -> &str {
        &self.raw_display_name
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    can_use: bool,
    consumable: bool,
    count: i64,
    display_name: String,
    #[serde(rename = "itemID")]
    item_id: i64,
    price: i64,
    raw_description: String,
    raw_display_name: String,
    slot: i64,
}

impl Item {
    #[must_use]
    pub fn can_use(&self) -> bool {
        self.can_use
    }
    #[must_use]
    pub fn consumable(&self) -> bool {
        self.consumable
    }
    #[must_use]
    pub fn count(&self) -> i64 {
        self.count
    }
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
    #[must_use]
    pub fn item_id(&self) -> i64 {
        self.item_id
    }
    #[must_use]
    pub fn price(&self) -> i64 {
        self.price
    }
    #[must_use]
    pub fn raw_description(&self) -> &str {
        &self.raw_description
    }
    #[must_use]
    pub fn raw_display_name(&self) -> &str {
        &self.raw_display_name
    }
    #[must_use]
    pub fn slot(&self) -> i64 {
        self.slot
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Events {
    events: Box<[Event]>,
}

impl Events {
    #[must_use]
    pub fn events(&self) -> &[Event] {
        &self.events
    }
}

impl core::ops::Index<usize> for Events {
    type Output = Event;

    fn index(&self, index: usize) -> &Self::Output {
        &self.events[index]
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
    #[serde(rename = "EventID")]
    event_id: i64,
    event_name: String,
    event_time: f64,
}

impl Event {
    #[must_use]
    pub fn event_id(&self) -> i64 {
        self.event_id
    }
    #[must_use]
    pub fn event_name(&self) -> &str {
        &self.event_name
    }
    #[must_use]
    pub fn event_time(&self) -> f64 {
        self.event_time
    }
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

impl GameData {
    #[must_use]
    pub fn game_mode(&self) -> &str {
        &self.game_mode
    }
    #[must_use]
    pub fn game_time(&self) -> f64 {
        self.game_time
    }
    #[must_use]
    pub fn map_name(&self) -> &str {
        &self.map_name
    }
    #[must_use]
    pub fn map_number(&self) -> i64 {
        self.map_number
    }
    #[must_use]
    pub fn map_terrain(&self) -> &str {
        &self.map_terrain
    }
}

/// Enum representation of different team IDs
pub enum TeamID {
    ALL,
    UNKNOWN,
    ORDER,
    CHAOS,
    NEUTRAL,
}
