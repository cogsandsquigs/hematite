use crate::{engine::Engine, game::moves::Move};

/// Engine API for scared moves.
impl Engine {
    /// Returns the move the snake should make when it's scared. In this case, it tries to maximize its area control.
    pub fn scared_move(&self) -> Option<Move> {
        let best_move = self.safe_moves().max_by_key(|&move_| {
            let head = &move_.to_coord(&self.you.head);

            self.area_control(head) + self.area_accessible(head)
        });

        best_move
    }

    /// Returns true if the snake is scared.
    pub fn is_scared(&self) -> bool {
        let other_snakes = self.board.other_snakes(&self.you);

        let closest_snake = other_snakes
            .map(|snake| (snake, snake.head.distance(&self.you.head)))
            .min_by_key(|(_, distance)| *distance);

        match closest_snake {
            // There are no snakes on the board, so we should act scared by default.
            None => true,

            // If the closest snake is within 3 spaces, and it's longer than us, we should act scared.
            Some((snake, distance)) => distance <= 3 && snake.body.len() >= self.you.body.len(),
        }
    }
}
