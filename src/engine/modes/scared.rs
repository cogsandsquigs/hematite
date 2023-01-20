use itertools::Itertools;

use crate::{engine::Engine, game::moves::Move};
use std::collections::HashSet;

/// API for using the scared `Mode`.
impl Engine {
    /// Get all the scared moves for the engine. This is the set of moves that gets it as close
    /// as possible to its own tail.
    pub fn scared(&self, moves: HashSet<Move>) -> HashSet<Move> {
        let head = self.you.head;
        let tail = self.you.tail();

        moves
            .iter()
            .min_set_by_key(|move_| {
                let point = move_.to_coord(&head);
                point.distance(&tail)
            })
            .iter()
            .copied()
            .copied()
            .collect()
    }
}
