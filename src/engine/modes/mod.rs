pub mod aggressive;
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

    /// The snake is agressive and attacks smaller snakes.
    Aggressive,
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
            Mode::Aggressive
        };

        debug!("Engine mode: {:?}", self.mode);
    }
}
