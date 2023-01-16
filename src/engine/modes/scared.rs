use crate::{engine::Engine, game::moves::Move};
use std::collections::HashSet;

/// API for using the scared `Mode`.
impl Engine {
    /// Get all the scared moves for the engine. In this case, it is the set of moves that
    /// gets it as far away as possible to its tail.
    pub fn scared(&self, moves: HashSet<Move>) -> HashSet<Move> {
        let head = &self.you.head;
        let tail = self
            .you
            .body
            .last()
            .expect("The snake should always have a tail!");

        let mut max_distance = 0;
        let mut max_moves = HashSet::new();

        for move_ in moves {
            let coord = move_.to_coord(head);
            let distance = coord.manhattan_distance(tail);

            match distance {
                distance if distance > max_distance => {
                    max_distance = distance;
                    max_moves.clear();
                    max_moves.insert(move_);
                }
                distance if distance == max_distance => {
                    max_moves.insert(move_);
                }
                _ => {}
            }
        }

        max_moves
    }
}
