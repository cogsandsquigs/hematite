mod modes;
mod monte_carlo;
mod rules;
mod utils;

use self::{modes::Mode, monte_carlo::MonteCarlo};
use crate::{
    configuration::engine::EngineConfig,
    objects::{moves::Move, point::Point, snake::Snake, GameState},
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

    /// The current monte-carlo search tree.
    tree: MonteCarlo,
}

/// Public API for the engine.
impl Engine {
    /// Create a new engine.
    pub fn new(config: EngineConfig, initial_state: GameState) -> Self {
        Self {
            config,
            state: initial_state.clone(),
            mode: Mode::Hungry,
            tree: MonteCarlo::new(initial_state, config.mcts),
        }
    }

    /// Update the engine with a new game state.
    pub fn update(&mut self, state: GameState) {
        self.state = state.clone();
        // Update the mode of the engine.
        self.update_mode();
        // Update the monte-carlo search tree.
        self.tree.update(state);
    }

    /// Get the next move for the snake. Should always be called before `update`, to
    /// update both the game state as well as the mode the engine is in.
    pub fn get_move(&mut self) -> Move {
        // Get the move the engine makes based on the mode it's in.
        let move_ = match self.mode {
            Mode::Hungry => self.searching_move(), //self.hungry_move(),
            Mode::Searching => self.searching_move(),
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
    fn health(&self) -> i32 {
        self.state.you.health
    }

    /// Get the turn number.
    fn turn(&self) -> u32 {
        self.state.turn
    }

    /// Get all the food on the board. Returns an iterator over the food.
    fn food(&self) -> impl Iterator<Item = &Point> + '_ {
        self.state.board.food.iter()
    }

    /// Get all the hazards on the board.
    fn hazards(&self) -> impl Iterator<Item = &Point> + '_ {
        self.state.board.hazards.iter()
    }

    /// Get all the snakes on the board. Returns an iterator over the snakes.
    fn snakes(&self) -> impl Iterator<Item = &Snake> + '_ {
        self.state.board.snakes.values()
    }
}
