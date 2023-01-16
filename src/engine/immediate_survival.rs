use super::Engine;
use crate::board::{
    Battlesnake,
    Move::{self, *},
};
use std::collections::HashSet;

/// API for getting all the immediately safe/non-lethal moves for the engine's snake.
impl Engine {
    /// Get all the safe moves for the engine
    pub fn engine_safe_moves(&self) -> HashSet<Move> {
        self.snake_safe_moves(&self.you)
    }
}

/// API for getting all the immediately safe/non-lethal moves for any snake.
impl Engine {
    /// Gets all the safe moves for `snake`.
    pub fn snake_safe_moves(&self, snake: &Battlesnake) -> HashSet<Move> {
        let mut moves = Move::all();

        self.snake_non_intersecting_moves(&mut moves, snake);
        self.snake_inside_board_moves(&mut moves, snake);

        moves
    }

    /// Gets all the moves for `snake` that wont intersect other snakes, including itself.
    /// If it could intersect itself, exclude the head from the check.
    fn snake_non_intersecting_moves(&self, moves: &mut HashSet<Move>, snake: &Battlesnake) {
        let head = &snake.head; // Coordinates of your head

        // Flat-map the snakes into a list of all coordinates, and check if the head
        // of the snake is in any of the other snakes' bodies.
        for coord in self.board.snakes.iter().flat_map(|snake| &snake.body) {
            // Don't check the head of the snake against itself. Its ok to just
            // check if the current coordinate is the head, because the head should
            // never be in the body of the snake, unless the game is over, in which
            // case we don't care about the moves.
            if coord == head {
                continue;
            }

            // Convert the two coordinates (`coord` and `head`) into a move, and
            // remove it from the list of possible moves.
            if let Some(move_) = Move::from_coords(coord, head) {
                moves.remove(&move_);
            }
        }
    }

    /// Get all the moves for `snake` that won't go over the board's bounds.
    fn snake_inside_board_moves(&self, moves: &mut HashSet<Move>, snake: &Battlesnake) {
        let head = &snake.body[0]; // Coordinates of your head
        let board_width = &self.board.width;
        let board_height = &self.board.height;

        // Head is at the left edge, don't move left
        if head.x == 0 {
            moves.remove(&Left);
        }
        // Head is at the right edge, don't move right
        if head.x == (board_width - 1) as i32 {
            moves.remove(&Right);
        }
        // Head is at the bottom edge, don't move down
        if head.y == 0 {
            moves.remove(&Down);
        }
        // Head is at the top edge, don't move up
        if head.y == (board_height - 1) as i32 {
            moves.remove(&Up);
        }
    }
}
