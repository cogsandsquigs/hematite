pub mod area;
pub mod astar;

use super::Engine;
use crate::objects::{moves::Move, point::Point};
use log::{info, warn};
use rand::seq::IteratorRandom;

/// Miscellaneous utility functions for the engine.
impl Engine {
    /// Get all the other snakes in the game.
    pub fn other_snakes(&self) -> impl Iterator<Item = &crate::objects::snake::Snake> {
        self.state.board.other_snakes(&self.state.you)
    }

    /// Checks if a point is on the board
    pub fn is_on_board(&self, point: &Point) -> bool {
        self.state.board.is_on_board(point)
    }

    /// Returns an iterator of all safe moves.
    pub fn safe_moves(&self) -> impl Iterator<Item = Move> + '_ {
        let head = self.head();

        Move::all()
            .iter()
            .copied()
            .filter(move |&move_| !self.is_unsafe(&move_.to_coord(head)))
            .collect::<Vec<_>>()
            .into_iter()
    }

    /// Returns a random move. It will try to make the move safe, but if there is no safe move available,
    /// it will return a random move regardless of safety. This function automatically logs the fact that
    /// it is choosing a random move, so you don't need to do so yourself.
    pub fn random_move(&self) -> Move {
        info!("Choosing a random safe move.");

        let safe_move = self
            .head()
            .neighbors()
            .iter()
            .filter(|&n| !self.is_unsafe(n))
            .choose(&mut rand::thread_rng())
            .copied();

        if let Some(safe_move) = safe_move {
            Move::from_coords(self.head(), &safe_move)
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
