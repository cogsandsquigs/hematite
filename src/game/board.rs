use super::{point::Point, snake::Snake};
use rocket::serde::{Deserialize, Serialize};

/// The board where the game is played.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Board {
    pub height: u32,
    pub width: u32,
    pub food: Vec<Point>,
    pub snakes: Vec<Snake>,
    pub hazards: Vec<Point>,
}

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

    /// Gets all the other snakes on the board.
    pub fn other_snakes(&self, you: &Snake) -> Vec<&Snake> {
        self.snakes
            .iter()
            .filter(|snake| snake.id != you.id)
            .collect()
    }
}
