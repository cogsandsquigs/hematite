use crate::engine::Engine;
use itertools::Itertools;

/// API for using the scared `Mode`.
impl Engine {
    /// Get all the scared moves for the engine. This is the set of moves that gets it as close
    /// as possible to its own tail.
    pub fn scared(&mut self) {
        let head = self.you.head;
        let tail = self.you.tail();

        // Get the snake head that is closest to the head
        let min_snake_head = self
            .board
            .other_snakes(&self.you)
            .iter()
            .map(|snake| (snake.head, head.distance(&snake.head)))
            .min_by_key(|(_, distance)| *distance);

        // If a snake is close to the head, then avoid it
        if let Some((min_snake_head, distance)) = min_snake_head {
            if distance <= 4 {
                let moves = self
                    .moves
                    .into_iter()
                    .map(|(move_, _)| move_)
                    // Maximize (not minimize!) the distance to the closest snake head
                    .max_set_by_key(|move_| {
                        let point = move_.to_coord(&head);
                        point.distance(&min_snake_head)
                    });

                self.moves.invalidate_others_many(&moves);

                return;
            }
        }

        // If we haven't returned yet, then we are safe to get as close to the tail as possible
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
