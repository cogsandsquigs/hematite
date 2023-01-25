use crate::{engine::Engine, game::moves::Move};

/// Engine API for scared moves.
impl Engine {
    /// Returns the move the snake should make when it's scared.
    pub fn scared_move(&self) -> Option<Move> {
        let other_snakes = self.board.other_snakes(&self.you);

        let closest_snake = other_snakes
            .iter()
            .min_by_key(|snake| snake.head.distance(&self.you.head))?;

        // Move away from the closest snake.
        Move::all()
            .iter()
            .filter(|move_| self.is_safe(&move_.to_coord(&self.you.head)))
            .min_by_key(|move_| move_.to_coord(&self.you.head).distance(&closest_snake.head))
            .copied()
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
