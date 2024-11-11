//! This module is full of auto generated JSON for in game events
//!
//! if anything fails to serialize this module probably needs to
//! be updated to a newer version of the in-game API.
//!
//! Well the types and returned values do not match, the format will serialize
//! to the same value

use rmpv::Value;
use serde::de::{Error, IgnoredAny, Unexpected, Visitor};
use serde::ser::SerializeStruct;
use serde::{
    Deserialize as DeserializeTrait, Deserializer, Serialize as SerializeTrait, Serializer,
};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt::Formatter;
use time::Duration;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllGameData {
    #[serde(deserialize_with = "deserialize_active_player")]
    active_player: Option<ActivePlayer>,
    all_players: Box<[AllPlayer]>,
    events: Events,
    game_data: GameData,
}

fn deserialize_active_player<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<ActivePlayer>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    #[allow(clippy::large_enum_variant)]
    enum ActivePlayerOrNull {
        ActivePlayer(Option<ActivePlayer>),
        Error {
            #[serde(rename = "error")]
            // This error will always be "This feature is not supported in spectator mode", so it can be ignored
            _error: IgnoredAny,
        },
    }

    let maybe_player = ActivePlayerOrNull::deserialize(deserializer)?;

    Ok(match maybe_player {
        ActivePlayerOrNull::ActivePlayer(player) => player,
        ActivePlayerOrNull::Error { .. } => None,
    })
}

impl AllGameData {
    #[must_use]
    pub const fn active_player(&self) -> Option<&ActivePlayer> {
        self.active_player.as_ref()
    }
    #[must_use]
    pub const fn all_players(&self) -> &[AllPlayer] {
        &self.all_players
    }
    #[must_use]
    pub const fn events(&self) -> &Events {
        &self.events
    }
    #[must_use]
    pub const fn game_data(&self) -> &GameData {
        &self.game_data
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivePlayer {
    abilities: Abilities,
    champion_stats: ChampionStats,
    current_gold: f64,
    full_runes: Runes,
    level: u8,
    #[serde(flatten)]
    riot_id: RiotId,
}

#[derive(Debug, Clone, PartialEq)]
struct RiotId {
    riot_id: Box<str>,
    separator_index: usize,
}

impl RiotId {
    #[must_use]
    const fn riot_id(&self) -> &str {
        &self.riot_id
    }
    #[must_use]
    fn game_name(&self) -> &str {
        &self.riot_id[0..self.separator_index]
    }
    #[must_use]
    fn tag_line(&self) -> &str {
        &self.riot_id[self.separator_index + 1..]
    }
    #[must_use]
    fn url_encoded_riot_id(&self) -> String {
        let game_name = self.game_name();
        let tag_line = self.tag_line();

        let len = game_name.len() + 1 + tag_line.len();
        let mut url_encoded = String::with_capacity(len);

        url_encoded.push_str(game_name);
        url_encoded.push('%');
        url_encoded.push_str(tag_line);

        url_encoded
    }
}

impl<'de> DeserializeTrait<'de> for RiotId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::struct_field_names)]
        struct TmpRiotId {
            riot_id: Box<str>,
        }

        let active_player = TmpRiotId::deserialize(deserializer)?;

        let separator_index = active_player.riot_id.find('#').ok_or_else(|| {
            Error::invalid_value(
                Unexpected::Other(&format!(
                    "Riot ID did not contain a separator '#', expected: GameName#TagLine, instead found found: {}",
                    active_player.riot_id
                )),
                &"A string in the format GameName#TagLine"
            )
        })?;

        Ok(Self {
            riot_id: active_player.riot_id,
            separator_index,
        })
    }
}

impl SerializeTrait for RiotId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut id_struct = serializer.serialize_struct("RiotId", 3)?;

        id_struct.serialize_field("RiotId", &self.riot_id)?;
        id_struct.serialize_field("RiotIdGameName", self.game_name())?;
        id_struct.serialize_field("RiotIdTagLine", self.tag_line())?;

        id_struct.end()
    }
}

impl ActivePlayer {
    #[must_use]
    pub const fn abilities(&self) -> &Abilities {
        &self.abilities
    }
    #[must_use]
    pub const fn champion_stats(&self) -> &ChampionStats {
        &self.champion_stats
    }
    #[must_use]
    pub const fn current_gold(&self) -> f64 {
        self.current_gold
    }
    #[must_use]
    pub const fn full_runes(&self) -> &Runes {
        &self.full_runes
    }
    #[must_use]
    pub const fn level(&self) -> u8 {
        self.level
    }
    #[must_use]
    pub const fn riot_id(&self) -> &str {
        self.riot_id.riot_id()
    }
    #[must_use]
    pub fn game_name(&self) -> &str {
        self.riot_id.game_name()
    }
    #[must_use]
    pub fn tag_line(&self) -> &str {
        self.riot_id.tag_line()
    }
    #[must_use]
    pub fn url_encoded_riot_id(&self) -> String {
        self.riot_id.url_encoded_riot_id()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Abilities {
    passive: AbilityInfo,
    q: Ability,
    w: Ability,
    e: Ability,
    r: Ability,
}

impl Abilities {
    #[must_use]
    pub const fn passive(&self) -> &AbilityInfo {
        &self.passive
    }
    #[must_use]
    pub const fn q(&self) -> &Ability {
        &self.q
    }
    #[must_use]
    pub const fn w(&self) -> &Ability {
        &self.w
    }
    #[must_use]
    pub const fn e(&self) -> &Ability {
        &self.e
    }
    #[must_use]
    pub const fn r(&self) -> &Ability {
        &self.r
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityInfo {
    display_name: Box<str>,
    id: Box<str>,
    raw_description: Box<str>,
    raw_display_name: Box<str>,
}

impl AbilityInfo {
    #[must_use]
    pub const fn display_name(&self) -> &str {
        &self.display_name
    }
    #[must_use]
    pub const fn id(&self) -> &str {
        &self.id
    }
    #[must_use]
    pub const fn raw_description(&self) -> &str {
        &self.raw_description
    }
    #[must_use]
    pub const fn raw_display_name(&self) -> &str {
        &self.raw_display_name
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    ability_level: u8,
    #[serde(flatten)]
    ability_info: AbilityInfo,
}

impl Ability {
    #[must_use]
    pub const fn ability_level(&self) -> u8 {
        self.ability_level
    }
    #[must_use]
    pub const fn ability_info(&self) -> &AbilityInfo {
        &self.ability_info
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats {
    ability_power: f64,
    armor: f64,
    armor_penetration_flat: f64,
    armor_penetration_percent: f64,
    ability_haste: f64,
    attack_damage: f64,
    attack_range: f64,
    attack_speed: f64,
    bonus_armor_penetration_percent: f64,
    bonus_magic_penetration_percent: f64,
    crit_chance: f64,
    crit_damage: f64,
    current_health: f64,
    heal_shield_power: f64,
    health_regen_rate: f64,
    life_steal: f64,
    magic_lethality: f64,
    magic_penetration_flat: f64,
    magic_penetration_percent: f64,
    magic_resist: f64,
    max_health: f64,
    move_speed: f64,
    #[serde(rename = "omnivamp")]
    omni_vamp: f64,
    physical_lethality: f64,
    physical_vamp: f64,
    resource_max: f64,
    resource_regen_rate: f64,
    resource_type: AbilityResource,
    resource_value: f64,
    spell_vamp: f64,
    tenacity: f64,
}

impl ChampionStats {
    #[must_use]
    pub const fn ability_power(&self) -> f64 {
        self.ability_power
    }
    #[must_use]
    pub const fn armor(&self) -> f64 {
        self.armor
    }
    #[must_use]
    pub const fn armor_penetration_flat(&self) -> f64 {
        self.armor_penetration_flat
    }
    #[must_use]
    pub const fn armor_penetration_percent(&self) -> f64 {
        self.armor_penetration_percent
    }
    #[must_use]
    pub const fn attack_damage(&self) -> f64 {
        self.attack_damage
    }
    #[must_use]
    pub const fn attack_range(&self) -> f64 {
        self.attack_range
    }
    #[must_use]
    pub const fn attack_speed(&self) -> f64 {
        self.attack_speed
    }
    #[must_use]
    pub const fn bonus_armor_penetration_percent(&self) -> f64 {
        self.bonus_armor_penetration_percent
    }
    #[must_use]
    pub const fn bonus_magic_penetration_percent(&self) -> f64 {
        self.bonus_magic_penetration_percent
    }
    #[must_use]
    pub const fn ability_haste(&self) -> f64 {
        self.ability_haste
    }
    #[must_use]
    pub const fn crit_chance(&self) -> f64 {
        self.crit_chance
    }
    #[must_use]
    pub const fn crit_damage(&self) -> f64 {
        self.crit_damage
    }
    #[must_use]
    pub const fn current_health(&self) -> f64 {
        self.current_health
    }
    #[must_use]
    pub const fn health_regen_rate(&self) -> f64 {
        self.health_regen_rate
    }
    #[must_use]
    pub const fn life_steal(&self) -> f64 {
        self.life_steal
    }
    #[must_use]
    pub const fn magic_lethality(&self) -> f64 {
        self.magic_lethality
    }
    #[must_use]
    pub const fn magic_penetration_flat(&self) -> f64 {
        self.magic_penetration_flat
    }
    #[must_use]
    pub const fn magic_penetration_percent(&self) -> f64 {
        self.magic_penetration_percent
    }
    #[must_use]
    pub const fn magic_resist(&self) -> f64 {
        self.magic_resist
    }
    #[must_use]
    pub const fn max_health(&self) -> f64 {
        self.max_health
    }
    #[must_use]
    pub const fn move_speed(&self) -> f64 {
        self.move_speed
    }
    #[must_use]
    pub const fn physical_lethality(&self) -> f64 {
        self.physical_lethality
    }
    #[must_use]
    pub const fn resource_max(&self) -> f64 {
        self.resource_max
    }
    #[must_use]
    pub const fn resource_regen_rate(&self) -> f64 {
        self.resource_regen_rate
    }
    #[must_use]
    pub const fn resource_type(&self) -> &AbilityResource {
        &self.resource_type
    }
    #[must_use]
    pub const fn resource_value(&self) -> f64 {
        self.resource_value
    }
    #[must_use]
    pub const fn spell_vamp(&self) -> f64 {
        self.spell_vamp
    }
    #[must_use]
    pub const fn tenacity(&self) -> f64 {
        self.tenacity
    }
    #[must_use]
    pub const fn heal_shield_power(&self) -> f64 {
        self.heal_shield_power
    }
    #[must_use]
    pub const fn omni_vamp(&self) -> f64 {
        self.omni_vamp
    }
    #[must_use]
    pub const fn physical_vamp(&self) -> f64 {
        self.physical_vamp
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runes {
    keystone: Rune,
    primary_rune_tree: Rune,
    secondary_rune_tree: Rune,
    general_runes: Option<[Rune; 6]>,
    stat_runes: Option<[StatRune; 3]>,
}

impl Runes {
    #[must_use]
    pub const fn keystone(&self) -> &Rune {
        &self.keystone
    }
    #[must_use]
    pub const fn primary_rune_tree(&self) -> &Rune {
        &self.primary_rune_tree
    }
    #[must_use]
    pub const fn secondary_rune_tree(&self) -> &Rune {
        &self.secondary_rune_tree
    }
    #[must_use]
    pub const fn general_runes(&self) -> Option<&[Rune; 6]> {
        self.general_runes.as_ref()
    }
    #[must_use]
    pub const fn stat_runes(&self) -> Option<&[StatRune; 3]> {
        self.stat_runes.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rune {
    display_name: Box<str>,
    // This never goes above 8500
    id: u16,
    raw_description: Box<str>,
    raw_display_name: Box<str>,
}

impl Rune {
    #[must_use]
    pub const fn display_name(&self) -> &str {
        &self.display_name
    }
    #[must_use]
    pub const fn id(&self) -> u16 {
        self.id
    }
    #[must_use]
    pub const fn raw_description(&self) -> &str {
        &self.raw_description
    }
    #[must_use]
    pub const fn raw_display_name(&self) -> &str {
        &self.raw_display_name
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatRune {
    // These are around 5000
    id: u16,
    raw_description: Box<str>,
}

impl StatRune {
    #[must_use]
    pub const fn id(&self) -> u16 {
        self.id
    }
    #[must_use]
    pub const fn raw_description(&self) -> &str {
        &self.raw_description
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllPlayer {
    champion_name: Box<str>,
    is_bot: bool,
    is_dead: bool,
    #[serde(with = "fixed_option_array")]
    items: [Option<Item>; 7],
    level: u8,
    position: Position,
    raw_champion_name: Box<str>,
    #[serde(with = "duration")]
    respawn_timer: Duration,
    runes: Runes,
    scores: Scores,
    #[serde(rename = "skinID")]
    skin_id: i64,
    #[serde(flatten)]
    riot_id: RiotId,
    summoner_spells: SummonerSpells,
    team: TeamID,
    skin_name: Option<Box<str>>,
    raw_skin_name: Option<Box<str>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Position {
    Top,
    Jungle,
    Middle,
    Bottom,
    #[serde(rename = "UTILITY")]
    Support,
    None,
    #[serde(untagged)]
    Unknown(Box<str>),
}

impl AllPlayer {
    #[must_use]
    pub const fn champion_name(&self) -> &str {
        &self.champion_name
    }
    #[must_use]
    pub const fn is_bot(&self) -> bool {
        self.is_bot
    }
    #[must_use]
    pub const fn is_dead(&self) -> bool {
        self.is_dead
    }
    #[must_use]
    pub const fn items(&self) -> &[Option<Item>; 7] {
        &self.items
    }
    #[must_use]
    pub fn purchased_items(&self) -> &[Option<Item>] {
        &self.items[0..6]
    }
    #[must_use]
    pub const fn ward(&self) -> &Option<Item> {
        &self.items[6]
    }
    #[must_use]
    pub const fn level(&self) -> u8 {
        self.level
    }
    #[must_use]
    pub const fn position(&self) -> &Position {
        &self.position
    }
    #[must_use]
    pub const fn raw_champion_name(&self) -> &str {
        &self.raw_champion_name
    }
    #[must_use]
    pub const fn respawn_timer(&self) -> Duration {
        self.respawn_timer
    }
    #[must_use]
    pub const fn runes(&self) -> &Runes {
        &self.runes
    }
    #[must_use]
    pub const fn scores(&self) -> &Scores {
        &self.scores
    }
    #[must_use]
    pub const fn skin_id(&self) -> i64 {
        self.skin_id
    }
    #[must_use]
    pub const fn riot_id(&self) -> &str {
        self.riot_id.riot_id()
    }
    #[must_use]
    pub fn game_name(&self) -> &str {
        self.riot_id.game_name()
    }
    #[must_use]
    pub fn tag_line(&self) -> &str {
        self.riot_id.tag_line()
    }
    #[must_use]
    pub const fn summoner_spells(&self) -> &SummonerSpells {
        &self.summoner_spells
    }
    #[must_use]
    pub const fn team(&self) -> &TeamID {
        &self.team
    }
    #[must_use]
    /// These will be `None` in spectator mode
    pub fn raw_skin_name(&self) -> Option<&str> {
        self.raw_skin_name.as_deref()
    }
    #[must_use]
    /// These will be `None` in spectator mode
    pub fn skin_name(&self) -> Option<&str> {
        self.skin_name.as_deref()
    }
    #[must_use]
    pub fn url_encoded_riot_id(&self) -> String {
        self.riot_id.url_encoded_riot_id()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scores {
    kills: u8,
    deaths: u8,
    assists: u8,
    creep_score: u16,
    ward_score: f64,
}

impl Scores {
    #[must_use]
    pub const fn kills(&self) -> u8 {
        self.kills
    }
    #[must_use]
    pub const fn deaths(&self) -> u8 {
        self.deaths
    }
    #[must_use]
    pub const fn assists(&self) -> u8 {
        self.assists
    }
    #[must_use]
    pub const fn creep_score(&self) -> u16 {
        self.creep_score
    }
    #[must_use]
    pub const fn ward_score(&self) -> f64 {
        self.ward_score
    }
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    /// This should match how it's displayed on the tab screen
    pub const fn ward_score_u64(&self) -> u64 {
        self.ward_score as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpells {
    summoner_spell_one: SummonerSpell,
    summoner_spell_two: SummonerSpell,
}

impl SummonerSpells {
    #[must_use]
    pub const fn summoner_spell_one(&self) -> &SummonerSpell {
        &self.summoner_spell_one
    }
    #[must_use]
    pub const fn summoner_spell_two(&self) -> &SummonerSpell {
        &self.summoner_spell_two
    }
}

impl core::ops::Index<usize> for SummonerSpells {
    type Output = SummonerSpell;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => self.summoner_spell_one(),
            1 => self.summoner_spell_two(),
            _ => panic!("Index Out Of Bounds, expected 0 or 1, but found {index}!"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpell {
    display_name: Box<str>,
    raw_description: Box<str>,
    raw_display_name: Box<str>,
}

impl SummonerSpell {
    #[must_use]
    pub const fn display_name(&self) -> &str {
        &self.display_name
    }
    #[must_use]
    pub const fn raw_description(&self) -> &str {
        &self.raw_description
    }
    #[must_use]
    pub const fn raw_display_name(&self) -> &str {
        &self.raw_display_name
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    can_use: bool,
    consumable: bool,
    // The max consumable stack is 5
    count: u8,
    display_name: Box<str>,
    #[serde(rename = "itemID")]
    item_id: u32,
    // This price never goes over 8000
    price: u16,
    raw_description: Box<str>,
    raw_display_name: Box<str>,
    // This is a value between 1 and 7
    slot: u8,
}

impl Item {
    #[must_use]
    pub const fn can_use(&self) -> bool {
        self.can_use
    }
    #[must_use]
    pub const fn consumable(&self) -> bool {
        self.consumable
    }
    #[must_use]
    pub const fn count(&self) -> u8 {
        self.count
    }
    #[must_use]
    pub const fn display_name(&self) -> &str {
        &self.display_name
    }
    #[must_use]
    pub const fn item_id(&self) -> u32 {
        self.item_id
    }
    #[must_use]
    pub const fn price(&self) -> u16 {
        self.price
    }
    #[must_use]
    pub const fn raw_description(&self) -> &str {
        &self.raw_description
    }
    #[must_use]
    pub const fn raw_display_name(&self) -> &str {
        &self.raw_display_name
    }
    #[must_use]
    pub const fn slot(&self) -> u8 {
        self.slot
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Events {
    events: Box<[Event]>,
}

impl Events {
    #[must_use]
    pub fn dragons_killed(&self) -> u8 {
        self.events.iter().fold(0, |acc, event| {
            acc + u8::from(matches!(
                event.event_details,
                EventDetails::DragonKill { .. }
            ))
        })
    }
    #[must_use]
    pub fn harold_killed(&self) -> bool {
        self.events
            .iter()
            .any(|event| matches!(event.event_details, EventDetails::HeraldKill(_)))
    }
    #[must_use]
    pub fn grub_groups_killed(&self) -> u8 {
        self.events.iter().fold(0, |acc, event| {
            acc + u8::from(matches!(event.event_details, EventDetails::HordeKill(_)))
        })
    }
    #[must_use]
    pub fn barons_killed(&self) -> u8 {
        self.events.iter().fold(0, |acc, event| {
            acc + u8::from(matches!(event.event_details, EventDetails::BaronKill(_)))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
    #[serde(rename = "EventID")]
    event_id: i64,
    #[serde(with = "duration")]
    event_time: Duration,
    #[serde(flatten)]
    event_details: EventDetails,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all_fields = "PascalCase")]
#[serde(tag = "EventName")]
pub enum EventDetails {
    GameStart,
    MinionsSpawning,
    Ace {
        acer: Box<str>,
        acing_team: TeamID,
    },
    ChampionKill {
        #[serde(flatten)]
        kill_info: KillInfo,
        victim_name: Box<str>,
    },
    FirstBlood {
        recipient: Box<str>,
    },
    #[serde(rename = "Multikill")]
    MultiKill {
        kill_streak: u16,
        killer_name: Box<str>,
    },
    TurretKilled {
        #[serde(flatten)]
        kill_info: KillInfo,
        turret_killed: Structure,
    },
    FirstBrick {
        killer_name: Box<str>,
    },
    DragonKill {
        dragon_type: DragonType,
        #[serde(flatten)]
        kill_info: MonsterKill,
    },
    HordeKill(MonsterKill),
    HeraldKill(MonsterKill),
    BaronKill(MonsterKill),
    InhibKilled {
        #[serde(flatten)]
        kill_info: KillInfo,
        inhib_killed: Structure,
    },
    InhibRespawned {
        inhib_respawned: Structure,
    },
    GameEnd {
        result: Box<str>,
    },
    #[serde(untagged)]
    Unknown(Value),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DragonType {
    Fire,
    Earth,
    Water,
    Air,
    Hextech,
    Chemtech,
    Elder,
    #[serde(untagged)]
    Unknown(Box<str>),
}

/// This represents all the data concerning a Turret or Inhibitor
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Structure {
    /// This is either `StructureType::Turret` or `StructureType::Barracks` aka inhibitor
    structure_type: StructureType,
    /// Blue side is `TeamID::Order`,
    /// Red side is `TeamID::Chaos`
    team_id: TeamID,
    /// This is either `Lane::Top`, `Lane::Mid` or `Lane::Bot`
    /// ARAM:
    /// - `TeamID::Order` is `Lane::Mid`
    /// - `TeamID::Chaos` is `Lane::Top`
    lane: Lane,
    /// The place of inhibitors are always 1.
    ///
    /// The place of turrets is map dependent, a guide has been laid out below.
    ///
    /// On summoners rift side-lanes are 1 through 3, and mid-lane is 1 through 5
    /// The outermost turret is always the greatest integer, while the innermost is always 1
    ///
    /// On Nexus blitz turrets are between 4 and 1, the order being:
    /// Top left - 4
    /// Bottom left - 3
    /// Top right - 2
    /// Bottom right - 1
    ///
    /// On Aram the order is side specific
    ///
    /// Red side is laid out as:
    /// Outermost Turret - 1
    /// Inhibitor Turret - 2
    /// Top Nexus Turret - 3
    /// Bot Nexus Turret - 4
    ///
    /// Blue side is laid out as:
    /// Outermost Turret - 8
    /// Inhibitor Turret - 7
    /// Top Nexus Turret - 10
    /// Bot Nexus Turret - 9
    ///
    /// <div>
    /// <img src="https://raw.githubusercontent.com/AlsoSylv/Irelia/master/irelia/src/in_game/StructureNames.png" />
    /// </div>
    place: u8,
}

impl Structure {
    #[must_use]
    /// Returns true if the structure is a turret and false otherwise
    pub fn is_turret(&self) -> bool {
        self.structure_type == StructureType::Turret
    }

    #[must_use]
    /// Returns true if the structure is an inhibitor and false otherwise
    pub fn is_inhibitor(&self) -> bool {
        self.structure_type == StructureType::Barracks
    }

    #[must_use]
    /// Either Turret or Barracks (aka inhibitor)
    pub const fn structure_type(&self) -> &StructureType {
        &self.structure_type
    }

    #[must_use]
    /// This still exists in aram, but it's useless
    pub const fn lane(&self) -> &Lane {
        &self.lane
    }

    #[must_use]
    /// Returns true if the team the structure belongs to is blue-side, aka Order
    pub fn is_blue_side(&self) -> bool {
        self.team_id == TeamID::Order
    }

    #[must_use]
    /// Returns true if the team the structure belongs to is red-side, aka Chaos
    pub fn is_red_side(&self) -> bool {
        self.team_id == TeamID::Chaos
    }

    #[must_use]
    /// Blue side = Order
    /// Red side = Chaos
    pub const fn team_id(&self) -> &TeamID {
        &self.team_id
    }

    #[must_use]
    /// <h1>Inhibitors: </h1>
    /// The place of inhibitors is always 1
    ///
    /// <h1>Turrets: </h1>
    /// The place of turrets is map dependent
    ///
    /// <h2> Summoners Rift: </h2>
    /// <h3> Side Lanes: </h3>
    /// <ul>
    ///     <li> Outermost Turret - 3 </li>
    ///     <li> Middle Turret - 2 </li>
    ///     <li> Inhibitor Turret - 1 </li>
    /// </ul>
    /// <h3> Mid-Lane: </h3>
    /// <ul>
    ///     <li> Outermost Turret - 5 </li>
    ///     <li> Middle Turret - 4 </li>
    ///     <li> Inhibitor Turret - 3 </li>
    ///     <li> Top Nexus Turret - 2 </li>   
    ///     <li> Bot Nexus Turret - 1 </li>
    /// </ul>
    ///
    /// <h2> Nexus Blitz: </h2>
    /// <ul>
    ///     <li> Top left - 4 </li>
    ///     <li> Bottom left - 3 </li>
    ///     <li> Top right - 2 </li>
    ///     <li> Bottom right - 1 </li>
    /// </ul>
    ///
    /// <h2> Aram: </h2>
    /// <h3> Red Side: </h3>
    /// <ul>
    ///     <li> Outermost Turret - 1 </li>
    ///     <li> Inhibitor Turret - 2 </li>
    ///     <li> Top Nexus Turret - 3 </li>
    ///     <li> Bot Nexus Turret - 4 </li>
    /// </ul>
    /// <h3> Blue Side: </h3>
    /// <ul>
    ///     <li> Outermost Turret - 8 </li>
    ///     <li> Inhibitor Turret - 7 </li>
    ///     <li> Top Nexus Turret - 10 </li>
    ///     <li> Bot Nexus Turret - 9 </li>
    /// </ul>
    ///
    /// <img src="https://raw.githubusercontent.com/AlsoSylv/Irelia/master/irelia/src/in_game/StructureNames.png" width="600" height = "200"/>
    pub const fn place(&self) -> u8 {
        self.place
    }

    #[must_use]
    /// Using the information above, this returns an enum describing the position of the Structure relative to the map
    ///
    /// Inhibitors always return `StructurePlace::Inner`
    ///
    /// In Nexus Blitz, Turrets are either inner or outer
    ///
    /// Returns `None` if the map has no structures
    pub fn place_determined(&self, map: &MapName) -> Option<StructurePlace> {
        if self.is_inhibitor() {
            return Some(StructurePlace::Inner);
        }

        match map {
            MapName::SummonersRift | MapName::TutorialMap => {
                match (self.place, self.lane == Lane::Mid) {
                    (5, true) | (3, false) => StructurePlace::Outer,
                    (4, true) | (2, false) => StructurePlace::Middle,
                    (3, true) | (1, false) => StructurePlace::Inner,
                    (2, true) => StructurePlace::TopNexus,
                    (1, true) => StructurePlace::BotNexus,
                    _ => unreachable!("Side lanes have three turrets, while mid has five"),
                }.into()
            }
            MapName::HowlingAbyss => match self.place {
                1 | 8 => StructurePlace::Outer,
                2 | 7 => StructurePlace::Inner,
                3 | 10 => StructurePlace::TopNexus,
                4 | 9 => StructurePlace::BotNexus,
                _ => unreachable!("At the time of writing, aram has 4 towers on each side"),
            }.into(),
            MapName::NexusBlitz => match self.place {
                1 | 2 => StructurePlace::Inner,
                3 | 4 => StructurePlace::Outer,
                _ => unreachable!("Nexus Blitz only has four turrets"),
            }.into(),
            MapName::Arena | MapName::TFT | MapName::Swarm => None,
            MapName::TwistedTreeline => unimplemented!("This game mode does not exist but let me hope"),
            MapName::Other(other) => unimplemented!(
                "Map {other} is new and unsupported, report this on github and it will be fixed"
            ),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Structure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        fn determine_structure_team(team: &str) -> TeamID {
            match team {
                "T1" => TeamID::Order,
                "T2" => TeamID::Chaos,
                team => unreachable!("Expected T1 or T2, found: {:?}", team),
            }
        }

        fn determine_structure_lane(lane: u8) -> Lane {
            match lane {
                b'L' => Lane::Top,
                b'C' => Lane::Mid,
                b'R' => Lane::Bot,
                unrecognized => unreachable!("{}", unrecognized),
            }
        }

        fn determine_structure_type(ty: &str) -> StructureType {
            match ty {
                "Turret" => StructureType::Turret,
                "Barracks" => StructureType::Barracks,
                unrecognized => unreachable!("{}", unrecognized),
            }
        }

        struct StructureVisitor;

        impl Visitor<'_> for StructureVisitor {
            type Value = Structure;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A string in one of the formats in StructureNames.png")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let mut split = v.split('_');

                let structure_type = split
                    .next()
                    .expect("The first string in the split is the structure name");

                let team = split
                    .next()
                    .expect("The second string in the split is the team");

                let lane = split
                    .next()
                    .expect("The third string in the split is the lane");

                let structure_type = determine_structure_type(structure_type);
                // This is always a single byte
                let lane = determine_structure_lane(lane.as_bytes()[0]);
                let team_id = determine_structure_team(team);

                let place = if structure_type == StructureType::Turret {
                    let place = split
                        .next()
                        .expect("The fourth string in the split is the place");
                    place.parse().expect("This is always a number")
                } else {
                    1
                };

                Ok(Structure {
                    structure_type,
                    team_id,
                    lane,
                    place,
                })
            }
        }

        deserializer.deserialize_any(StructureVisitor)
    }
}

impl SerializeTrait for Structure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let lane = match &self.lane {
            Lane::Bot => "R",
            Lane::Mid => "C",
            Lane::Top => "L",
        };

        let team = match self.team_id {
            TeamID::Order => "T1",
            TeamID::Chaos => "T2",
            _ => unreachable!(),
        };

        let place = self.place;

        let str = if self.structure_type == StructureType::Barracks {
            format!("Barracks_{team}_{lane}1")
        } else {
            format!("Turret_{team}_{lane}_0{place}_A")
        };

        serializer.serialize_str(&str)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructureType {
    Turret,
    Barracks,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lane {
    Top,
    Mid,
    Bot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructurePlace {
    Outer,
    Middle,
    Inner,
    TopNexus,
    BotNexus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct KillInfo {
    #[serde(with = "option_slice")]
    assisters: Option<Box<[String]>>,
    killer_name: String,
}

impl KillInfo {
    #[must_use]
    pub fn assisters(&self) -> Option<&[String]> {
        self.assisters.as_deref()
    }
    #[must_use]
    pub fn killer_name(&self) -> &str {
        &self.killer_name
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MonsterKill {
    #[serde(flatten)]
    kill_info: KillInfo,
    #[serde(with = "string_to_bool")]
    stolen: bool,
}

impl MonsterKill {
    #[must_use]
    pub const fn kill_info(&self) -> &KillInfo {
        &self.kill_info
    }
    #[must_use]
    pub const fn stolen(&self) -> bool {
        self.stolen
    }
}

impl Event {
    #[must_use]
    pub const fn event_id(&self) -> i64 {
        self.event_id
    }
    #[must_use]
    pub const fn event_time(&self) -> Duration {
        self.event_time
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
    game_mode: GameMode,
    #[serde(with = "duration")]
    game_time: Duration,
    map_name: MapName,
    map_number: u8,
    map_terrain: MapTerrain,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum GameMode {
    #[serde(rename = "CLASSIC")]
    SummonersRift,
    #[serde(rename = "ODIN", alias = "CREPE")]
    Aram,
    Tutorial,
    #[serde(rename = "TUTORIAL_MODULE_1")]
    /// Part 1 of the tutorial
    Tutorial1,
    #[serde(rename = "TUTORIAL_MODULE_2")]
    /// Part 2 of the tutorial
    Tutorial2,
    #[serde(rename = "TUTORIAL_MODULE_3")]
    /// Part 3 of the tutorial
    Tutorial3,
    Urf,
    PracticeTool,
    OneForAll,
    #[serde(alias = "GAMEMODEX")]
    NexusBlitz,
    #[serde(rename = "ULTBOOK")]
    UltimateSpellbook,
    #[serde(rename = "CHERRY")]
    Arena,
    #[serde(rename = "STRAWBERRY")]
    Swarn,
    /// If this variant pops up, see the riot docs at <https://static.developer.riotgames.com/docs/lol/gameModes.json>
    /// However, this may not be up-to-date
    #[serde(untagged)]
    Other(Box<str>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MapName {
    #[serde(rename = "Map3")]
    TutorialMap,
    #[serde(rename = "Map10")]
    TwistedTreeline,
    #[serde(rename = "Map11")]
    SummonersRift,
    #[serde(rename = "Map12")]
    HowlingAbyss,
    #[serde(rename = "Map21")]
    NexusBlitz,
    #[serde(rename = "Map22")]
    TFT,
    #[serde(rename = "Map30")]
    Arena,
    #[serde(rename = "Map33")]
    Swarm,
    /// If this variant pops up, see the riot docs at <https://static.developer.riotgames.com/docs/lol/maps.json>
    /// However, this may be out of date, if that's the case, look at <https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/maps.json>
    /// for the latest maps that patch
    #[serde(untagged)]
    Other(Box<str>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MapTerrain {
    Default,
    Infernal,
    Ocean,
    Mountain,
    Cloud,
    Hextech,
    Chemtech,
}

impl GameData {
    #[must_use]
    pub const fn game_mode(&self) -> &GameMode {
        &self.game_mode
    }
    #[must_use]
    pub const fn game_time(&self) -> Duration {
        self.game_time
    }
    #[must_use]
    pub const fn map_name(&self) -> &MapName {
        &self.map_name
    }
    #[must_use]
    pub const fn map_number(&self) -> u8 {
        self.map_number
    }
    #[must_use]
    pub const fn map_terrain(&self) -> &MapTerrain {
        &self.map_terrain
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
/// Enum representation of different team IDs
pub enum TeamID {
    All,
    Order,
    Chaos,
    Neutral,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
/// Ability Resource
pub enum AbilityResource {
    Mana,
    Energy,
    None,
    Shield,
    BattleFury,
    DragonFury,
    Rage,
    Heat,
    GnarFury,
    Ferocity,
    BloodWell,
    Wind,
    Ammo,
    MoonLight,
    Max,
    #[serde(other)]
    Other,
}

mod string_to_bool {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let stolen = String::deserialize(deserializer)?;

        Ok(match stolen.as_str() {
            "False" => false,
            "True" => true,
            _ => unreachable!(),
        })
    }

    // This has to be passed by ref to work with serde
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S: Serializer>(stolen: &bool, serializer: S) -> Result<S::Ok, S::Error> {
        let value = if *stolen { "True" } else { "False" };
        serializer.serialize_str(value)
    }
}

mod fixed_option_array {
    use crate::in_game::types::Item;
    use serde::de::{SeqAccess, Visitor};
    use serde::ser::SerializeSeq;
    use serde::{Deserializer, Serializer};
    use std::fmt::Formatter;

    pub fn serialize<S: Serializer>(
        items: &[Option<Item>; 7],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(None)?;

        for item in items.iter().flatten() {
            seq.serialize_element(item)?;
        }

        seq.end()
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<[Option<Item>; 7], D::Error> {
        struct SequenceVisitor;

        impl<'a> Visitor<'a> for SequenceVisitor {
            type Value = [Option<Item>; 7];

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("Expecting an array between 1 and 7")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'a>,
            {
                let mut arr: [Option<Item>; 7] = std::array::from_fn(|_| None);

                while let Some(item) = seq.next_element::<Item>()? {
                    let tmp = item.slot;
                    arr[tmp as usize] = Some(item);
                }

                Ok(arr)
            }
        }

        deserializer.deserialize_seq(SequenceVisitor)
    }
}

mod option_slice {
    use serde::ser::SerializeSeq;
    use serde::{Deserialize, Deserializer, Serializer};

    // Serde requires it to be this way
    #[allow(clippy::ref_option)]
    pub fn serialize<S: Serializer>(
        slice: &Option<Box<[String]>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let mut len = None;

        if let Some(slice) = &slice {
            len = Some(slice.len());
        }

        let mut seq = serializer.serialize_seq(len)?;

        if let Some(slice) = slice {
            for player in slice.as_ref() {
                seq.serialize_element(player)?;
            }
        }

        seq.end()
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Box<[String]>>, D::Error> {
        let slice: Box<[String]> = Box::deserialize(deserializer)?;
        if slice.is_empty() {
            Ok(None)
        } else {
            Ok(Some(slice))
        }
    }
}

pub(crate) mod duration {
    use serde::{Deserialize, Deserializer, Serializer};
    use time::Duration;

    pub fn serialize<S: Serializer>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_f64(duration.as_seconds_f64())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Duration, D::Error> {
        f64::deserialize(deserializer).map(Duration::seconds_f64)
    }
}

#[cfg(test)]
mod tests {
    use crate::in_game::types::Events;

    #[test]
    fn event_deserialize() {
        const JSON: &str = include_str!("events.json");

        let events: Events = serde_json::from_str(JSON).unwrap();

        println!("{events:#?}");

        let json = serde_json::to_string_pretty(&events).unwrap();

        println!("{json:#}");

        // Test that it goes back into the proper format
        let _: Events = serde_json::from_str(&json).unwrap();
    }
}
