pub mod board;
pub mod moves;
pub mod point;

use self::board::{Battlesnake, Board};
use rocket::serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

// API and Response Objects
// See https://docs.battlesnake.com/api

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Game {
    pub id: String,
    pub ruleset: HashMap<String, Value>,
    pub timeout: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameState {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: Battlesnake,
}
