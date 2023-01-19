use crate::{engine::Engine, game::moves::Move};
use std::collections::HashSet;

/// API for using the hungry `Mode`.
impl Engine {
    /// Get all the hungry moves for the engine. In this case, it is the set of moves that
    /// gets it as close as possible to food.
    pub fn hungry(&self, moves: HashSet<Move>) -> HashSet<Move> {
        let head = self.you.head;

        let closest_food = self
            .board
            .food
            .iter()
            .min_by_key(|food| food.distance(&head));

        if let Some(closest_food) = closest_food {
            let path = self.astar(vec![head], *closest_food);

            if let Some(path) = path {
                let next = path[1];
                let move_to_next = Move::from_coords(&head, &next).expect("Move should exist");
                return moves.into_iter().filter(|m| m == &move_to_next).collect();
            }
        }
        // If we haven't returned yet, then we can't find a path to food, so we should act scared.
        // TODO: Act scared
        moves
    }
}
