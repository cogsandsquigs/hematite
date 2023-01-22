// Controlls when the snake decides that it's hungry, and *needs* to eat. Specifically, when the snake
// is hungry, it will move towards the nearest food, even if it means moving into a wall.

use crate::{
    engine::Engine,
    game::{moves::Move, point::Point},
};

/// Engine API for hungry moves.
impl Engine {
    /// Returns the move the snake should make when it's hungry.
    pub fn hungry_move(&self) -> Move {
        // Get all the neighbors which are safe to move into.
        let neighbors: Vec<Point> = self
            .you
            .head
            .neighbors()
            .iter()
            .filter(|&n| self.is_safe(n))
            .copied()
            .collect();

        // Get the nearest food.
        let Some(nearest_food) = self
            .board
            .food
            .iter()
            .min_by_key(|f| f.distance(&self.you.head))
        else {
            // If there is no nearest food, return a random move.
            return self.random_move();
        };

        // Pathfind to the nearest food. If there is no path to the nearest food, return a random move.
        let Some(path) = self.astar_find(&neighbors, nearest_food) else {
            return self.random_move();
        };

        // Return the next move in the path.
        Move::from_coords(&self.you.head, &path[0]).expect("A* paths should generate valid moves.")
    }

    /// Returns true if the snake is hungry. this is when the snake is below 50 health, or it's the
    /// first 50 turns of the game.
    pub fn is_hungry(&self) -> bool {
        self.turn <= self.config.hungry_moves || self.you.health < 50
    }
}
