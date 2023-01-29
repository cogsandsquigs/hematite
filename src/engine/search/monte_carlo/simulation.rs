use crate::objects::{board::Board, moves::Move, point::Point, snake::SnakeID, GameState};

/// A state in the game.
#[derive(Debug, Clone)]
pub struct State {
    /// The current board state.
    board: Board,

    /// The current player.
    you: SnakeID,
}

impl State {
    /// Creates a new game state.
    pub fn new(state: GameState) -> Self {
        Self {
            board: state.board,
            you: state.you.id,
        }
    }

    // /// Gets all the current legal moves for the current player.
    // pub fn legal_moves(&self) -> HashSet<Move> {
    //     let mut moves = HashSet::new();

    //     // If we are not in the game (not on the board), we have died
    //     // and cannot make any moves.
    //     if !self.board.snakes.contains_key(&self.you) {
    //         return moves;
    //     }

    //     for move_ in Move::all() {
    //         let point = &move_.to_point(&self.board.snakes[&self.you].head);
    //         if self.board.is_on_board(point) && !self.is_snake(point) {
    //             moves.insert(move_);
    //         }
    //     }

    //     moves
    // }

    // /// Checks if a point intersects unsafely with a snake.
    // fn is_snake(&self, point: &Point) -> bool {
    //     for snake in self.board.snakes.values() {
    //         let length = snake.body.len();
    //         // The head of the snake.
    //         let head = snake.head;
    //         // All of the snake's body except for the tail.
    //         let init = &snake.body[..snake.body.len() - 1];
    //         // The tail of the snake.
    //         let tail = snake.tail();

    //         // If the point is in the snake's body, then it is a snake. Or, if the point is the tail, and the
    //         // snake is longer than 3 tiles long or is moving into a food, then it is also counted as a body
    //         // part.
    //         if init.contains(point)
    //             || tail == *point
    //                 && length >= 3
    //                 && head
    //                     .neighbors()
    //                     .iter()
    //                     .all(|neighbor| !self.board.food.contains(neighbor))
    //         {
    //             return true;
    //         }
    //     }

    //     // If we haven't returned true by now, then the point is not in a snake's body.
    //     false
    // }
}
