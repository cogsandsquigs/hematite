pub mod hungry;

use super::Engine;
use log::debug;

/// The different modes the snake can be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// The snake is hungry and wants to eat food.
    Hungry,
    // /// The snake is searching for a good move.
    // Searching,
}

/// Engine API for modes.
impl Engine {
    /// Update the mode of the engine.
    pub fn update_mode(&mut self) {
        // self.mode = if self.is_hungry() {
        //     Mode::Hungry
        // } else {
        //     Mode::Searching
        // };

        self.mode = Mode::Hungry;

        debug!("Engine mode: {:?}", self.mode);
    }
}
