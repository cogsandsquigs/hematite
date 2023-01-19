use super::Engine;
use crate::game::{moves::Move, point::Point};
use std::collections::HashSet;

/// API for getting all the immediately safe/non-lethal moves for the engine's snake.
impl Engine {
    /// Get all the safe moves for the engine
    pub fn engine_safe_moves(&self) -> HashSet<Move> {
        Move::all()
            .into_iter()
            .filter(|m| !self.is_unsafe(&m.to_coord(&self.you.head)))
            .collect()
    }
}

/// API for getting all the immediately safe/non-lethal moves for any snake.
impl Engine {
    /// Checks if a point is an unsafe/deadly point. This is different from a hazardous
    /// point in that a hazardous point may still be crossed, but with some penalty.
    pub fn is_unsafe(&self, point: &Point) -> bool {
        // If the point is out of bounds, it is unsafe.
        // TODO: This should not be a factor IF we are playing on wrapped mode.
        !self.board.is_on_board(point)
            || self.is_snake(point)
            || self.is_potential_snake_move(point)
    }

    /// Checks if a point intersects with a snake's body.
    pub fn is_snake(&self, point: &Point) -> bool {
        self.board
            .snakes
            .iter()
            .any(|snake| snake.body.contains(point))
    }

    /// Checks if a point could be a space a snake moves to. If the point could be
    /// a space a snake moves to, it is unsafe IF the snake is longer than you.
    pub fn is_potential_snake_move(&self, point: &Point) -> bool {
        self.board.snakes.iter().any(|snake| {
            // If the snake is not you,
            snake.id != self.you.id
                && snake // and the snake's head is a neighbor of the point,
                    .head
                    .neighbors()
                    .iter()
                    .any(|neighbor| neighbor == point)
                && snake.body.len() > self.you.body.len() // and if the snake is longer than you, it is unsafe.
        })
    }
}
