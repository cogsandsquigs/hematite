use itertools::Itertools;

use crate::{engine::Engine, objects::moves::Move};

/// Engine API for aggressive moves.
impl Engine {
    /// Returns the move the snake should make when it's aggressive. In this case, it should attack
    /// smaller snakes.
    pub fn aggressive_move(&self) -> Option<Move> {
        // Gets all the heads of snakes that are smaller than us.
        let heads = self
            .other_snakes()
            .filter(|snake| snake.length < self.len())
            .map(|snake| snake.head)
            .collect_vec();

        // If there are no smaller snakes, we can't attack. This only happens if all snakes died,
        // so we return None.
        if heads.is_empty() {
            None
        } else {
            let path = self.astar_find(self.head(), &heads)?;

            Move::from_coords(&path[0], &path[1])
        }
    }

    /// Tells the engine if the snake should act aggressive.
    pub fn is_aggressive(&self) -> bool {
        // The number of snakes smaller than us.
        let smaller_snakes = self
            .other_snakes()
            .filter(|snake| snake.length < self.len())
            .count();

        // If there are no smaller snakes, we can't attack. This only happens if all snakes died,
        // so we return false.
        if smaller_snakes == 0 {
            return false;
        }

        // If there are two or more snakes smaller than us, we should attack.
        smaller_snakes >= 2
    }
}
