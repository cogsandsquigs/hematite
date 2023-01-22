use serde::{Deserialize, Serialize};

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
