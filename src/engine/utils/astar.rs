use super::heap_item::HeapItem;
use crate::{engine::Engine, game::point::Point};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

impl Engine {
    /// Given a set of starting coordinates and an ending coordinate, it finds the shortest
    /// path between the two coordinates, and returns the path as a vector of coordinates.
    /// If there is no path, it returns None.
    pub fn astar(&self, starts: Vec<Point>, end: Point) -> Option<Vec<Point>> {
        // The set of points that need to be explored/re-expanded. Initially, it contains the
        // starting points.
        let mut open_set: BinaryHeap<Reverse<HeapItem<Point>>> = starts
            .iter()
            .map(|p| Reverse(HeapItem::new(*p, 0)))
            .collect();

        // 'came_from' is a map of points to the node that they can most efficiently be reached from.
        let mut came_from: HashMap<Point, Point> = HashMap::new();

        // A map of points to their current cost from the start node. This cost is the same as the
        // cost used in Dijkstra's algorithm. It starts with the starting points having a cost of 0.
        let mut g_score: HashMap<Point, i32> = starts.iter().map(|p| (*p, 0)).collect();

        // A map of points to their `g_score` plus their heuristic cost. This is the cost that is
        // the best-estimate following through the most efficient path. Note that in this case,
        // it is the manhattan distance to the end. It contains the starting points having a cost
        // of their heuristic cost.
        let mut f_score: HashMap<Point, i32> = starts
            .iter()
            .map(|p| (*p, p.distance(&end) as i32))
            .collect();

        while let Some(Reverse(HeapItem { value: point, .. })) = open_set.pop() {
            // If we have reached the end, then we have found the shortest path.
            if point == end {
                // We have found the shortest path, so we can reconstruct the path and return it.
                let mut path = vec![end];
                let mut current = end;

                while let Some(next) = came_from.get(&current) {
                    // Instead of `push`ing the element to the list, we insert it at index 0 so that we don't
                    // have to reverse the list at the end.
                    path.insert(0, *next);
                    current = *next;
                }

                return Some(path);
            }

            self.board
                .neighbors(&point)
                .iter()
                // If the neighbor is unsafe, then we skip it. Originally, the cost function
                // labled unsafe nodes as having a cost of `i32::MAX`, but this caused a bug
                // where if the heuristic cost was more than 0, then the cost would roll over
                // and become negative, which would cause the node to be added to the open set.
                .filter(|neighbor| !self.is_unsafe(neighbor))
                // Now, the main A* algorithm.
                .for_each(|neighbor| {
                    // The g_score that we are currently considering is the g_score of the current
                    // point plus the cost of moving to the neighbor.
                    let tentative_g_score = g_score[&point] + self.cost(neighbor);

                    // If the node does exist, and the tentative g_score is better than the current
                    // g_score, then we update the came_from and g_score maps. If it doesn't exist,
                    // we do the same thing, and add it to the open set.
                    if tentative_g_score < *g_score.get(neighbor).unwrap_or(&i32::MAX) {
                        // This is the best path we have found so far, so we update the came_from
                        // and g_score maps.
                        came_from.insert(*neighbor, point);
                        g_score.insert(*neighbor, tentative_g_score);
                        f_score.insert(
                            *neighbor,
                            tentative_g_score + neighbor.distance(&end) as i32,
                        );

                        // If the neighbor is not in the open set, then we add it to the open set.
                        if !open_set.iter().any(|item| item.0.value == *neighbor) {
                            open_set.push(Reverse(HeapItem::new(*neighbor, f_score[neighbor])));
                        }
                    }
                });
        }

        // We didn't find a path, so return `None`
        None
    }
}
