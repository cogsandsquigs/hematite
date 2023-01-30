use crate::{engine::Engine, objects::moves::Move};
use itertools::Itertools;
use log::info;

/// Engine API for searching/MCTS moves.
impl Engine {
    /// Returns the best move via MCTS.
    pub fn searching_move(&mut self) -> Option<Move> {
        let scores = self.tree.move_scores();
        let (move_, score) = scores
            .iter()
            // Filter out moves that are not safe.
            .filter(|(move_, _)| self.safe_moves().collect_vec().contains(move_))
            .max_by(|(_, a), (_, b)| a.total_cmp(b))?;

        self.tree.search(1000);

        info!("Number of simulated wins: {}", self.tree.wins());

        info!(
            "Best move: {:?} (chance of winning: {}%)",
            move_,
            score * 100.0
        );

        Some(*move_)
    }
}
