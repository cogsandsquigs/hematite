// This module deals with area-control-like algorithms and other things that have to do with areas

use crate::{engine::Engine, game::point::Point};
use itertools::Itertools;
use std::collections::HashSet;

impl Engine {
    /// Returns the area the engine controls as a u32.
    pub fn area_control(&self, head: &Point) -> u32 {
        // The other heads of the snakes
        let other_heads = self.other_snakes().map(|s| s.head).collect_vec();

        let mut area = 0;
        let mut queue = Vec::new();
        let mut visited = HashSet::new();

        queue.push(*head);

        while let Some(point) = queue.pop() {
            if visited.contains(&point) {
                continue;
            }

            visited.insert(point);

            let snake_distance = point.distance(head);
            let other_distance = point.closest_distance(&other_heads).unwrap_or(u32::MAX);

            if snake_distance < other_distance {
                area += 1;
            }

            for neighbor in point.neighbors() {
                if self.is_on_board(&neighbor) && !visited.contains(&neighbor) {
                    queue.push(neighbor);
                }
            }
        }

        area
    }

    /// Returns the area the engine can access as a u32.
    pub fn area_accessible(&self, head: &Point) -> u32 {
        let mut area = 0;
        let mut queue = Vec::new();
        let mut visited = HashSet::new();

        queue.push(*head);

        while let Some(point) = queue.pop() {
            if visited.contains(&point) {
                continue;
            }

            visited.insert(point);

            area += 1;

            for neighbor in point.neighbors() {
                if !self.is_unsafe(&point) && !visited.contains(&neighbor) {
                    queue.push(neighbor);
                }
            }
        }

        area
    }
}
