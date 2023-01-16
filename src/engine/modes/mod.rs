pub mod scared;

use log::debug;
use Mode::*;

use super::Engine;

/// The different modes the engine can be in.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    /// The normal mode.
    Scared,

    /// When the snake is running low on health, it will try to find food.
    Hungry,
}

/// API for updating the `Mode`.
impl Engine {
    /// Update the mode based on the snake's health.
    pub fn update_engine_mode(&mut self) {
        if self.you.health < 50 {
            self.mode = Hungry;
        } else {
            self.mode = Scared;
        }

        // Log the mode.
        debug!("Mode: {:?}", self.mode)
    }
}
