mod immediate_survival;

use crate::{
    board::{Battlesnake, Board, Move},
    game::GameState,
};
use rand::seq::SliceRandom;

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

        // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
        // let food = &board.food;

        chosen
    }
}
