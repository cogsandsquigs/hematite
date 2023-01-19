pub mod astar;
pub mod floodfill;
pub mod heap_item;

use super::Engine;
use crate::game::point::Point;

/// General utilities for the Hematite engine.
impl Engine {
    /// Get's the engine's health.
    pub fn health(&self) -> u32 {
        self.you.health
    }
}

/// General `Point` utilities for the Hematite engine.
impl Engine {
    /// Gets the cost of moving to a certain point, assuming that the point you are moving
    /// from is a neighbor of the point you are moving to.
    pub fn cost(&self, point: &Point) -> i32 {
        if self.board.food.contains(point) {
            // If the point has food, it is a valid move, but with a cost of -1.
            -1
        } else {
            // If the point is empty, it is a valid move, but with a cost of 0.
            0
        }
    }
}
