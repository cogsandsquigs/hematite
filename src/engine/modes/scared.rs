use crate::{engine::Engine, game::moves::Move};

/// Engine API for scared moves.
impl Engine {
    /// Returns the move the snake should make when it's scared. In this case, it's the move that
    /// gets it closest to its body.
    pub fn scared_move(&self) -> Option<Move> {
        let closest_move = self.safe_moves().min_by_key(|&move_| {
            let move_coord = move_.to_coord(&self.you.head);

            self.you
                .body
                .iter()
                .map(|c| c.distance(&move_coord))
                .min()
                .unwrap_or(u32::MAX)
        });

        closest_move
    }

    /// Returns true if the snake is scared.
    pub fn is_scared(&self) -> bool {
        let other_snakes = self.board.other_snakes(&self.you);

        let closest_snake = other_snakes
            .iter()
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