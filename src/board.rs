use rocket::serde::{Deserialize, Serialize};

/// The board where the game is played.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Board {
    pub height: u32,
    pub width: u32,
    pub food: Vec<Coord>,
    pub snakes: Vec<Battlesnake>,
    pub hazards: Vec<Coord>,
}

/// A battlesnake.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Battlesnake {
    pub id: String,
    pub name: String,
    pub health: u32,
    pub body: Vec<Coord>,
    pub head: Coord,
    pub length: u32,
    pub latency: String,
    pub shout: Option<String>,
}

/// A coordinate on the board.
#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

/// A move that a snake can make.
#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}
