use crate::{engine::Engine, game::moves::Move};
use std::collections::HashSet;

/// API for using the scared `Mode`.
impl Engine {
    /// Get all the scared moves for the engine. In this case, it is the set of moves that
    /// gets it as far away as possible from other snakes, and as close as possible to food.
    pub fn scared(&self, moves: HashSet<Move>) -> HashSet<Move> {
        let head = &self.you.head;

        let mut min_distance = 0;
        let mut min_moves = HashSet::new();

        for move_ in moves {
            let snake_distance: i32 = self
                .board
                .snakes
                .iter()
                .map(|snake| snake.head.distance(head) as i32)
                .sum();

            let food_distance: i32 = self
                .board
                .food
                .iter()
                .map(|food| food.distance(head) as i32)
                .min()
                .unwrap_or(0);

            match food_distance + snake_distance {
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
