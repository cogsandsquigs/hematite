use rocket::serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Move {
    #[serde(rename = "up")]
    Up,

    #[serde(rename = "down")]
    Down,

    #[serde(rename = "left")]
    Left,

    #[serde(rename = "right")]
    Right,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Up => write!(f, "up"),
            Move::Down => write!(f, "down"),
            Move::Left => write!(f, "left"),
            Move::Right => write!(f, "right"),
        }
    }
}
