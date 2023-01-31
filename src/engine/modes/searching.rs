use crate::{engine::Engine, objects::moves::Move};
use log::{debug, info};

/// Engine API for searching/MCTS moves.
impl Engine {
    /// Returns the best move via MCTS.
    pub fn searching_move(&mut self) -> Option<Move> {
        // Run the MCTS
        self.tree.search(200);

        debug!(
            "{:.2}% Chance of winning at this point",
            (self.tree.wins() as f64 / self.tree.visits() as f64) * 100.0
        );

        // Get the scores of all the moves.
        let scores = self.tree.move_scores();
        let (move_, score) = scores.iter().max_by(|(_, a), (_, b)| a.total_cmp(b))?;

        info!(
            "Best move: {:?} (chance of winning: {:.2}%)",
            move_,
            score * 100.0
        );

        Some(*move_)
    }
}
