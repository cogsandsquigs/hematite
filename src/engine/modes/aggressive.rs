use itertools::Itertools;

use crate::{engine::Engine, game::moves::Move};

/// Engine API for aggressive moves.
impl Engine {
    /// Returns the move the snake should make when it's aggressive. In this case, it should attack
    /// smaller snakes.
    pub fn aggressive_move(&self) -> Option<Move> {
        // Gets all the heads of snakes that are smaller than us.
        let heads = self
            .board
            .other_snakes(&self.you)
            .filter(|snake| snake.length < self.you.length)
            .map(|snake| snake.head)
            .collect_vec();

        // If there are no smaller snakes, we can't attack.
        if heads.is_empty() {
            None
        } else {
            let path = self.astar_find(&self.you.head, &heads)?;

            Move::from_coords(&path[0], &path[1])
        }
    }
}
