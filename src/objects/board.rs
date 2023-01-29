use super::{
    point::Point,
    snake::{Snake, SnakeID},
};
use rocket::serde::{Deserialize, Serialize};
use serde::Deserializer;
use std::collections::{HashMap, HashSet};

/// The board where the game is played.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Board {
    /// The height of the board.
    pub height: u32,

    /// The width of the board.
    pub width: u32,

    /// The set of all snakes on the board.
    #[serde(deserialize_with = "Board::parse_snakes")]
    pub snakes: HashMap<SnakeID, Snake>,

    /// The set of all food on the board.
    pub food: HashSet<Point>,

    /// The set of all hazards on the board.
    pub hazards: HashSet<Point>,
}

/// Public API for the board.
impl Board {
    /// Gets all the orthogonal neighbors of a coordinate point on the board
    pub fn neighbors(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = point.neighbors();
        neighbors.retain(|point| self.is_on_board(point));
        neighbors
    }

    /// Checks if a coordinate is on the board.
    pub fn is_on_board(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.width as i32 && point.y >= 0 && point.y < self.height as i32
    }

    /// Gets all the other snakes on the board. Returns an iterator.
    pub fn other_snakes<'a>(&'a self, you: &'a SnakeID) -> impl Iterator<Item = &'a Snake> + 'a {
        self.snakes
            .iter()
            .filter_map(move |(id, snake)| if id != you { Some(snake) } else { None })
    }
}

/// Private API for the board.
impl Board {
    /// Parse the snakes on the board.
    fn parse_snakes<'de, D>(deserializer: D) -> Result<HashMap<SnakeID, Snake>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let snakes: Vec<Snake> = Vec::deserialize(deserializer)?;
        Ok(snakes.into_iter().map(|snake| (snake.id, snake)).collect())
    }
}
