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
    /// points to the end point, if one exists. Otherwise, returns None.
    pub fn astar_find(&self, starts: &[Point], end: &Point) -> Option<Vec<Point>> {
        // The queue of positions to check. Initialized with the starting positions.
        let mut search_queue: BinaryHeap<Reverse<WeightedPoint>> = BinaryHeap::new();
        // The score of every point we have encountered.
        let mut scores: HashMap<Point, i32> = HashMap::new();
        // The map from a point to the point it came from with the best path.
        let mut came_from: HashMap<Point, Point> = HashMap::new();

        println!("a");

        // Initalize the search queue with the starting positions.
        starts
            .iter()
            .map(|&point| Reverse(WeightedPoint::new(point, 0)))
            .for_each(|point| search_queue.push(point));

        // Initialize the scores of the starting positions to their distance from the end.
        starts
            .iter()
            .map(|&point| (point, point.distance(end) as i32))
            .for_each(|(point, score)| {
                scores.insert(point, score);
            });

        while let Some(Reverse(WeightedPoint {
            point,
            weight: point_score,
        })) = search_queue.pop()
        {
            println!("b");
            // If we have found the end, return the path.
            if &point == end {
                let mut path = vec![*end];
                let mut current = *end;

                // Iterate through the came_from map to get the path.
                while let Some(next) = came_from.get(&current) {
                    path.insert(0, *next);
                    current = *next;
                }

                return Some(path);
            }

            // Iterate through all the neighbors of the current point.
            for neighbor in self.safe_neighbors() {
                println!("c");
                // The tentative score is the current score of `point`, plus the point's own score, *plus*
                // the distance from the neighbor to the end.
                let tentative_score =
                    point_score + self.score(&neighbor) + point.distance(&neighbor) as i32;

                // If the neighbor has not been encountered yet, or the tentative score is better than the
                // current score, update the score and the came_from map.
                if !scores.contains_key(&neighbor) || tentative_score < scores[&neighbor] {
                    // Update the score.
                    scores.insert(neighbor, tentative_score);

                    // Insert into the search queue.
                    search_queue.push(Reverse(WeightedPoint::new(neighbor, tentative_score)));

                    // Insert into the came_from map, so that if we find the end, we can trace back the path.
                    came_from.insert(neighbor, point);
                }
            }
        }

        // If we haven't found a path by now, return None.
        None
    }

    /// Scoring for a point for the A* algorithm.
    fn score(&self, point: &Point) -> i32 {
        // If the point is a food, give it a score of -1.
        if self.board.food.contains(point) {
            -3
        } else {
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
    pub weight: i32,
}

impl WeightedPoint {
    /// Create a new weighted point.
    fn new(point: Point, weight: i32) -> Self {
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
