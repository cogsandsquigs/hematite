use serde::{Deserialize, Serialize};

/// The structure holding the configuration for the engine.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct EngineConfig {
    /// How many moves to be hungry for (i.e., the number of initial moves the snake is hungry for).
    #[serde(default = "EngineConfig::default_hungry_moves")]
    pub hungry_moves: u32,
}

impl EngineConfig {
    /// Returns the default number of hungry moves.
    fn default_hungry_moves() -> u32 {
        50
    }
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            hungry_moves: Self::default_hungry_moves(),
        }
    }
}
