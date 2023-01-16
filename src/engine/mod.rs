use crate::{
    board::{
        Battlesnake, Board,
        Move::{self, *},
    },
    game::GameState,
};
use rand::seq::SliceRandom;
use std::collections::HashSet;

/// The engine for Ferrite.
#[derive(Debug, Clone)]
pub struct Engine {
    /// The ID of the game.
    #[allow(dead_code)]
    game_id: String,

    /// The board where the game is played.
    board: Board,

    /// The Battlesnake that this engine is controlling.
    you: Battlesnake,
}

impl Engine {
    /// Create a new engine.
    pub fn new(game_id: String, initial_state: GameState) -> Self {
        Self {
            game_id,
            board: initial_state.board,
            you: initial_state.you,
        }
    }

    /// Update the engine with a new game state.
    pub fn update(&mut self, state: GameState) {
        self.board = state.board;
        self.you = state.you;
    }

    /// Get the next move for the snake.
    pub fn get_move(&self) -> Move {
        // Are there any safe moves left?
        let safe_moves = self.engine_safe_moves().into_iter().collect::<Vec<_>>();

        // Choose a random move from the safe ones
        let Some(chosen ) = safe_moves.choose(&mut rand::thread_rng()).copied()
        // If there are no safe moves, choose a random move from all the moves
        else {
            return Move::random();
        };

        // TODO: Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
        // let opponents = &board.snakes;

        // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
        // let food = &board.food;

        chosen
    }
}

/// API for getting all the immediately safe/non-lethal moves.
impl Engine {
    /// Get all the safe moves for the engine
    fn engine_safe_moves(&self) -> HashSet<Move> {
        self.snake_safe_moves(&self.you)
    }
}

/// Private API for snake-related operations.
impl Engine {
    /// Gets all the safe moves for `snake`.
    fn snake_safe_moves(&self, snake: &Battlesnake) -> HashSet<Move> {
        let mut moves = Move::all();

        self.snake_non_intersecting_moves(&mut moves, snake);
        self.snake_inside_board_moves(&mut moves, snake);

        moves
    }

    /// Gets all the moves for `snake` that wont intersect other snakes, including itself.
    /// If it could intersect itself, exclude the head from the check.
    fn snake_non_intersecting_moves(&self, moves: &mut HashSet<Move>, snake: &Battlesnake) {
        let head = &snake.head; // Coordinates of your head

        for opponent in self.board.snakes.iter() {
            for (i, coord) in opponent.body.iter().enumerate() {
                // Don't check the head of the snake against itself
                if opponent.id == snake.id && i == 0 {
                    continue;
                }

                for direction in moves.clone() {
                    let next_coord = direction.to_coord(*head);

                    if next_coord == *coord {
                        moves.remove(&direction);
                    }
                }
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
