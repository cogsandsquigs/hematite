use crate::{engine::Engine, objects::moves::Move};
use log::{debug, info};

/// Engine API for searching/MCTS moves.
impl Engine {
    /// Returns the best move via MCTS.
    pub fn searching_move(&mut self) -> Option<Move> {
        // Run the MCTS
        self.tree.search();

        debug!(
            "{:.2}% Chance of winning at this point",
            self.tree.win_rate() * 100.0
        );

        // Get the scores of all the moves.
        if let Some((best_move, score)) = self.tree.best_action() {
            info!(
                "Best move: {:?} (chance of winning: {:.2}%)",
                best_move,
                score * 100.0
            );

            Some(best_move)
        } else {
            None
        }
    }
}
