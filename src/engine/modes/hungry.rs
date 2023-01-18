use crate::{engine::Engine, game::moves::Move};
use std::collections::HashSet;

/// API for using the hungry `Mode`.
impl Engine {
    /// Get all the hungry moves for the engine. In this case, it is the set of moves that
    /// gets it as close as possible to food.
    pub fn hungry(&self, moves: HashSet<Move>) -> HashSet<Move> {
        let head = &self.you.head;
        let food = self
            .board
            .food
            .first()
            .expect("There should always be food!");

        let mut min_distance = u32::MAX;
        let mut min_moves = HashSet::new();

        for move_ in moves {
            let point = move_.to_coord(head);
            let distance = point.distance(food);

            match distance {
                distance if distance < min_distance => {
                    min_distance = distance;
                    min_moves.clear();
                    min_moves.insert(move_);
                }
                distance if distance == min_distance => {
                    min_moves.insert(move_);
                }
                _ => {}
            }
        }

        min_moves
    }
}
