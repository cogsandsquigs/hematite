pub mod hungry;
pub mod scared;

use super::Engine;
use log::debug;

/// The different modes the engine can be in.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    // /// The scared mode.
    // Scared,
    /// When the snake is running low on health, it will try to find food.
    Hungry,
}

/// API for updating the `Mode`.
impl Engine {
    /// Update the mode based on the snake's health.
    pub fn update_engine_mode(&mut self) {
        // if self.health() <= self.average_distance_to_food() + 10 {
        //     self.mode = Hungry;
        // } else {
        //     self.mode = Scared;
        // }

        // Log the mode.
        debug!("Mode: {:?}", self.mode)
    }

    /// Get the average distance to the food.
    pub fn average_distance_to_food(&self) -> u32 {
        let head = self.you.head;

        self.board
            .food
            .iter()
            .map(|food| head.distance(food))
            .sum::<u32>()
            / self.board.food.len() as u32
    }
}
