// This contains the main rules logic for the game. Specifically, we have a function that returns if any given point
// on the board is safe to move to, assuming the snake's head is in a neighboring position.

use crate::{engine::Engine, game::point::Point};

/// Engine API for move safety
impl Engine {
    /// Returns an iterator of all the safe neighbors of the snake's head.
    pub fn safe_neighbors(&self, point: &Point) -> impl Iterator<Item = Point> + '_ {
        point
            .neighbors()
            .into_iter()
            .filter(move |&n| !self.is_unsafe(&n))
    }

    /// Returns true if the given point is unsafe to move to. Otherwise, returns false.
    pub fn is_unsafe(&self, point: &Point) -> bool {
        !self.board.is_on_board(point) || self.is_snake(point) || self.is_snake_move(point)
    }

    /// Returns true if a given point is in a snake's body. Otherwise, returns false. This skips over the snake's
    /// tail, because that is allowed to be moved into as the other snake will move out of it. However, if the snake
    /// is less than 3 tiles long, or is moving into a food, then the tail is not allowed to be moved into.
    pub fn is_snake(&self, point: &Point) -> bool {
        for snake in &self.board.snakes {
            let length = snake.body.len();
            // The head of the snake.
            let head = snake.head;
            // All of the snake's body except for the tail.
            let init = &snake.body[..snake.body.len() - 1];
            // The tail of the snake.
            let tail = snake.tail();

            // If the point is in the snake's body, then it is a snake. Or, if the point is the tail, and the
            // snake is longer than 3 tiles long or is moving into a food, then it is also counted as a body
            // part.
            if init.contains(point)
                || tail == *point
                    && length >= 3
                    && head
                        .neighbors()
                        .iter()
                        .all(|neighbor| !self.board.food.contains(neighbor))
            {
                return true;
            }
        }

        // If we haven't returned true by now, then the point is not in a snake's body.
        false
    }
}
