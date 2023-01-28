use serde::{Deserialize, Deserializer, Serialize};

/// The rules associated with the current game.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Ruleset {
    /// The name of the game type.
    pub name: GameType,

    /// The settings for the game.
    pub settings: RulesetSettings,
}

/// The settings for the current game.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RulesetSettings {
    /// The percent chance food has to spawn every round
    #[serde(deserialize_with = "RulesetSettings::parse_food_spawn_chance")]
    pub food_spawn_chance: f64,

    /// The minimum food to keep on the board at all times
    pub minimum_food: u32,

    /// The amount of damage hazards do per turn
    pub hazard_damage_per_turn: u32,

    /// The settings for royale games.
    pub royale: RulesetRoyaleSettings,

    /// The settings for squad games.
    pub squad: RulesetSquadSettings,
}

impl RulesetSettings {
    /// Parse the food spawn chance as a float.
    fn parse_food_spawn_chance<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(f64::deserialize(deserializer)? / 100.0)
    }
}

/// The settings for royale games.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RulesetRoyaleSettings {
    /// The number of turns to wait before shrinking the map.
    pub shrink_every_n_turns: u32,
}

/// The settings for squad games.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RulesetSquadSettings {
    /// If we allow snakes on the same squad to collide.
    pub allow_body_collisions: bool,

    /// If one snake on a squad dies, does the whole squad die?
    pub shared_elimination: bool,

    /// Do squad members share health?
    pub shared_health: bool,

    /// Do squad members share length?
    pub shared_length: bool,
}

/// The name of the game type.
/// - standard
/// - royale
/// - constrictor
/// - wrapped
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum GameType {
    #[serde(rename = "standard")]
    Standard,

    #[serde(rename = "royale")]
    Royale,

    #[serde(rename = "constrictor")]
    Constrictor,

    #[serde(rename = "wrapped")]
    Wrapped,
}
