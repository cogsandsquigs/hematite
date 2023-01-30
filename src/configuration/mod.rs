pub mod engine;
pub mod mcts;
pub mod snake;

use self::{engine::EngineConfig, snake::SnakeConfig};
use cargo_toml::Manifest;
use serde::{Deserialize, Serialize};

/// The structure holding the configuration for the battlesnake itself and the engine.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    /// My battlesnake username.
    pub battlesnake_username: String,

    /// The configuration for the battlesnake itself.
    pub snake: SnakeConfig,

    /// The configuration for the engine.
    #[serde(default)]
    pub engine: EngineConfig,
}

impl Config {
    /// Loads the configuration from `Cargo.toml`. Panics if either the `package` or
    /// `package.metadata` keys are missing from the configuration.
    pub fn load() -> Self {
        // The cargo.toml file is included as a byte array at compile time.
        let cargo_file = include_bytes!("../../Cargo.toml");

        // This is the complete structure of the cargo.toml file.
        let manifest = toml::from_slice::<Manifest<Config>>(cargo_file).unwrap();

        manifest
            .package
            .expect("The 'package' key should exist")
            .metadata
            .expect("The 'package.metadata' key should exist")
    }
}
