use super::Engine;
use crate::game::point::Point;

/// API for getting all the immediately safe/non-lethal moves for the engine's snake.
impl Engine {
    /// Get all the safe moves for the engine
    pub fn safe_moves(&mut self) {
        let head = self.you.head;

        self.moves.into_iter().for_each(|(move_, _)| {
            if self.is_unsafe(&move_.to_coord(&head)) {
                self.moves.invalidate(&move_);
            }
        });
    }

    /// Checks if a point is an unsafe/deadly point. This is different from a hazardous
    /// point in that a hazardous point may still be crossed, but with some penalty.
    pub fn is_unsafe(&self, point: &Point) -> bool {
        // If the point is out of bounds, it is unsafe.
        // TODO: This should not be a factor IF we are playing on wrapped mode.
        !self.board.is_on_board(point) || self.is_snake(point)
    }

    /// Checks if a point intersects with a snake's body. Excludes the snake's tail, as
    /// if the snake moves, the tail moves out of the way.
    /// TODO: Check if the snake's head will eat a food, which means that it is UNSAFE
    /// to move to the tail.
    pub fn is_snake(&self, point: &Point) -> bool {
        self.board
            .snakes
            .iter()
            .any(|snake| snake.body[..snake.body.len() - 1].contains(point))
    }
}
