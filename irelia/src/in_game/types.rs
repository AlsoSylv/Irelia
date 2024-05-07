//! This module is full of auto generated JSON for in game events
//!
//! if anything fails to serialize this module probably needs to
//! be updated to a newer version of the in-game API.
//!
//! Well the types and returned values do not match, the format will serialize
//! to the same value

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use time::Duration;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    passive: AbilityInfo,
    q: Ability,
    w: Ability,
    e: Ability,
    r: Ability,
}

impl Abilities {
    #[must_use]
    pub fn passive(&self) -> &AbilityInfo {
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
pub struct AbilityInfo {
    display_name: String,
    id: String,
    raw_description: String,
    raw_display_name: String,
}

impl AbilityInfo {
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
    #[serde(flatten)]
    ability_info: AbilityInfo
}

impl Ability {
    #[must_use]
    pub fn ability_level(&self) -> i64 {
        self.ability_level
    }
    #[must_use]
    pub fn ability_info(&self) -> &AbilityInfo {
        &self.ability_info
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    omnivamp: f64,
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
    pub fn resource_type(&self) -> &AbilityResource {
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
    #[must_use]
    pub fn heal_shield_power(&self) -> f64 {
        self.heal_shield_power
    }
    #[must_use]
    pub fn omnivamp(&self) -> f64 {
        self.omnivamp
    }
    #[must_use]
    pub fn physical_vamp(&self) -> f64 {
        self.physical_vamp
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullRunes {
    keystone: Rune,
    primary_rune_tree: Rune,
    secondary_rune_tree: Rune,
    general_runes: [Rune; 6],
    stat_runes: [StatRune; 3],
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
    pub fn general_runes(&self) -> &[Rune; 6] {
        &self.general_runes
    }
    #[must_use]
    pub fn stat_runes(&self) -> &[StatRune; 3] {
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllPlayer {
    champion_name: String,
    is_bot: bool,
    is_dead: bool,
    items: Box<[Item]>,
    level: i64,
    position: Position,
    raw_champion_name: String,
    #[serde(with = "duration")]
    respawn_timer: Duration,
    runes: Runes,
    scores: Scores,
    #[serde(rename = "skinID")]
    skin_id: i64,
    summoner_name: String,
    summoner_spells: SummonerSpells,
    team: TeamID,
    skin_name: Option<String>,
    raw_skin_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    Unknown(String),
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
    pub fn position(&self) -> &Position {
        &self.position
    }
    #[must_use]
    pub fn raw_champion_name(&self) -> &str {
        &self.raw_champion_name
    }
    #[must_use]
    pub fn respawn_timer(&self) -> Duration {
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
    pub fn team(&self) -> &TeamID {
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

    // RustRover is falsely saying that this does not get used
    //noinspection RsLiveness
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
    display_name: String,
    raw_description: String,
    raw_display_name: String,
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
        acer: String,
        acing_team: TeamID,
    },
    ChampionKill {
        #[serde(flatten)]
        kill_info: KillInfo,
        victim_name: String,
    },
    FirstBlood {
        recipient: String,
    },
    #[serde(rename = "Multikill")]
    MultiKill {
        kill_streak: u16,
        killer_name: String,
    },
    TurretKilled {
        #[serde(flatten)]
        kill_info: KillInfo,
        turret_killed: Structure,
    },
    FirstBrick {
        killer_name: String,
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
        result: String,
    },
    #[serde(untagged)]
    Unknown(serde_json::Value),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DragonType {
    Fire,
    Earth,
    Water,
    Air,
    Hextech,
    Chemtech,
    Elder,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Structure {
    structure_type: StructureType,
    /// Blue side = Order
    /// Red side = Chaos,
    team_id: TeamID,
    /// This still exists in aram, but it's useless
    lane: Lane,
    /// This gets tricky, on summoners rift, this is 1-3 for top
    /// and bot lane, and 1-5 for mid, going in reverse. So the outer turret is
    /// 3 or 5, and the inner is 1 or 2.
    ///
    /// On Nexus blitz, it's 4-1, with 4 and 1 being the top outer and inner
    /// while 3 and 2 are the bottom outer and inner turrets
    ///
    /// However, on Aram, on aram, the red side is 1-4 where 1 is the outermost turret
    /// And blue side being 8-10, with 8 as the outermost, 7 as the inner, while 9 and 10
    /// are the innermost nexus turrets!
    place: u8,
}

impl Structure {
    #[must_use]
    /// Either Turret or Barracks (aka inhibitor)
    pub fn structure_type(&self) -> &StructureType {
        &self.structure_type
    }

    #[must_use]
    /// This still exists in aram, but it's useless
    pub fn lane(&self) -> &Lane {
        &self.lane
    }

    #[must_use]
    /// Blue side = Order
    /// Red side = Chaos
    pub fn team_id(&self) -> &TeamID {
        &self.team_id
    }

    #[must_use]
    /// This gets tricky, on summoners rift, this is `1..3` for top
    /// and bot lane, and `1..5` for mid, going in reverse. So the outer turret is
    /// 3 or 5, and the inner is 1 or 2.
    ///
    /// On Nexus blitz, it's `4..1`, with 4 and 1 being the top outer and inner
    /// while 3 and 2 are the bottom outer and inner turrets
    ///
    /// However, on Aram, on aram, the red side is `1..4` where 1 is the outermost turret
    /// And blue side being `8..10`, with 8 as the outermost, 7 as the inner, while 9 and 10
    /// are the innermost nexus turrets!
    ///
    /// See `StructureNames.png` for a diagram
    pub fn place(&self) -> u8 {
        self.place
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

        let data: &str = Deserialize::deserialize(deserializer)?;

        let split: Box<[&str]> = data.split('_').collect();

        let structure = match split.as_ref() {
            // The last value here is always A
            &["Turret", team, lane, number, _] => Structure {
                structure_type: StructureType::Turret,
                team_id: determine_structure_team(team),
                place: number.parse().unwrap(),
                lane: determine_structure_lane(lane.as_bytes()[0]),
            },
            &["Barracks", team, lane] => Structure {
                structure_type: StructureType::Barracks,
                team_id: determine_structure_team(team),
                // This is always 1, as all lanes have one inhib
                place: 1,
                lane: determine_structure_lane(lane.as_bytes()[0]),
            },
            todo => unreachable!("A new type of structure must have been added, please open an issue with the following info: {:?}", todo),
        };

        Ok(structure)
    }
}

impl Serialize for Structure {
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

#[derive(Debug, Clone, PartialEq)]
pub enum StructureType {
    Turret,
    Barracks,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lane {
    Top,
    Mid,
    Bot,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MonsterKill {
    #[serde(flatten)]
    kill_info: KillInfo,
    #[serde(with = "string_to_bool")]
    stolen: bool,
}

impl MonsterKill {
    #[must_use]
    pub fn kill_info(&self) -> &KillInfo {
        &self.kill_info
    }
    #[must_use]
    pub fn stolen(&self) -> bool {
        self.stolen
    }
}

impl Event {
    #[must_use]
    pub fn event_id(&self) -> i64 {
        self.event_id
    }
    #[must_use]
    pub fn event_time(&self) -> Duration {
        self.event_time
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
    game_mode: GameMode,
    #[serde(with = "duration")]
    game_time: Duration,
    map_name: MapName,
    map_number: i64,
    map_terrain: MapTerrain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum GameMode {
    #[serde(rename = "CLASSIC")]
    SummonersRift,
    #[serde(rename = "ODIN")]
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
    /// If this variant pops up, see the riot docs at <https://static.developer.riotgames.com/docs/lol/gameModes.json>
    /// However, this may not be up-to-date
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
    /// If this variant pops up, see the riot docs at <https://static.developer.riotgames.com/docs/lol/maps.json>
    /// However, this may be out of date, if that's the case, look at <https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/maps.json>
    /// for the latest maps that patch
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
    pub fn game_mode(&self) -> &GameMode {
        &self.game_mode
    }
    #[must_use]
    pub fn game_time(&self) -> Duration {
        self.game_time
    }
    #[must_use]
    pub fn map_name(&self) -> &MapName {
        &self.map_name
    }
    #[must_use]
    pub fn map_number(&self) -> i64 {
        self.map_number
    }
    #[must_use]
    pub fn map_terrain(&self) -> &MapTerrain {
        &self.map_terrain
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
/// Ability Resource
///
/// This defaults to Mana
pub enum AbilityResource {
    #[default]
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
        let stolen: &str = Deserialize::deserialize(deserializer)?;

        Ok(match stolen {
            "False" => false,
            "True" => true,
            _ => unreachable!()
        })
    }

    // This has to be passed by ref to work with serde
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S: Serializer>(stolen: &bool, serializer: S) -> Result<S::Ok, S::Error> {
        let value = if *stolen {
            "True"
        } else {
            "False"
        };
        serializer.serialize_str(value)
    }
}

mod option_slice {
    use serde::ser::SerializeSeq;
    use serde::{Deserialize, Deserializer, Serializer};

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
            for player in slice.iter() {
                seq.serialize_element(player)?;
            }
        }

        seq.end()
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Box<[String]>>, D::Error> {
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

    pub(crate) fn serialize<S: Serializer>(
        duration: &Duration,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_f64(duration.as_seconds_f64())
    }

    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Duration, D::Error> {
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
