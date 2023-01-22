use cargo_toml::Manifest;
use serde::{Deserialize, Serialize};

/// The structure holding the configuration for the battlesnake itself and the engine.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    /// My battlesnake username.
    pub battlesnake_username: String,

    /// The configuration for the battlesnake itself.
    pub snake: SnakeConfig,
}

/// The structure holding the configuration for the battlesnake itself.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SnakeConfig {
    /// The color of the snake.
    pub color: String,

    /// OPTIONAL: The debug/development color of the snake.
    pub debug_color: Option<String>,

    /// The head of the snake.
    pub head: String,

    /// The tail of the snake.
    pub tail: String,
}

impl Config {
    /// Loads the configuration from `Cargo.toml`. Panics if either the `package` or
    /// `package.metadata` keys are missing from the configuration.
    pub fn load() -> Self {
        // The cargo.toml file is included as a byte array at compile time.
        let cargo_file = include_bytes!("../Cargo.toml");

        // This is the complete structure of the cargo.toml file.
        let manifest = toml::from_slice::<Manifest<Config>>(cargo_file).unwrap();

        manifest
            .package
            .expect("The 'package' key should exist")
            .metadata
            .expect("The 'package.metadata' key should exist")
    }
}
