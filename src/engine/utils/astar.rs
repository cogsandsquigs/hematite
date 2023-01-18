use super::heap_item::HeapItem;
use crate::{engine::Engine, game::point::Point};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

impl Engine {
    /// Given a set of starting coordinates and an ending coordinate, it finds the shortest
    /// path between the two coordinates, and returns the path as a vector of coordinates.
    /// If there is no path, it returns None.
    pub fn astar(&self, starts: Vec<Point>, end: Point) -> Vec<Point> {
        let mut queue: BinaryHeap<Reverse<HeapItem<Point>>> = BinaryHeap::new();
        let mut visited: HashSet<Point> = HashSet::new();

        // Add the starting coordinates to the queue.
        // Using `Reverse` because `BinaryHeap` is a max-heap, and we want a min-heap.
        starts
            .into_iter()
            .for_each(|point| queue.push(Reverse(HeapItem::new(point, 0))));

        // While there are still coordinates to check
        while let Some(Reverse(HeapItem { value: point, .. })) = queue.pop() {
            // If the coordinate is already filled, or if it is part of a snake, skip it.
            if visited.contains(&point) || self.is_unsafe(&point) {
                continue;
            }

            // If the coordinate is the end coordinate, return the path
            if point == end {
                todo!()
            }

            // Add the coordinate to the list of filled coordinates
            visited.insert(point);

            // Add the coordinates of the adjacent spaces to the queue
            self.board.neighbors(&point).into_iter().for_each(|point| {
                queue.push(Reverse(HeapItem::new(
                    point,
                    self.cost(&point) + end.distance(&point) as i32,
                )))
            });
        }

        todo!()
    }
}
