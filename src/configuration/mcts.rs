use serde::{Deserialize, Serialize};

/// Configuration for MCTS
#[derive(Copy, Debug, Clone, Serialize, Deserialize)]
pub struct MCTSConfig {
    /// The number of games to simulate per search iteration.
    #[serde(default = "MCTSConfig::default_games_per_search")]
    pub games_per_search: u32,

    /// The number of iterations to run the MCTS algorithm for.
    #[serde(default = "MCTSConfig::default_max_depth")]
    pub max_depth: u32,
}

impl Default for MCTSConfig {
    fn default() -> Self {
        Self {
            games_per_search: Self::default_games_per_search(),
            max_depth: Self::default_max_depth(),
        }
    }
}

impl MCTSConfig {
    /// Returns the default number of iterations.
    fn default_games_per_search() -> u32 {
        5
    }

    /// Returns the default number of iterations.
    fn default_max_depth() -> u32 {
        100
    }
}
