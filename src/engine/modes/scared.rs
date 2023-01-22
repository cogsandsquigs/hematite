use crate::engine::Engine;
use itertools::Itertools;

/// API for using the scared `Mode`.
impl Engine {
    /// Get all the scared moves for the engine. This is the set of moves that gets it as close
    /// as possible to its own tail.
    pub fn scared(&mut self) {
        let head = self.you.head;
        let tail = self.you.tail();

        let moves = self
            .moves
            .into_iter()
            // Get the moves that are closest to the tail
            .min_set_by_key(|(move_, _)| {
                let point = move_.to_coord(&head);
                point.distance(&tail)
            })
            .iter()
            .map(|(move_, _)| *move_)
            .collect::<Vec<_>>();

        self.moves.invalidate_others_many(&moves);
    }
}
