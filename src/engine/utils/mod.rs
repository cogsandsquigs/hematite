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
        let mut cost = 1;

        if self.board.food.contains(point) {
            // If the point is a food, then it is a good move, so it is rewarded accordingly.
            cost -= 5;
        }

        if self
            .board
            .snakes
            .iter()
            .flat_map(|s| s.head.neighbors())
            .any(|p| &p == point)
        {
            // If the point crosses right in front of a snake's head, it is a valid, but risky, move, so it
            // is punished accordingly.
            cost += 10;
        }

        cost
    }
}
