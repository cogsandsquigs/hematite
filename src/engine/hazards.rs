use super::Engine;
use crate::game::point::Point;

/// API for gettting all the long-term safe moves for the engine's snake.
impl Engine {
    /// Get all the long-term safe moves for the engine
    pub fn hazard_moves(&mut self) {
        let head = self.you.head;

        for (move_, _) in self.moves {
            let point = move_.to_coord(&head);
            let risk = self.is_hazardous(&point);

            self.moves.add(move_, risk);
        }
    }

    /// Checks if a point is a hazardous point. A hazardous point is a point that is
    /// possibly safe to move to, but has some risk associated with it. Unlike `is_unsafe`,
    /// this function returns a `f32` which represents the risk associated with the point.
    /// A point with a risk of `0.` is completely safe, and a point with a risk of `1.`
    /// is completely unsafe.
    pub fn is_hazardous(&self, point: &Point) -> f32 {
        let mut risk = 0.0;

        if self.is_potential_snake_move(point) {
            risk += 0.7;
        }

        if self.is_trapping(point) {
            risk += f32::INFINITY;
        }

        risk
    }

    /// Checks if a point will trap a snake. A point traps a snake if it leads to a space
    /// which has an area smaller than the snake's body plus any food inside the area.
    fn is_trapping(&self, point: &Point) -> bool {
        let (area, food) = self.floodfill(point);
        area < self.you.body.len() as u32 + food
    }

    /// Checks if a point could be a space a snake moves to. If the point could be
    /// a space a snake moves to, it is unsafe IF the snake is longer than you.
    pub fn is_potential_snake_move(&self, point: &Point) -> bool {
        self.board.snakes.iter().any(|snake| {
            // If the snake is not you,
            snake.id != self.you.id
                && snake // and the snake's head is a neighbor of the point,
                    .head
                    .neighbors()
                    .iter()
                    .any(|neighbor| neighbor == point)
                && snake.body.len() > self.you.body.len() // and if the snake is longer than you, it is unsafe.
        })
    }
}
