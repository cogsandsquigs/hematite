use super::coord::Coord;
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

impl Board {
    /// Gets all the orthogonal neighbors of a coordinate point on the board
    pub fn ortho_neighbors(&self, point: &Coord) -> Vec<Coord> {
        let mut neighbors = point.ortho_neighbors();
        neighbors.retain(|coord| self.is_on_board(coord));
        neighbors
    }

    /// Checks if a coordinate is on the board.
    pub fn is_on_board(&self, coord: &Coord) -> bool {
        coord.x >= 0 && coord.x < self.width as i32 && coord.y >= 0 && coord.y < self.height as i32
    }
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
