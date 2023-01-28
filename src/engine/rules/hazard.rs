// This controls how we percieve hazards. These are obstacles/dangers that are not immediately threataning/lethal, but
// are still dangerous to move into.

use crate::{engine::Engine, objects::point::Point};
use itertools::Itertools;

/// Public Engine API for hazards.
impl Engine {
    /// Returns true if the given point is a hazard, but does NOT consider if a point is safe to move to. This is
    /// because hazards are not always dangerous to move into, such as other snakes' moves.
    pub fn is_hazard(&self, point: &Point) -> bool {
        self.hazards().contains(point) || self.is_snake_move(point)
    }

    /// Returns true if the point is a possible move of any other snake than the Engine's, that is unsafe to move
    /// into IF the other snake moves into it. Otherwise, returns false.
    pub fn is_snake_move(&self, point: &Point) -> bool {
        self.other_snakes()
            .flat_map(|snake| {
                snake
                    .head
                    .neighbors()
                    .iter()
                    .map(move |move_point| (*move_point, snake.length))
                    .collect::<Vec<_>>()
            })
            .any(|(move_point, length)| move_point == *point && self.len() <= length)
    }
}
