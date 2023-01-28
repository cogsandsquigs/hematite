use crate::{engine::Engine, game::moves::Move};

/// Engine API for defensive moves.
impl Engine {
    /// Returns the move the snake should make when it's defensive. In this case, it tries to maximize its area control.
    pub fn defensive_move(&self) -> Option<Move> {
        let best_move = self.safe_moves().max_by_key(|&move_| {
            let head = &move_.to_coord(self.head());

            self.area_control(head) + self.area_accessible(head)
        });

        best_move
    }
}
