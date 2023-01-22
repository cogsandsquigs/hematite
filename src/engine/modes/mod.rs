pub mod hungry;

use super::Engine;

/// The different modes the snake can be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// The snake is hungry and wants to eat food.
    Hungry,

    /// The snake is using minimax/tree search to find the best move.
    Searching,
}

/// Engine API for modes.
impl Engine {
    /// Update the mode of the engine.
    pub fn update_mode(&mut self) {
        self.mode = if self.is_hungry() {
            Mode::Hungry
        } else {
            Mode::Searching
        };
    }
}
