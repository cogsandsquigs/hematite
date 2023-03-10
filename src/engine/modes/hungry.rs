// Controlls when the snake decides that it's hungry, and *needs* to eat. Specifically, when the snake
// is hungry, it will move towards the nearest food, even if it means moving into a wall.

use crate::{engine::Engine, objects::moves::Move};
use itertools::Itertools;

/// Engine API for hungry moves.
impl Engine {
    /// Returns the move the snake should make when it's hungry.
    pub fn hungry_move(&self) -> Option<Move> {
        // Get the nearest foods.
        let nearest_foods = self
            .food()
            .copied()
            .min_set_by_key(|f| f.distance(self.head()));

        // Pathfind to the nearest food. If there is no path to the nearest food, return None.
        let path = self.astar_find(self.head(), &nearest_foods)?;

        // Return the next move in the path. `path[0]` is the head of the snake, and `path[1]` is the
        // next move.
        Some(Move::from_points(&path[0], &path[1]).expect("A* paths should generate valid moves."))
    }

    /// Returns true if the snake is hungry. this is when the snake is below 50 health, or it's the
    /// first 50 turns of the game.
    pub fn is_hungry(&self) -> bool {
        self.turn() <= self.config.hungry_moves || self.health() < 50
    }
}
