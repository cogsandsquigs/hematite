// This is really just some code to run the A* algorithm on the given board.

use crate::{engine::Engine, game::point::Point};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

/// Runs the A* algorithm on the given map, starting at the given position and
/// ending at the given position.
impl Engine {
    /// Runs the A* algorithm on the given map, starting at the given positions and
    /// ending at the given position. Returns a path starting from any of the starting
    /// points to the end point, if one exists. Otherwise, returns None. If `ends` is
    /// empty, it panics.
    pub fn astar_find(&self, start: &Point, ends: &[Point]) -> Option<Vec<Point>> {
        if ends.is_empty() {
            panic!("'ends' should not be empty!");
        }

        // The queue of positions to check. Initialized with the starting positions.
        let mut search_queue: BinaryHeap<Reverse<WeightedPoint>> = BinaryHeap::new();
        // The best-case cost of moving to a point from the start.
        let mut g_score: HashMap<Point, u32> = HashMap::new();
        // The approximate cost of moving to the end from the start, using the
        // path from the start to the point.
        let mut f_score: HashMap<Point, u32> = HashMap::new();
        // The map from a point to the point it came from with the best path.
        let mut came_from: HashMap<Point, Point> = HashMap::new();

        // Initalize the search queue with the starting positions.
        search_queue.push(Reverse(WeightedPoint {
            point: *start,
            weight: 0,
        }));
        // Initialize the scores of the starting positions to their distance from the end.
        g_score.insert(*start, 0);
        f_score.insert(
            *start,
            start
                .closest_distance(ends)
                .expect("'ends' should not be empty!"),
        );

        while let Some(Reverse(WeightedPoint { point, .. })) = search_queue.pop() {
            // If we have found the end, return the path.
            if ends.contains(&point) {
                let mut path = vec![point];
                let mut current = point;

                // Iterate through the came_from map to get the path.
                while let Some(next) = came_from.get(&current) {
                    path.insert(0, *next);
                    current = *next;
                }

                return Some(path);
            }

            // Iterate through all the neighbors of the current point.
            for neighbor in self.safe_neighbors(&point) {
                // The tentative score is the current score of `point`, plus the point's own score, *plus*
                // the distance from the neighbor to the end.
                let tentative_g_score = g_score[&point] + self.g_score(&neighbor);

                // If the neighbor has not been encountered yet, or the tentative score is better than the
                // current score, update the score and the came_from map.
                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                    // Update the scores.
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(
                        neighbor,
                        tentative_g_score
                            + neighbor
                                .closest_distance(ends)
                                .expect("'ends' should not be empty!"),
                    );

                    // Insert into the came_from map, so that if we find the end, we can trace back the path.
                    came_from.insert(neighbor, point);

                    // Insert into the search queue.
                    search_queue.push(Reverse(WeightedPoint::new(neighbor, f_score[&neighbor])));
                }
            }
        }

        // If we haven't found a path by now, return None.
        None
    }

    /// Scoring heuristic for the A* algorithm: the score of a point is the cost of moving to that point
    /// (the h_score is the distance from the point to the end, which is calculated in the A* algorithm).
    fn g_score(&self, point: &Point) -> u32 {
        // If the point is a food, we would like to eat it. Usually, that would mean that the cost is negative,
        // however, there was a bug with that. If two food items appeared next to each other, the A* algorithm
        // would get stuck in an infinite loop, because it would keep switching between the two food items (and
        // giving them more and more negative scores). Thus, the score must be 0.
        if self.board.food.contains(point) {
            0
        }
        // If the point is a neighbor of a snake's head, we want to avoid it.
        else if self
            .board
            .other_snakes(&self.you)
            .any(|snake| snake.head.neighbors().contains(point))
        {
            3
        }
        // If the point is another snake's move, we want to never cross it.
        else if self.is_dangerous_snake_move(point) {
            999999999
        }
        // If the point is a hazard, we *really* want to avoid it, because it will kill us faster.
        // TODO: Differentiate between lethal and non-lethal hazards depending on game type.
        else if self.is_hazard(point) {
            16
        }
        // Otherwise, the point is safe, however every move decreases our health by 1.
        else {
            1
        }
    }
}

// The type for the heap of positions to check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct WeightedPoint {
    /// The point to check.
    pub point: Point,

    /// The weight of the point.
    pub weight: u32,
}

impl WeightedPoint {
    /// Create a new weighted point.
    fn new(point: Point, weight: u32) -> Self {
        Self { point, weight }
    }
}

impl PartialOrd for WeightedPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.weight.cmp(&other.weight))
    }
}

impl Ord for WeightedPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}
