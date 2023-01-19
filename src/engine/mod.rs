mod longevity;
mod modes;
mod safety;
mod utils;

use self::modes::Mode;
use crate::game::{
    board::{Battlesnake, Board},
    moves::Move,
    GameState,
};
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};

/// The engine for Hematite.
#[derive(Debug, Clone)]
pub struct Engine {
    /// The board where the game is played.
    board: Board,

    /// The Battlesnake that this engine is controlling.
    you: Battlesnake,

    /// The mode the engine is in
    mode: Mode,

    /// The random number generator for the engine.
    rng: SmallRng,
}

impl Engine {
    /// Create a new engine.
    pub fn new(initial_state: GameState) -> Self {
        Self {
            board: initial_state.board,
            you: initial_state.you,
            mode: Mode::Hungry,
            rng: SmallRng::from_entropy(),
        }
    }

    /// Update the engine with a new game state.
    pub fn update(&mut self, state: GameState) {
        self.board = state.board;
        self.you = state.you;
    }

    /// Get the next move for the snake.
    pub fn get_move(&mut self) -> Move {
        // Update the engine mode.
        self.update_engine_mode();

        // Get the set of immediately safe moves
        let safe_moves = self.engine_safe_moves();

        // Get the set of long-term safe moves
        let moves = self.engine_longevity_moves(safe_moves.clone());

        // If there are no moves, choose a random move from all safe moves.
        if moves.is_empty() {
            // If there are no moves, choose a random move from all safe moves
            return if let Some(chosen) = safe_moves
                .into_iter()
                .collect::<Vec<_>>()
                .choose(&mut self.rng)
                .copied()
            {
                chosen
            }
            // If there are no safe moves, choose a random move from all the moves
            else {
                Move::random()
            };
        }

        let moves = match self.mode {
            Mode::Scared => self.scared(moves),
            Mode::Hungry => self.hungry(moves),
        };

        // Choose a random move from the set of moves
        if let Some(chosen) = moves
            .into_iter()
            .collect::<Vec<_>>()
            .choose(&mut self.rng)
            .copied()
        {
            chosen
        }
        // If there are no moves, choose a random move from all safe moves
        else if let Some(chosen) = safe_moves
            .into_iter()
            .collect::<Vec<_>>()
            .choose(&mut self.rng)
            .copied()
        {
            chosen
        }
        // If there are no safe moves, choose a random move from all the moves
        else {
            Move::random()
        }
    }
}
