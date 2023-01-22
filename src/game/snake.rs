use super::point::Point;
use rocket::serde::{Deserialize, Serialize};

/// A battlesnake.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Snake {
    pub id: String,
    pub name: String,
    pub health: u32,
    pub body: Vec<Point>,
    pub head: Point,
    pub length: u32,
    pub latency: String,
    pub shout: Option<String>,
}

impl Snake {
    /// Gets the snake's tail.
    pub fn tail(&self) -> Point {
        *self.body.last().expect("All snakes should have a tail.")
    }
}
