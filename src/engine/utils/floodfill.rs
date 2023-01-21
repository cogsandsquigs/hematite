use crate::{engine::Engine, game::point::Point};
use std::collections::HashSet;

impl Engine {
    /// Floodfills the board from the given coordinates, and returns the number of spaces
    /// that were filled, the number of foods that were found, as well as all the points
    /// visited. This is the number of spaces that the snake can move into, accounting for
    /// growth.
    pub fn floodfill(&self, point: &Point) -> (u32, u32) {
        let mut filled = 0;
        let mut foods = 0;
        let mut queue = Vec::new();
        let mut visited = HashSet::new();

        // Add the starting coordinate to the queue
        queue.push(*point);

        // While there are still coordinates to check
        while let Some(point) = queue.pop() {
            // If the coordinate is already filled, or if it is part of a snake, skip it.
            if visited.contains(&point) || self.is_unsafe(&point) {
                continue;
            }

            // Add the coordinate to the list of filled coordinates
            visited.insert(point);

            // Add the coordinates of the adjacent spaces to the queue
            self.board
                .neighbors(&point)
                .into_iter()
                .for_each(|point| queue.push(point));

            // If the coordinate has food, skip it, as we don't want to count it
            if self.board.food.contains(&point) {
                foods += 1;
            }

            // Increment the number of filled spaces
            filled += 1;
        }

        (filled, foods)
    }
}
