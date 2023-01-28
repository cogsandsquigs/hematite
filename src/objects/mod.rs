pub mod board;
pub mod moves;
pub mod point;
pub mod settings;
pub mod snake;

use self::{board::Board, settings::Ruleset, snake::Snake};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// API and Response Objects
// See https://docs.battlesnake.com/api

/// The object holding the game and it's settings.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Game {
    pub id: Uuid,
    pub ruleset: Ruleset,
    pub timeout: u32,
}

/// The state of the game.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameState {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: Snake,
}
