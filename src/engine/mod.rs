mod modes;
mod rules;
mod utils;

use self::modes::Mode;
use crate::{
    configuration::engine::EngineConfig,
    game::{board::Board, moves::Move, snake::Snake, state::GameState},
};

/// The engine for Hematite.
#[derive(Debug, Clone)]
pub struct Engine {
    /// The configuration for the engine.
    config: EngineConfig,

    /// Turn counter
    turn: u32,

    /// The board where the game is played.
    board: Board,

    /// The Snake that this engine is controlling.
    you: Snake,

    /// The mode the engine is in
    mode: Mode,
}

impl Engine {
    /// Create a new engine.
    pub fn new(config: EngineConfig, initial_state: GameState) -> Self {
        Self {
            config,
            board: initial_state.board,
            you: initial_state.you,
            mode: Mode::Hungry,
            turn: 0,
        }
    }

    /// Update the engine with a new game state.
    pub fn update(&mut self, state: GameState) {
        self.board = state.board;
        self.you = state.you;
        self.turn = state.turn;

        // Update the mode of the engine.
        self.update_mode();
    }

    /// Get the next move for the snake. Should always be called before `update`, to
    /// update both the game state as well as the mode the engine is in.
    pub fn get_move(&mut self) -> Move {
        // Get the move the engine makes based on the mode it's in.
        let move_ = match self.mode {
            Mode::Hungry => self.hungry_move(),
            Mode::Defensive => self.defensive_move(),
            Mode::Aggressive => self.aggressive_move(),
        };

        match move_ {
            Some(m) => m,
            None => self.random_move(),
        }
    }
}
