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
    /// Checks if a point is an unsafe/deathly point. This is different from a hazardous
    /// point in that a hazardous point may still be crossed, but with some penalty.
    pub fn is_unsafe(&self, point: &Point) -> bool {
        // If the point is out of bounds, it is unsafe.
        // TODO: This should not be a factor IF we are playing on wrapped mode.
        !self.board.is_on_board(point)
            || self // If the point is part of a snake, it is unsafe.
                .board
                .snakes
                .iter()
                .any(|snake| snake.body.contains(point))
    }

    /// Gets the cost of moving to a certain point, assuming that the point you are moving
    /// from is a neighbor of the point you are moving to.
    pub fn cost(&self, point: &Point) -> i32 {
        // If the point is unsafe, it is not a valid move.
        if self.is_unsafe(point) {
            i32::MAX
        } else if self.board.food.contains(point) {
            // If the point has food, it is a valid move, but with a cost of -1.
            -1
        } else {
            // If the point is empty, it is a valid move, but with a cost of 0.
            0
        }
    }
}
