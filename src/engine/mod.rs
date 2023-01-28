mod modes;
mod rules;
mod utils;

use self::modes::Mode;
use crate::{
    configuration::engine::EngineConfig,
    game::{moves::Move, point::Point, snake::Snake, state::GameState},
};

/// The engine for Hematite.
#[derive(Debug, Clone)]
pub struct Engine {
    /// The configuration for the engine.
    config: EngineConfig,

    /// The current state of the game.
    state: GameState,

    /// The current mode of the engine.
    mode: Mode,
}

/// Public API for the engine.
impl Engine {
    /// Create a new engine.
    pub fn new(config: EngineConfig, initial_state: GameState) -> Self {
        Self {
            config,
            state: initial_state,
            mode: Mode::Hungry,
        }
    }

    /// Update the engine with a new game state.
    pub fn update(&mut self, state: GameState) {
        self.state = state;
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

/// Private API for the engine.
impl Engine {
    /// Get the head of the snake.
    fn head(&self) -> &Point {
        &self.state.you.head
    }

    /// Get the length of the snake.
    fn len(&self) -> u32 {
        self.state.you.length
    }

    /// Get the health of the snake.
    fn health(&self) -> u32 {
        self.state.you.health
    }

    /// Get all the food on the board.
    fn food(&self) -> &[Point] {
        &self.state.board.food
    }

    /// Get all the hazards on the board.
    fn hazards(&self) -> &[Point] {
        &self.state.board.hazards
    }

    /// Get the turn number.
    fn turn(&self) -> u32 {
        self.state.turn
    }

    /// Get all the snakes on the board.
    fn snakes(&self) -> &[Snake] {
        &self.state.board.snakes
    }
}
