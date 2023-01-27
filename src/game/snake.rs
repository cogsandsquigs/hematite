use super::point::Point;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A battlesnake.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Snake {
    pub id: SnakeID,
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

/// An ID for a snake.
#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(transparent)]
#[repr(transparent)]
pub struct SnakeID(Uuid);
