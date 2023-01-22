pub mod astar;

use super::Engine;
use crate::game::moves::Move;
use log::{info, warn};
use rand::seq::IteratorRandom;

/// Miscellaneous utility functions for the engine.
impl Engine {
    /// Returns a random move. It will try to make the move safe, but if there is no safe move available,
    /// it will return a random move regardless of safety.
    pub fn random_move(&self) -> Move {
        info!("Choosing a random safe move.");

        let safe_move = self
            .you
            .head
            .neighbors()
            .iter()
            .filter(|&n| self.is_safe(n))
            .choose(&mut rand::thread_rng())
            .copied();

        if let Some(safe_move) = safe_move {
            Move::from_coords(&self.you.head, &safe_move)
                .expect("A* paths should generate valid moves.")
        } else {
            warn!("There are no safe moves available. Returning a random move.");

            Move::all()
                .iter()
                .choose(&mut rand::thread_rng())
                .copied()
                .expect("There should always be a move available.")
        }
    }
}
