mod modes;
mod rules;
mod utils;

use self::modes::Mode;
use crate::{
    configuration::engine::EngineConfig,
    game::{board::Board, moves::Move, snake::Snake, GameState},
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
    }

    /// Get the next move for the snake.
    pub fn get_move(&mut self) -> Move {
        // Update the mode of the engine.
        self.update_mode();

        self.hungry_move()
    }
}
