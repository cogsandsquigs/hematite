pub mod hungry;
pub mod scared;

use super::Engine;
use log::debug;

/// The different modes the snake can be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// The snake is hungry and wants to eat food.
    Hungry,

    /// The snake is scared and wants to avoid danger.
    Scared,
}

/// Engine API for modes.
impl Engine {
    /// Update the mode of the engine.
    pub fn update_mode(&mut self) {
        self.mode = if self.is_hungry() {
            Mode::Hungry
        } else if self.is_scared() {
            Mode::Scared
        } else {
            panic!("Engine is in an invalid mode.")
        };

        debug!("Engine mode: {:?}", self.mode);
    }
}
