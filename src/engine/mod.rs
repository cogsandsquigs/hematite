mod longevity;
mod safety;

use crate::game::{
    board::{Battlesnake, Board},
    moves::Move,
    GameState,
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
        // Get the set of immediately safe moves
        let safe_moves = self.engine_safe_moves();

        // Get the set of long-term safe moves
        let moves = self.engine_longevity_moves(safe_moves.clone());

        // Choose a random move from the set of moves
        if let Some(chosen) = moves
            .into_iter()
            .collect::<Vec<_>>()
            .choose(&mut rand::thread_rng())
            .copied()
        {
            chosen
        }
        // If there are no moves, choose a random move from all safe moves
        else if let Some(chosen) = safe_moves
            .into_iter()
            .collect::<Vec<_>>()
            .choose(&mut rand::thread_rng())
            .copied()
        {
            chosen
        }
        // If there are no safe moves, choose a random move from all the moves
        else {
            println!("No safe moves! Choosing a random move.");
            Move::random()
        }
    }
}
