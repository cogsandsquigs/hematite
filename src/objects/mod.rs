pub mod board;
pub mod moves;
pub mod point;
pub mod snake;
pub mod state;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

// API and Response Objects
// See https://docs.battlesnake.com/api

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Game {
    pub id: Uuid,
    pub ruleset: HashMap<String, Value>,
    pub timeout: u32,
}
